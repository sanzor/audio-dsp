
/// Instantiates the compiled wasm module with zero host imports (mirroring
/// `WebAssembly.instantiate(binary)` in the editor's graph-worklet.js exactly
/// — this also validates the module will actually link in the browser) and
/// calls its metadata export. Fuel-limited since this briefly executes
/// attacker-controlled wasm server-side.
pub fn introspect_metadata(wasm_bytes: &[u8], fuel_limit: u64) -> Result<TransformMetadataJson, String> {
    let mut config = Config::new();
    config.consume_fuel(true);
    let engine = Engine::new(&config).map_err(|e| e.to_string())?;
    let module = Module::new(&engine, wasm_bytes).map_err(|e| format!("invalid wasm module: {e}"))?;

    let linker: Linker<()> = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    store
        .set_fuel(fuel_limit)
        .map_err(|e| format!("failed to set fuel budget: {e}"))?;

    let instance = linker.instantiate(&mut store, &module).map_err(|e| {
        format!("wasm module failed to instantiate (it must declare zero imports, matching how the editor loads it): {e}")
    })?;

    let ptr_fn = instance
        .get_typed_func::<(), i32>(&mut store, "transform_metadata_ptr")
        .map_err(|_| {
            "missing export `transform_metadata_ptr` — did you call transform_sdk::export_transform!(...)?".to_string()
        })?;
    let len_fn = instance
        .get_typed_func::<(), i32>(&mut store, "transform_metadata_len")
        .map_err(|_| "missing export `transform_metadata_len`".to_string())?;
    let memory = instance
        .get_memory(&mut store, "memory")
        .ok_or_else(|| "wasm module does not export linear memory".to_string())?;

    // Feature-detection only — presence, not value, is what matters here.
    // Its absence means a module built against a pre-multi-input-ports SDK,
    // still speaking the old in-place single-buffer `process(ptr, len,
    // params_ptr, params_len)` signature (no return value). See
    // `agents/transforms.md`'s ABI contract section.
    let has_abi_version = instance.get_export(&mut store, "transform_abi_version").is_some();

    let ptr = ptr_fn
        .call(&mut store, ())
        .map_err(|e| format!("metadata call trapped: {e}"))? as usize;
    let len = len_fn
        .call(&mut store, ())
        .map_err(|e| format!("metadata call trapped: {e}"))? as usize;

    let mut buf = vec![0u8; len];
    memory
        .read(&store, ptr, &mut buf)
        .map_err(|e| format!("metadata read out of bounds: {e}"))?;

    let json = String::from_utf8(buf).map_err(|_| "metadata is not valid UTF-8".to_string())?;
    let metadata: TransformMetadataJson =
        serde_json::from_str(&json).map_err(|e| format!("metadata JSON is malformed: {e}"))?;

    validate_metadata(&metadata, has_abi_version)?;

    Ok(metadata)
}

/// Catches shape problems that would otherwise surface as an opaque Postgres
/// constraint violation later, plus ABI-load-bearing shape rules that only
/// make sense to enforce here (once introspected, before anything is ever
/// persisted). `transform_param` has UNIQUE(transform_id, name) and
/// UNIQUE(transform_id, param_order); `transform_port` has no such DB-level
/// constraint across directions (a transform legitimately has an input and
/// an output both named the same or both at order 0), but names must now be
/// unique *within* a direction, since `.port("name")`-style lookups exist.
fn validate_metadata(metadata: &TransformMetadataJson, has_abi_version: bool) -> Result<(), String> {
    if metadata.name.trim().is_empty() {
        return Err("metadata.name must not be empty".to_string());
    }

    let mut seen_input_names = HashSet::new();
    let mut seen_output_names = HashSet::new();
    let mut output_ports: Vec<&PortMetadataJson> = Vec::new();
    let mut program_input_count = 0usize;

    for port in &metadata.ports {
        if port.name.trim().is_empty() {
            return Err("a port has an empty name".to_string());
        }

        match port.direction {
            DirectionJson::Input => {
                if !seen_input_names.insert(port.name.as_str()) {
                    return Err(format!("duplicate input port name: {}", port.name));
                }
                if port.kind == PortKindJson::Program {
                    program_input_count += 1;
                }
            }
            DirectionJson::Output => {
                if !seen_output_names.insert(port.name.as_str()) {
                    return Err(format!("duplicate output port name: {}", port.name));
                }
                output_ports.push(port);
            }
        }
    }

    // Load-bearing since the ABI has exactly one dedicated output pointer —
    // previously unchecked because it didn't matter yet.
    if output_ports.len() != 1 {
        return Err(format!(
            "a transform must declare exactly one output port, found {}",
            output_ports.len()
        ));
    }
    let output = output_ports[0];
    if output.kind != PortKindJson::Program || output.cardinality != PortCardinalityJson::Single {
        return Err(format!(
            "output port '{}' must be kind=program, cardinality=single (found kind={:?}, cardinality={:?})",
            output.name, output.kind, output.cardinality
        ));
    }

    // A module with no `transform_abi_version` export speaks the old
    // in-place, single-buffer ABI — the old runtime has no way to route a
    // second signal, so such a module must declare exactly one Program
    // input port. In practice this branch is effectively unreachable for a
    // *fresh* compile (export_transform! always emits transform_abi_version
    // now), but it's a real, still-checkable consistency rule, not dead
    // code — see agents/transforms.md's ABI contract section.
    if !has_abi_version && program_input_count != 1 {
        return Err(format!(
            "module exports only the legacy `process` ABI (no transform_abi_version) — it must declare exactly one Program input port, found {program_input_count}"
        ));
    }

    let mut seen_param_names = HashSet::new();
    let mut seen_param_orders = HashSet::new();
    for param in &metadata.params {
        if param.name.trim().is_empty() {
            return Err("a param has an empty name".to_string());
        }
        if !seen_param_names.insert(param.name.as_str()) {
            return Err(format!("duplicate param name: {}", param.name));
        }
        if !seen_param_orders.insert(param.order) {
            return Err(format!("duplicate param order: {}", param.order));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn program_port(name: &str, direction: DirectionJson, order: i32) -> PortMetadataJson {
        PortMetadataJson {
            name: name.to_string(),
            direction,
            order,
            description: None,
            kind: PortKindJson::Program,
            cardinality: PortCardinalityJson::Single,
        }
    }

    fn base_metadata(ports: Vec<PortMetadataJson>) -> TransformMetadataJson {
        TransformMetadataJson {
            name: "Test".to_string(),
            description: None,
            ports,
            params: vec![],
        }
    }

    #[test]
    fn accepts_a_single_input_single_output_transform_with_abi_version() {
        let metadata = base_metadata(vec![
            program_port("in", DirectionJson::Input, 0),
            program_port("out", DirectionJson::Output, 0),
        ]);
        assert!(validate_metadata(&metadata, true).is_ok());
    }

    #[test]
    fn accepts_multi_input_when_abi_version_is_present() {
        let metadata = base_metadata(vec![
            program_port("a", DirectionJson::Input, 0),
            program_port("b", DirectionJson::Input, 1),
            program_port("out", DirectionJson::Output, 0),
        ]);
        assert!(validate_metadata(&metadata, true).is_ok());
    }

    #[test]
    fn rejects_multi_input_without_abi_version() {
        let metadata = base_metadata(vec![
            program_port("a", DirectionJson::Input, 0),
            program_port("b", DirectionJson::Input, 1),
            program_port("out", DirectionJson::Output, 0),
        ]);
        let err = validate_metadata(&metadata, false).unwrap_err();
        assert!(err.contains("legacy"), "unexpected error: {err}");
    }

    #[test]
    fn rejects_zero_output_ports() {
        let metadata = base_metadata(vec![program_port("in", DirectionJson::Input, 0)]);
        let err = validate_metadata(&metadata, true).unwrap_err();
        assert!(err.contains("exactly one output port"), "unexpected error: {err}");
    }

    #[test]
    fn rejects_two_output_ports() {
        let metadata = base_metadata(vec![
            program_port("in", DirectionJson::Input, 0),
            program_port("out1", DirectionJson::Output, 0),
            program_port("out2", DirectionJson::Output, 1),
        ]);
        let err = validate_metadata(&metadata, true).unwrap_err();
        assert!(err.contains("exactly one output port"), "unexpected error: {err}");
    }

    #[test]
    fn rejects_sidechain_output_port() {
        let mut output = program_port("out", DirectionJson::Output, 0);
        output.kind = PortKindJson::Sidechain;
        let metadata = base_metadata(vec![program_port("in", DirectionJson::Input, 0), output]);
        let err = validate_metadata(&metadata, true).unwrap_err();
        assert!(err.contains("kind=program"), "unexpected error: {err}");
    }

    #[test]
    fn rejects_many_cardinality_output_port() {
        let mut output = program_port("out", DirectionJson::Output, 0);
        output.cardinality = PortCardinalityJson::Many;
        let metadata = base_metadata(vec![program_port("in", DirectionJson::Input, 0), output]);
        let err = validate_metadata(&metadata, true).unwrap_err();
        assert!(err.contains("cardinality=single"), "unexpected error: {err}");
    }

    #[test]
    fn rejects_duplicate_port_names_within_a_direction() {
        let metadata = base_metadata(vec![
            program_port("in", DirectionJson::Input, 0),
            program_port("in", DirectionJson::Input, 1),
            program_port("out", DirectionJson::Output, 0),
        ]);
        let err = validate_metadata(&metadata, true).unwrap_err();
        assert!(err.contains("duplicate input port name"), "unexpected error: {err}");
    }

    #[test]
    fn allows_same_name_across_directions() {
        let metadata = base_metadata(vec![
            program_port("main", DirectionJson::Input, 0),
            program_port("main", DirectionJson::Output, 0),
        ]);
        assert!(validate_metadata(&metadata, true).is_ok());
    }

    #[test]
    fn accepts_sidechain_input_port_alongside_one_program_input() {
        let mut sidechain = program_port("key", DirectionJson::Input, 1);
        sidechain.kind = PortKindJson::Sidechain;
        let metadata = base_metadata(vec![
            program_port("program", DirectionJson::Input, 0),
            sidechain,
            program_port("out", DirectionJson::Output, 0),
        ]);
        assert!(validate_metadata(&metadata, true).is_ok());
    }
}

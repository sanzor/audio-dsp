use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::ticket_worker::processor::{transform_metadata::{DirectionJson, ParamMetadataJson, PortCardinalityJson, PortKindJson, PortMetadataJson}, wasm::wasm_parser::ParsedPrimitiveWasm};

/// A `PrimitiveMetadataJson` that has passed `validate_primitive_metadata_contract`.
/// Only reachable through validation, so a caller can't build a
/// `TransformMetadataJson` from metadata that skipped the contract check.
#[derive(Debug,Serialize,Deserialize)]
pub struct ValidatedPrimitiveMetadata {
    pub name: String,
    pub description: Option<String>,
    pub ports: Vec<PortMetadataJson>,
    pub params: Vec<ParamMetadataJson>,
}

/// Catches shape problems that would otherwise surface as an opaque Postgres
/// constraint violation later, plus ABI-load-bearing shape rules that only
/// make sense to enforce here (once introspected, before anything is ever
/// persisted). `transform_param` has UNIQUE(transform_id, name) and
/// UNIQUE(transform_id, param_order); `transform_port` has no such DB-level
/// constraint across directions (a transform legitimately has an input and
/// an output both named the same or both at order 0), but names must now be
/// unique *within* a direction, since `.port("name")`-style lookups exist.
pub(crate) fn validate_primitive(
    data: ParsedPrimitiveWasm,
) -> Result<ValidatedPrimitiveMetadata, String> {
    let ParsedPrimitiveWasm { metadata, has_abi_version } = data;
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

    Ok(ValidatedPrimitiveMetadata {
        name: metadata.name,
        description: metadata.description,
        ports: metadata.ports,
        params: metadata.params,
    })
}

#[cfg(test)]
#[path = "validator_tests.rs"]
mod tests;
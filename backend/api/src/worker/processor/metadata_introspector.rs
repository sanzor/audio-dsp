use std::collections::HashSet;

use serde::Deserialize;
use wasmtime::{Config, Engine, Linker, Module, Store};

/// Backend-side mirror of `transform_sdk::TransformMetadata`. Kept separate
/// from the SDK crate's serialize-side types since they compile for
/// different targets (native here, wasm32 there).
#[derive(Debug, Deserialize)]
pub struct TransformMetadataJson {
    pub name: String,
    pub description: Option<String>,
    pub ports: Vec<PortMetadataJson>,
    pub params: Vec<ParamMetadataJson>,
}

#[derive(Debug, Deserialize)]
pub struct PortMetadataJson {
    pub name: String,
    pub direction: DirectionJson,
    pub order: i32,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DirectionJson {
    Input,
    Output,
}

impl DirectionJson {
    pub fn as_db_str(&self) -> &'static str {
        match self {
            DirectionJson::Input => "input",
            DirectionJson::Output => "output",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ParamMetadataJson {
    pub name: String,
    pub order: i32,
    pub default: f32,
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub description: Option<String>,
}

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

    validate_metadata(&metadata)?;

    Ok(metadata)
}

/// Catches shape problems that would otherwise surface as an opaque Postgres
/// constraint violation later. `transform_params` has UNIQUE(transform_id, name)
/// and UNIQUE(transform_id, param_order); `transform_ports` has no such
/// constraints (a transform legitimately has an input and an output both at
/// order 0), so ports are only checked for a non-empty name.
fn validate_metadata(metadata: &TransformMetadataJson) -> Result<(), String> {
    if metadata.name.trim().is_empty() {
        return Err("metadata.name must not be empty".to_string());
    }

    for port in &metadata.ports {
        if port.name.trim().is_empty() {
            return Err("a port has an empty name".to_string());
        }
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

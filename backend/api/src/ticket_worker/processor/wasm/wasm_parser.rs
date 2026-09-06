use serde::Deserialize;
use wasmtime::{Config, Engine, Linker, Module, Store};

use crate::ticket_worker::processor::transform_metadata::{ParamMetadataJson, PortMetadataJson};

pub struct WasmInput<'a> {
    pub wasm_bytes: &'a [u8],
    pub fuel_limit: u64,
}

/// Metadata exported by a primitive transform's WASM module.
/// A graph belongs to a composite draft and must never appear here.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrimitiveMetadataJson {
    pub name: String,
    pub description: Option<String>,
    pub ports: Vec<PortMetadataJson>,
    pub params: Vec<ParamMetadataJson>,
}

/// Parsed primitive metadata plus the ABI capability detected from the module.
/// Metadata-contract validation is deliberately a separate caller concern.
#[derive(Debug)]
pub struct ParsedPrimitiveWasm {
    pub metadata: PrimitiveMetadataJson,
    pub has_abi_version: bool,
}
/// Instantiates primitive WASM with zero host imports, invokes its metadata
/// exports, and parses the returned JSON.
pub fn parse_wasm(input: WasmInput<'_>) -> Result<ParsedPrimitiveWasm, String> {
    let mut config = Config::new();
    config.consume_fuel(true);
    let engine = Engine::new(&config).map_err(|e| e.to_string())?;
    let module =
        Module::new(&engine, input.wasm_bytes).map_err(|e| format!("invalid wasm module: {e}"))?;

    let linker: Linker<()> = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    store
        .set_fuel(input.fuel_limit)
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

    let has_abi_version = instance
        .get_export(&mut store, "transform_abi_version")
        .is_some();

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

    let json = String::from_utf8(buf.to_owned()).map_err(|_| "metadata is not valid UTF-8".to_string())?;
    let metadata = serde_json::from_str(&json)
        .map_err(|e| format!("primitive metadata JSON is malformed: {e}"))?;

    Ok(ParsedPrimitiveWasm {
        metadata,
        has_abi_version,
    })
}


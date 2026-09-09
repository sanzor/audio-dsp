use crate::transform_drafts::data_provider::transform_drafts_data_provider::CompiledPrimitiveDraft;
use crate::transform_drafts::processor::{
    build_job::{check_draft_source_code, compile_transform_source},
    build_job_config::BuildJobConfig,
    validator::validator::validate_primitive,
    wasm::wasm_parser::{parse_wasm, WasmInput},
};

/// The single entry point for turning a primitive transform's source into a
/// compiled, validated artifact. Nothing outside this module should call
/// `build_job`/`wasm_parser`/`validator` directly — go through `Processor`.
#[derive(Clone)]
pub struct Processor {
    build_job_config: BuildJobConfig,
    metadata_fuel_limit: u64,
}

impl Processor {
    pub fn new(build_job_config: BuildJobConfig, metadata_fuel_limit: u64) -> Self {
        Self {
            build_job_config,
            metadata_fuel_limit,
        }
    }

    /// A fast `cargo check` — no wasm artifact produced or kept. Meant for
    /// quick editor feedback before Save, not a substitute for it.
    pub async fn check(&self, source_code: &str) -> Result<(), String> {
        check_draft_source_code(&self.build_job_config, source_code).await
    }

    /// Compiles `source_code` to wasm, introspects and validates its
    /// exported metadata, and packages the result — synchronously, on
    /// whatever hot path calls it (currently Save). Rejects as a whole on
    /// any step's failure; nothing is partially produced.
    pub async fn compile_primitive(
        &self,
        source_code: &str,
    ) -> Result<CompiledPrimitiveDraft, String> {
        let wasm_bytecode = compile_transform_source(&self.build_job_config, source_code).await?;

        let parsed = parse_wasm(WasmInput {
            fuel_limit: self.metadata_fuel_limit,
            wasm_bytes: &wasm_bytecode,
        })?;

        let validated = validate_primitive(parsed)?;

        let metadata = serde_json::to_string(&validated)
            .map_err(|e| format!("failed to serialize compiled metadata: {e}"))?;

        Ok(CompiledPrimitiveDraft {
            wasm_bytecode,
            name: validated.name,
            description: validated.description,
            metadata,
        })
    }
}

use serde::{Deserialize, Serialize};

pub type TransformDraftId=i64;
/// Bucket 2 — "save". Always has source_code (a plain draft, possibly
/// non-compiling). The binary/metadata fields are only present once a
/// compiled resource has been attached via a save call; a source-only save
/// leaves them as whatever they already were.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbTransformDraft {
    pub transform_id: TransformDraftId,
    pub source_code: String,
    pub wasm_bytecode: Option<Vec<u8>>,
    pub wasm_source_code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub kind: String,
    pub metadata:Vec<u32>
}

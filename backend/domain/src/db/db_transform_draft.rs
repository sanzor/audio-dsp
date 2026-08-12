use serde::{Deserialize, Serialize};

use crate::db::{
    db_transform::TransformId
};

/// Bucket 2 — "save". Always has source_code (a plain draft, possibly
/// non-compiling). The binary/metadata fields are only present once a
/// compiled resource has been attached via a save call; a source-only save
/// leaves them as whatever they already were.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbTransformDraft {
    pub transform_id: TransformId,
    pub source_code: String,
    pub wasm_bytecode: Option<Vec<u8>>,
    pub wasm_source_code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub kind: String
}

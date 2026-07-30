use serde::{Deserialize, Serialize};

use crate::db::{
    db_transform::TransformId,
    transform_snapshot::{CompositeGraphDefinition, ParamSnapshot, PortSnapshot},
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
    /// The exact source text `wasm_bytecode` was compiled from, snapshotted at
    /// attach time (same moment as wasm_bytecode/name/description/ports/params).
    /// `None` iff `wasm_bytecode` is `None`. Publish compares this against
    /// `source_code` to detect drift from a later source-only save — see
    /// `agents/decisions/0002-transform-draft-lifecycle-decisions.md`.
    pub wasm_source_code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    /// Present only for kind = "composite" drafts — the working copy of the
    /// wiring graph, mirroring how source_code is the primitive working copy.
    pub graph_definition: Option<CompositeGraphDefinition>,
    pub ports: Vec<PortSnapshot>,
    pub params: Vec<ParamSnapshot>,
}

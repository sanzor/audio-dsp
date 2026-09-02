use serde::{Deserialize, Serialize};

use crate::db::db_transform::TransformId;

/// The id of a transform's in-progress (bucket 2) draft. Distinct from
/// `TransformId` so a draft id and a published id can't be swapped by
/// accident at a call site — today a draft and its transform share the
/// same underlying row id (a draft row is created together with its
/// transform row, see `TransformsDataProvider::insert_transform_draft`),
/// so converting between them is always exact.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, sqlx::Type,
)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct TransformDraftId(pub i64);

impl std::fmt::Display for TransformDraftId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for TransformDraftId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<TransformDraftId> for i64 {
    fn from(value: TransformDraftId) -> Self {
        value.0
    }
}

/// See `TransformId`'s `From<TransformDraftId>` impl — the reverse
/// direction of the same identity.
impl From<TransformId> for TransformDraftId {
    fn from(value: TransformId) -> Self {
        Self(value.0)
    }
}

impl utoipa::PartialSchema for TransformDraftId {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        i64::schema()
    }
}

impl utoipa::ToSchema for TransformDraftId {}

/// Bucket 2 — "save". A primitive draft has `source_code` (possibly
/// non-compiling); a composite draft instead has its wiring graph nested in
/// `metadata` and no `source_code` at all. `wasm_bytecode`/`wasm_source_code`
/// are only present once a compiled resource has been attached via a save
/// call (primitive only); a source-only save leaves them as whatever they
/// already were.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbTransformDraft {
    pub transform_id: TransformDraftId,
    pub source_code: Option<String>,
    pub wasm_bytecode: Option<Vec<u8>>,
    pub wasm_source_code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub kind: String,
    /// JSON: `{name, description, ports, params}` for a primitive; a
    /// composite additionally carries `graph` — its authored wiring graph.
    pub metadata: Option<String>,
}

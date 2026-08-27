use domain::db::db_transform_draft::{DbTransformDraft, TransformDraftId};
use serde::Serialize;
use utoipa::ToSchema;

use crate::ticket_worker::processor::transform_metadata::PortMetadataJson;

/// A transform's in-progress (bucket 2) draft state.
#[derive(Debug, Serialize, ToSchema)]
pub struct TransformDraftDto {
    pub transform_id: TransformDraftId,
    pub source_code: Option<String>,
    pub metadata_json: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub kind: String,
    /// Whether a compiled binary is currently attached (via a prior save
    /// with a `resource_id`) — not whether it's still in sync with
    /// `source_code`; a source-only save can leave a stale binary attached.
    pub has_binary: bool,
}

impl From<DbTransformDraft> for TransformDraftDto {
    fn from(value: DbTransformDraft) -> Self {
        Self {
            transform_id: value.transform_id,
            source_code: value.source_code,
            metadata_json: value.metadata,
            has_binary: value.wasm_bytecode.is_some(),
            name: value.name,
            description: value.description,
            kind: value.kind,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransformDraftsResponse {
    pub drafts: Vec<TransformDraftDto>,
}

/// A composite's derived externally-visible ports on a successful
/// `validate-graph` call.
#[derive(Debug, Serialize)]
pub struct ValidateGraphResponse {
    pub ports: Vec<PortMetadataJson>,
}

use domain::db::db_transform_draft::TransformDraftId;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct TransformDraftIdPath {
    pub transform_id: TransformDraftId,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TransformDraftIdsRequest {
    pub ids: Vec<TransformDraftId>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateTransformParams {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    /// "primitive" | "composite".
    pub kind: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CheckSourceParams {
    /// Not necessarily what's saved — the caller can check live in-progress
    /// edits before saving.
    pub source_code: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SavePrimitiveParams {
    pub source_code: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SaveCompositeParams {
    /// The wiring graph JSON (`{nodes, edges}`) — same shape
    /// `validate-graph` takes. Overwrites whatever was saved before.
    pub graph_json: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ValidateGraphParams {
    /// The composite's wiring graph JSON to validate — not necessarily
    /// what's currently saved; the caller can send live in-progress edits.
    pub graph_json: String,
}

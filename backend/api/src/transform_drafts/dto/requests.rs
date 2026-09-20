use domain::db::db_transform_draft::TransformDraftId;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct TransformDraftIdPath {
    pub draft_transform_id: TransformDraftId,
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
pub struct CheckSourceCodeParams {
    /// Not necessarily what's saved — callers may check live edits first.
    pub source_code: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SavePrimitiveParams {
    /// Save compiles this synchronously; the whole save is rejected if it
    /// doesn't compile into a valid transform.
    pub source_code: String,
    /// A draft's name/description are optional fields on Save (see
    /// `agents/decisions/0012-draft-name-description-editable.md`), folded
    /// in rather than living behind a separate endpoint — Save was already
    /// the only place a name/description edit needed to be flushed. `None`
    /// (including an entirely absent key — a plain `Option<T>` field
    /// deserializes that as `None` on its own; `#[serde(default)]` below is
    /// kept for explicitness, not because it's required for this case)
    /// means "this call isn't touching the name" and leaves the existing
    /// value untouched; it is never treated as "clear it". `Some(x)` sets it
    /// to `x`, taking priority over both the existing row value and any
    /// compiled-from-source fallback.
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SaveCompositeParams {
    /// The wiring graph (`{nodes, edges}`). Save intentionally persists this
    /// structurally without validating it; validation is a separate action.
    pub graph_definition: serde_json::Value,
    /// Same optional name/description treatment as `SavePrimitiveParams` —
    /// see the doc comment there.
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// The one Bucket-2 save payload. The draft's persisted `kind` decides
/// which variant is accepted, so callers never need to send a redundant
/// kind or transform id in the body.
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(untagged)]
pub enum SaveDraftParams {
    Primitive(SavePrimitiveParams),
    Composite(SaveCompositeParams),
}

/// The one Bucket-3 publish payload. Unlike `SaveDraftParams`, both
/// variants carry no data — the caller just declares which kind it thinks
/// it's publishing, and the service rejects the request (400) if that
/// doesn't match the draft's actual persisted `kind`. Tagged (not
/// untagged, unlike `SaveDraftParams`) because fieldless variants have no
/// structural shape of their own to disambiguate on.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum PublishDraftParams {
    Primitive,
    Composite,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ValidateGraphParams {
    /// The graph to validate, which may include unsaved canvas edits.
    pub graph_json: String,
}

#[cfg(test)]
#[path = "requests_tests.rs"]
mod requests_tests;

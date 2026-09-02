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
    /// Not necessarily what's saved — callers may check live edits first.
    pub source_code: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SavePrimitiveParams {
    pub source_code: String,
    /// Optional compiled WASM held temporarily by the Creator frontend.
    /// Omit for a source-only save; then any older saved build remains in
    /// place and Publish's source snapshot check will mark it stale.
    pub wasm_base64: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SaveCompositeParams {
    /// The wiring graph (`{nodes, edges}`). Save intentionally persists this
    /// structurally without validating it; validation is a separate action.
    pub graph_definition: serde_json::Value,
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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ValidateGraphParams {
    /// The graph to validate, which may include unsaved canvas edits.
    pub graph_json: String,
}

#[cfg(test)]
mod tests {
    use super::{SaveDraftParams, SavePrimitiveParams};

    #[test]
    fn save_primitive_payload_accepts_optional_base64_wasm() {
        let payload: SaveDraftParams = serde_json::from_str(
            r#"{"source_code":"impl Transform for Gain {}","wasm_base64":"AGFzbQ=="}"#,
        )
        .expect("primitive save payload should deserialize");

        match payload {
            SaveDraftParams::Primitive(SavePrimitiveParams {
                source_code,
                wasm_base64,
            }) => {
                assert_eq!(source_code, "impl Transform for Gain {}");
                assert_eq!(wasm_base64.as_deref(), Some("AGFzbQ=="));
            }
            SaveDraftParams::Composite(_) => panic!("expected primitive payload"),
        }
    }

    #[test]
    fn save_composite_payload_accepts_graph_definition() {
        let payload: SaveDraftParams =
            serde_json::from_str(r#"{"graph_definition":{"nodes":[],"edges":[]}}"#)
                .expect("composite save payload should deserialize");

        assert!(matches!(payload, SaveDraftParams::Composite(_)));
    }
}

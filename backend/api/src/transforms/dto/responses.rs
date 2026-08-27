use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use domain::db::db_transform::{DbTransform, TransformId};
use domain::domain_user::UserId;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize)]
pub struct TransformSummaryListResponse {
    pub transforms: Vec<TransformSummaryDto>,
    pub total: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransformSummaryDto {
    pub transform_id: TransformId,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    /// "primitive" | "composite".
    pub kind: String,
}

impl From<DbTransform> for TransformSummaryDto {
    fn from(value: DbTransform) -> Self {
        Self {
            transform_id: value.transform_id,
            name: value.name,
            description: value.description,
            icon: value.icon,
            kind: value.kind,
        }
    }
}

/// A published (bucket 3) transform's definition. `wasm_bytecode` isn't
/// included here — fetch it via the dedicated binary endpoints below, which
/// base64-encode it for transport.
#[derive(Debug, Serialize, ToSchema)]
pub struct TransformDto {
    pub transform_id: TransformId,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    /// "primitive" | "composite".
    pub kind: String,
    /// A primitive's compiled Rust source. `None` for a composite — see
    /// `metadata_json`.
    pub source_code: Option<String>,
    /// JSON: `{name, description, ports, params}`, plus a `graph` field
    /// (the authored wiring graph) for a composite.
    pub metadata_json: Option<String>,
    pub owner_user_id: UserId,
    /// RFC 3339 / ISO 8601.
    pub created_at: String,
}

impl From<DbTransform> for TransformDto {
    fn from(value: DbTransform) -> Self {
        Self {
            transform_id: value.transform_id,
            name: value.name,
            description: value.description,
            icon: value.icon,
            kind: value.kind,
            source_code: value.source_code,
            metadata_json: value.metadata,
            owner_user_id: value.owner_user_id,
            created_at: value.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransformsResponse {
    pub transforms: Vec<TransformDto>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransformBinaryDto {
    pub transform_id: TransformId,
    pub wasm_base64: String,
}

impl From<DbTransform> for TransformBinaryDto {
    fn from(value: DbTransform) -> Self {
        Self {
            transform_id: value.transform_id,
            wasm_base64: value.wasm_bytecode.map(|b| BASE64_STANDARD.encode(b)).unwrap_or_default(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransformBinariesResponse {
    pub binaries: Vec<TransformBinaryDto>,
}

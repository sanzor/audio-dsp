use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::db_transform_draft::TransformDraftId;
use crate::domain_user::UserId;

/// The id of a *published* (bucket 3) transform. Distinct from
/// `TransformDraftId` so a draft id and a published id can't be swapped by
/// accident at a call site — even though, today, a draft and its published
/// transform share the same underlying row id (see `TransformDraftId`'s
/// doc comment), so converting between them is always exact.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, sqlx::Type,
)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct TransformId(pub i64);

impl std::fmt::Display for TransformId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for TransformId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<TransformId> for i64 {
    fn from(value: TransformId) -> Self {
        value.0
    }
}

/// A draft id and its published transform id are, today, the same
/// underlying row id — see `TransformDraftId`'s doc comment.
impl From<TransformDraftId> for TransformId {
    fn from(value: TransformDraftId) -> Self {
        Self(value.0)
    }
}

impl utoipa::PartialSchema for TransformId {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        i64::schema()
    }
}

impl utoipa::ToSchema for TransformId {}

pub type TransformPortId = i64;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbTransform {
    pub transform_id: TransformId,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    /// "primitive" | "composite".
    pub kind: String,
    /// A primitive's compiled Rust source. `None` for a composite, which has
    /// no source of its own — see `metadata`.
    pub source_code: Option<String>,
    /// The runnable artifact: real wasm bytes for a primitive, or (once a
    /// composite's `resolve` step exists) the serialized flat primitive-only
    /// plan for a composite. `None` if nothing has been published yet.
    pub wasm_bytecode: Option<Vec<u8>>,
    /// JSON: `{name, description, ports, params}` for a primitive; a
    /// composite additionally carries `graph` — its authored wiring graph,
    /// alongside the same derived `ports`/`params` any consumer reads.
    pub metadata: Option<String>,
    pub owner_user_id: UserId,
    pub created_at: DateTime<Utc>,
}

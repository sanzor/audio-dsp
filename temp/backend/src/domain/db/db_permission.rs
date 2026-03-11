use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

pub type PermissionId = i64;
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbPermission {
    pub id: PermissionId,
    pub key: String,
    pub description: Option<String>,
}

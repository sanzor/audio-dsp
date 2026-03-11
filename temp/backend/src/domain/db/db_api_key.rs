use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;

pub type ApiKeyId = i64;
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbApiKey {
    pub id: ApiKeyId,
    pub org_id: OrganizationId,
    pub key: String,
    pub label: Option<String>,
    pub is_active: bool,
    pub created_by: Option<i64>,
}

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

pub type UsageId = i32;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbUsage {
    pub id: UsageId,
    pub user_id: domain::domain_user::UserId,
    pub project_count: i64,
    pub total_track_count: i64,
    pub total_storage_bytes: i64,
    pub updated_at: String,
}

use crate::domain::tier::Tier;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

pub type TierConfigId = i32;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
pub struct DbTierConfig {
    pub id: TierConfigId,
    pub tier: Tier,
    pub max_projects: i32,
    pub max_tracks_per_project: i32,
    pub max_storage_bytes: i64,
    pub updated_at: String,
}

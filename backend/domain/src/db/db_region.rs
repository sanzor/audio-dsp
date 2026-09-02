use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::db_region_set::RegionSetId;

pub type RegionId = i32;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbRegion {
    pub region_id: RegionId,
    pub region_set_id: RegionSetId,
    pub name: String,
    pub start_time_seconds: f32,
    pub end_time_seconds: f32,
    pub created_at: DateTime<Utc>,
}

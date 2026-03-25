use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::db_track::TrackId;

pub type RegionSetId = i32;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbRegionSet {
    pub region_set_id: RegionSetId,
    pub track_id: TrackId,
    pub name: String,
    pub track_length_seconds: f32,
    pub created_at: DateTime<Utc>,
}


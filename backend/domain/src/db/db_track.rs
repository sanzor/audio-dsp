use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type TrackId = i32;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbTrack {
    pub track_id: TrackId,
    pub name: String,
    pub extension: String,
    pub length_seconds: f32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DbTrackStorage {
    pub track_id: TrackId,
    pub storage_type: String,
    pub data: Vec<u8>,
}

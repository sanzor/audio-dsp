use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type SourceId = i32;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbSource {
    pub source_id: SourceId,
    pub name: String,
    pub extension: String,
    pub length_seconds: f32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbSourceMeta {
    pub source_id: SourceId,
    pub name: String,
    pub extension: String,
    pub length_seconds: f32,
    pub created_at: DateTime<Utc>,
}

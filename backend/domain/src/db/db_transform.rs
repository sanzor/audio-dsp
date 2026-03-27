use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type TransformId = i64;
pub type TransformPortId = i64;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbTransform {
    pub transform_id: TransformId,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbTransformPort {
    pub port_id: TransformPortId,
    pub transform_id: TransformId,
    pub name: String,
    pub direction: String, // "input" | "output"
    pub port_order: i32,
    pub description: Option<String>,
}

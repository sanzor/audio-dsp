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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbTransformDefinition {
    pub transform_id: TransformId,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub source_code: Option<String>,
    pub ports: Vec<DbTransformPort>,
    pub params: Vec<DbTransformParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbTransformBinary {
    pub transform_id: TransformId,
    pub wasm_bytecode: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbTransformPort {
    pub port_id: TransformPortId,
    pub transform_id: TransformId,
    pub name: String,
    pub direction: String, // "input" | "output"
    pub port_order: i32,
    pub description: Option<String>,
    pub kind: String,        // "program" | "sidechain"
    pub cardinality: String, // "single" | "many"
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbTransformParam {
    pub param_id: i64,
    pub transform_id: TransformId,
    pub name: String,
    pub param_order: i32,
    pub default_value: f32,
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
    pub description: Option<String>,
}

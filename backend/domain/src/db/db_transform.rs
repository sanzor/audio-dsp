use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain_user::UserId;

pub type TransformId = i64;
pub type TransformPortId = i64;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbTransform {
    pub transform_id: TransformId,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    /// "primitive" | "composite".
    pub kind: String,
    pub source_code:String,
    pub wasm_bytecode:Vec<u8>,
    pub metadata:Vec<u32>,
    pub owner_user_id: UserId,
    pub created_at: DateTime<Utc>,
}


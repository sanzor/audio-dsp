use serde::{Deserialize, Serialize};
use crate::db::db_transform::TransformPortId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformPort {
    pub port_id: TransformPortId,
    pub name: String,
    pub direction: String, // "input" | "output"
    pub port_order: i32,
    pub description: Option<String>,
}

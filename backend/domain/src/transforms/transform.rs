use serde::{Deserialize, Serialize};
use crate::db::db_transform::TransformId;
use super::transform_port::TransformPort;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub transform_id: TransformId,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub ports: Vec<TransformPort>,
}

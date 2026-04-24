use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::db::{GraphId, NodeId, TransformId};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodePosition {
    #[serde(default)]
    pub x: i32,
    #[serde(default)]
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: NodeId,
    #[serde(default)]
    pub graph_id: Option<GraphId>,
    #[serde(default)]
    pub transform_id: Option<TransformId>,
    #[serde(default)]
    pub position: NodePosition,
    #[serde(default)]
    pub params: HashMap<String, f32>,
}

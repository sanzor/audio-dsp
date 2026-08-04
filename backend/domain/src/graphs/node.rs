use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::db::{GraphId, NodeId, TransformId};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodePosition {
    // f32, not i32 -- ReactFlow's screenToFlowPosition/drag deltas are
    // always continuous floats, never integers. Nothing currently strictly
    // deserializes stored graph JSON into this struct (the save-state
    // endpoint stores an opaque serde_json::Value), so an i32 here hasn't
    // caused a live 400 the way it did for the Creator's identical
    // CompositeNodePosition -- but the same failure mode is latent the
    // moment anything does.
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
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

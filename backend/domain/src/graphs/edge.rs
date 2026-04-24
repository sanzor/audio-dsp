use serde::{Deserialize, Serialize};

use crate::db::{EdgeId, GraphId, NodeId, TransformPortId};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub id: EdgeId,
    #[serde(default)]
    pub graph_id: Option<GraphId>,
    #[serde(default)]
    pub from_node_id: NodeId,
    #[serde(default)]
    pub to_node_id: NodeId,
    #[serde(default)]
    pub from_port_id: Option<TransformPortId>,
    #[serde(default)]
    pub to_port_id: Option<TransformPortId>,
    #[serde(default)]
    pub to: Option<String>,
}

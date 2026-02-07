use serde::{Deserialize, Serialize};

use crate::db::GraphId;
use crate::graphs::{edge::Edge, node::Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub id: GraphId,
    pub name: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

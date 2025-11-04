use serde::{Deserialize, Serialize};

use crate::graphs::{edge::Edge, node::Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub id: String,
    pub name: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

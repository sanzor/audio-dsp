use serde::{Deserialize, Serialize};

use crate::db::{GraphId, RegionId};
use crate::graphs::{edge::Edge, node::Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSubtree {
    pub graph_id: GraphId,
    pub region_id: Option<RegionId>,
    pub name: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

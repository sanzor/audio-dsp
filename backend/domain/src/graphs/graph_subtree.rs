use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::db::{GraphId, RegionId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSubtree {
    pub graph_id: GraphId,
    pub region_id: Option<RegionId>,
    pub name: String,
    #[serde(default, alias = "graph_state")]
    pub repr: Value,
}

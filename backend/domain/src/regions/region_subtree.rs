use serde::{Deserialize, Serialize};

use crate::db::{RegionId, RegionSetId};
use crate::graphs::graph_subtree::GraphSubtree;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionSubtree {
    pub region_id: RegionId,
    pub region_set_id: RegionSetId,
    pub name: String,
    pub start_time: f32,
    pub end_time: f32,
    pub graph: Option<GraphSubtree>,
}

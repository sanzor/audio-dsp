use serde::{Deserialize, Serialize};

use crate::db::{RegionId, RegionSetId};
use crate::graphs::graph::Graph;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub region_set_id: RegionSetId,
    pub region_id: RegionId,
    pub name: String,
    pub start_time: f32,
    pub end_time: f32,
    pub graph: Option<Graph>,
}

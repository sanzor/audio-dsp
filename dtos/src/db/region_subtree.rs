use serde::Serialize;

use crate::db::graph_subtree::GraphSubtree;

#[derive(Clone, Debug, Serialize)]
pub struct RegionSubtree {
    pub id: String,
    pub region_set_id: String,
    pub name: String,
    pub start: f64,
    pub end: f64,
    pub graph_id: Option<GraphSubtree>,
}

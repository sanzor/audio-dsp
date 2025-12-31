use serde::Serialize;

use crate::db::region_subtree::RegionSubtree;

#[derive(Clone, Debug, Serialize)]
pub struct RegionSetSubtree {
    pub id: String,
    pub track_id: String,
    pub track_length: f32,
    pub name: String,
    pub regions: Vec<RegionSubtree>,
}

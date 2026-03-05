use serde::{Deserialize, Serialize};

use crate::db::TrackId;
use crate::region_set::region_set_subtree::RegionSetSubtree;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackSubtree {
    pub track_id: TrackId,
    pub name: String,
    pub extension: String,
    pub length_seconds: f32,
    pub region_sets: Vec<RegionSetSubtree>,
}

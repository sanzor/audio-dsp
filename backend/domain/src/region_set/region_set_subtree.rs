use serde::{Deserialize, Serialize};

use crate::{db::{RegionSetId, TrackId}, regions::region_subtree::RegionSubtree};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionSetSubtree {
    pub track_id: TrackId,
    pub track_length: f32,
    pub region_set_id: RegionSetId,
    pub name: String,
    pub regions: Vec<RegionSubtree>,
}
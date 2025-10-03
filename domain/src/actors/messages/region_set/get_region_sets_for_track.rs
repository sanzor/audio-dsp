use serde::Serialize;

use crate::regions::region_set::RegionSet;

pub struct GetRegionSetsForTrack {
    pub track_id: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct GetRegionSetsForTrackResult {
    pub track_id: String,
    pub region_sets: Vec<RegionSet>,
}

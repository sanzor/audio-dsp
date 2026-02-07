use serde::Serialize;

use crate::db::TrackId;
use crate::regions::region_set::RegionSet;

pub struct GetRegionSetsForTrack {
    pub track_id: TrackId,
}

#[derive(Serialize, Debug, Clone)]
pub struct GetRegionSetsForTrackResult {
    pub track_id: TrackId,
    pub region_sets: Vec<RegionSet>,
}

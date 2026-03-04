use crate::db::{DbRegionSet, TrackId};

pub struct GetRegionSetsForTrack {
    pub track_id: TrackId,
}

pub struct GetRegionSetsForTrackResult {
    pub track_id: TrackId,
    pub region_sets: Vec<DbRegionSet>,
}

use crate::{
    db::{RegionId, RegionSetId, TrackId},
    regions::region_subtree::RegionSubtree,
};

pub struct CopyRegionParams {
    pub source_region_id: RegionId,
    pub source_region_set_id: RegionSetId,
    pub source_track_id: TrackId,
    pub destination_region_set_id: RegionSetId,
    pub destination_track_id: TrackId,
    pub region_copy_name: String,
}

pub struct CopyRegionResult {
    pub region: RegionSubtree,
}

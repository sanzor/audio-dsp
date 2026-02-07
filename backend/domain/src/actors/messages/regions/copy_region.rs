use crate::db::{RegionId, RegionSetId, TrackId};
use crate::regions::region_set::RegionSet;

pub struct CopyRegion {
    pub copy_name: String,
    pub source_track_id: TrackId,
    pub source_region_set_id: RegionSetId,
    pub source_region_id: RegionId,
    pub destination_track_id: TrackId,
    pub destination_region_set_id: RegionSetId,
}

pub struct CopyRegionResult {
    pub region_set: RegionSet,
}

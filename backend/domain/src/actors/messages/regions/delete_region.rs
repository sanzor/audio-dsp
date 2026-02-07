use crate::db::{RegionId, RegionSetId};

pub struct DeleteRegion {
    pub region_id: RegionId,
    pub region_set_id: RegionSetId,
}

pub struct DeleteRegionResult {}

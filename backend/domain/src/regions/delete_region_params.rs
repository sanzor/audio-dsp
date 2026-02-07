use crate::db::{RegionId, RegionSetId};

pub struct DeleteRegionParams {
    pub region_set_id: RegionSetId,
    pub region_id: RegionId,
}

use crate::db::{RegionId, RegionSetId};

pub struct AddRegionParams {
    pub region_set_id: RegionSetId,
    pub start_time: f32,
    pub end_time: f32,
    pub name: String,
}

pub struct AddRegionResult {
    pub region_set_id: RegionSetId,
    pub region_id: RegionId,
    pub name: String,
}

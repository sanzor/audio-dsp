use crate::db::{RegionId, RegionSetId};
use crate::regions::region_set::RegionSet;

pub struct EditRegion {
    pub region_id: RegionId,
    pub region_set_id: RegionSetId,
    pub name: Option<String>,
    pub start_time: Option<f32>,
    pub end_time: Option<f32>,
}

pub struct EditRegionResult {
    pub region_set: RegionSet,
}

use crate::db::{RegionId, RegionSetId};

pub struct EditRegionParams {
    pub region_set_id: RegionSetId,
    pub region_id: RegionId,
    pub name: Option<String>,
    pub start_time: Option<f32>,
    pub end_time: Option<f32>,
}

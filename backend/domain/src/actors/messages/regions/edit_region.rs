use crate::regions::region_set::RegionSet;

pub struct EditRegion {
    pub region_id: String,
    pub region_set_id: String,
    pub name: Option<String>,
    pub start_time: Option<f32>,
    pub end_time: Option<f32>,
}

pub struct EditRegionResult {
    pub region_set: RegionSet,
}

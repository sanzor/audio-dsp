use crate::regions::region_set::RegionSet;

pub struct EditRegionSet {
    pub region_set_id: String,
    pub track_id: String,
    pub name: Option<String>,
}

pub struct EditRegionSetResult {
    pub region_set: RegionSet,
}

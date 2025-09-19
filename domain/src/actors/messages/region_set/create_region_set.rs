use crate::regions::region_set::RegionSet;

pub struct CreateRegionSet {
    pub track_id: String,
    pub name: Option<String>,
}

pub struct CreateRegionSetResult {
    pub region_set: RegionSet,
}

use crate::regions::region_set::RegionSet;

pub struct CopyRegion {
    pub copy_name: String,
    pub region_set_id: String,
    pub region_id: String,
}

pub struct CopyRegionResult {
    pub region_set: RegionSet,
}

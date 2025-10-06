use crate::regions::region_set::RegionSet;

pub struct CopyRegionParams {
    pub region_set_id: String,
    pub region_id: String,
    pub copy_name: String,
}

pub struct CopyRegionResult {
    pub set: RegionSet,
}

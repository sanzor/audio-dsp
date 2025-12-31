use crate::regions::region_set::RegionSet;

pub struct CopyRegionParams {
    pub source_region_id: String,
    pub destination_region_set_id: String,
    pub region_copy_name: String,
}

pub struct CopyRegionResult {
    pub set: RegionSet,
}

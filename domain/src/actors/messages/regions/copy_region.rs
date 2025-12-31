use crate::regions::region_set::RegionSet;

pub struct CopyRegion {
    pub copy_name: String,
    pub source_region_id: String,
    pub destination_region_set_id: String,
}

pub struct CopyRegionResult {
    pub region_set: RegionSet,
}

use serde::Serialize;

use crate::regions::region_set::RegionSet;

pub struct CopyRegionSet {
    pub track_id: String,
    pub region_set_id: String,
    pub region_set_copy_name: String,
}
#[derive(Serialize)]
pub struct CopyRegionSetResult {
    pub region_set:RegionSet
}

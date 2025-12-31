use dtos::db::region_set_subtree::RegionSetSubtree;
use serde::Serialize;

pub struct CopyRegionSet {
    pub region_set_id: String,
    pub region_set_copy_name: String,
}
#[derive(Serialize)]
pub struct CopyRegionSetResult {
    pub region_set: RegionSetSubtree,
}

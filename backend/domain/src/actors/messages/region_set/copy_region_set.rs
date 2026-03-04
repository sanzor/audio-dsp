use crate::db::RegionSetId;
use crate::region_set::region_set_subtree::RegionSetSubtree;

pub struct CopyRegionSet {
    pub region_set_id: RegionSetId,
    pub region_set_copy_name: String,
}

pub struct CopyRegionSetResult {
    pub region_set_subtree: RegionSetSubtree,
}

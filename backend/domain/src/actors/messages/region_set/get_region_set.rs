use crate::db::{DbRegionSet, RegionSetId};

pub struct GetRegionSet {
    pub region_set_id: RegionSetId,
}

pub struct GetRegionSetResult {
    pub region_set: DbRegionSet,
}

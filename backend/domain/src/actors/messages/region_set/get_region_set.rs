use serde::Serialize;

use crate::db::RegionSetId;
use crate::regions::region_set::RegionSet;

pub struct GetRegionSet {
    pub region_set_id: RegionSetId,
}
#[derive(Serialize)]
pub struct GetRegionSetResult {
    pub region_set: RegionSet,
}

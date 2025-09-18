use serde::Serialize;

use crate::regions::{region::Region, region_set::RegionSet};

pub struct GetRegionSet{
    pub region_set_id:String
}
#[derive(Serialize)]
pub struct GetRegionSetResult{
    pub region_set:RegionSet
}
use dtos::db::region_set_db_dto::RegionSetDbDto;
use serde::Serialize;

use crate::regions::region_set::RegionSet;

pub struct GetRegionSet {
    pub region_set_id: String,
}
#[derive(Serialize)]
pub struct GetRegionSetResult {
    pub region_set: RegionSetDbDto,
}

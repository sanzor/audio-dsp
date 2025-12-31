use std::collections::HashMap;

use dtos::db::region_set_db_dto::RegionSetDbDto;
use serde::Serialize;

pub struct GetRegionSets {}

#[derive(Serialize)]
pub struct GetRegionSetsResult {
    pub track_region_sets_map: HashMap<String, Vec<RegionSetDbDto>>,
}

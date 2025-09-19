use std::collections::HashMap;

use serde::Serialize;

use crate::regions::region_set::RegionSet;

pub struct GetRegionSets {
}

#[derive(Serialize)]
pub struct GetRegionSetsResult {
    pub track_region_sets_map: HashMap<String,Vec<RegionSet>>,
}
 
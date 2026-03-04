use std::collections::HashMap;

use crate::db::{DbRegionSet, TrackId};

pub struct GetRegionSets {}

pub struct GetRegionSetsResult {
    pub track_region_sets_map: HashMap<TrackId, Vec<DbRegionSet>>,
}

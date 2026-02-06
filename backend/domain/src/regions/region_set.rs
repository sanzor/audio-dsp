use serde::{Deserialize, Serialize};

use crate::regions::region::Region;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegionSet {
    pub track_id: String,
    pub track_length: f32,
    pub region_set_id: String,
    pub name: String,
    pub regions: Vec<Region>,
}

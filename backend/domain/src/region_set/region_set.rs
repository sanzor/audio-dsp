use serde::{Deserialize, Serialize};

use crate::db::{RegionSetId, TrackId};
use crate::regions::region::Region;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegionSet {
    pub track_id: TrackId,
    pub track_length: f32,
    pub region_set_id: RegionSetId,
    pub name: String,
    pub regions: Vec<Region>,
}

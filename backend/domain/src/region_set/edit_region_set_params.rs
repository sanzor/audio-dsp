use crate::db::{RegionSetId, TrackId};

pub struct EditRegionSetParams {
    pub region_set_id: RegionSetId,
    pub track_id: TrackId,
    pub name: Option<String>,
}

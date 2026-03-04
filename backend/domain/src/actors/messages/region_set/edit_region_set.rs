use crate::db::{DbRegionSet, RegionSetId, TrackId};

pub struct EditRegionSet {
    pub region_set_id: RegionSetId,
    pub track_id: TrackId,
    pub name: Option<String>,
}

pub struct EditRegionSetResult {
    pub region_set: DbRegionSet,
}

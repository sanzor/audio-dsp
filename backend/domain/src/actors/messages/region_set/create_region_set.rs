use crate::db::{DbRegionSet, TrackId};

pub struct CreateRegionSet {
    pub track_id: TrackId,
    pub name: Option<String>,
}

pub struct CreateRegionSetResult {
    pub region_set: DbRegionSet,
}

use crate::{db::{RegionSetId, TrackId}, regions::region_subtree::RegionSubtree};

pub struct RegionSetSubtree{
    pub track_id: TrackId,
    pub track_length: f32,
    pub region_set_id: RegionSetId,
    pub name: String,
    pub regions: Vec<RegionSubtree>,
}
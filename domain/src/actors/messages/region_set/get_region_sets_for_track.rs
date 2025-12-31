use dtos::db::region_set_db_dto::RegionSetDbDto;
use serde::Serialize;

pub struct GetRegionSetsForTrack {
    pub track_id: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct GetRegionSetsForTrackResult {
    pub track_id: String,
    pub region_sets: Vec<RegionSetDbDto>,
}

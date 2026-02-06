use serde::Serialize;

use crate::raw_track::RawTrack;

pub struct InsertTrack {
    pub track: RawTrack,
}
#[derive(Serialize)]
pub struct InsertTrackResult {
    pub track_id: String,
    pub user_id: String,
}

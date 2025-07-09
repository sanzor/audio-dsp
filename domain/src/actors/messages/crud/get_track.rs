use serde::Serialize;

use crate::raw_track::RawTrack;

pub struct GetTrack {
    pub track_id: String,
}
#[derive(Serialize)]
pub struct GetTrackResult {
    pub track: RawTrack,
}

use serde::Serialize;

use crate::track::Track;

pub struct GetTrack {
    pub track_id: String,
}
#[derive(Serialize)]
pub struct GetTrackResult {
    pub track: Track,
}

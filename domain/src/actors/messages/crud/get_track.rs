use serde::{Deserialize, Serialize};

use crate::raw_track::RawTrack;

pub struct GetRawTrack {
    pub track_id: String,
}
#[derive(Serialize,Deserialize)]
pub struct GetRawTrackResult {
    pub track: RawTrack,
}

use serde::{Deserialize, Serialize};

use crate::stored_track::StoredTrack;

pub struct GetStoredTrack {
    pub track_id: String,
}
#[derive(Serialize, Deserialize)]
pub struct GetStoredTrackResult {
    pub track: StoredTrack,
}

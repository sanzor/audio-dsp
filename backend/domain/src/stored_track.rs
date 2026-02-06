use serde::{Deserialize, Serialize};

use crate::raw_track::TrackInfo;

// Option A: In-memory only
#[derive(Serialize, Deserialize)]
pub struct StoredTrack {
    pub track_id: String,
    pub track_info: TrackInfo,
    pub canonical_audio: Vec<u8>, // Always in memory
}

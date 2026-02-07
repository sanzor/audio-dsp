use serde::{Deserialize, Serialize};

use crate::{db::TrackId, tracks::track_info::TrackInfo};

// Option A: In-memory only
#[derive(Serialize, Deserialize)]
pub struct StoredTrack {
    pub track_id: TrackId,
    pub track_info: TrackInfo,
    pub canonical_audio: Vec<u8>, // Always in memory
}

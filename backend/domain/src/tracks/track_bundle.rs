use serde::{Deserialize, Serialize};

use super::track_meta::TrackMeta;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackPayload {
    pub canonical_audio: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackBundle {
    pub meta: TrackMeta,
    pub payload: TrackPayload,
}

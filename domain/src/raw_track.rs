use audiolib::audio_buffer::AudioBuffer;
use serde::{Deserialize, Serialize};

use crate::track::Track;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTrack {
    pub info: TrackInfo,
    pub data: AudioBuffer,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    pub name: String,
}

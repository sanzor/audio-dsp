use audiolib::audio_buffer::AudioBuffer;
use serde::{Deserialize, Serialize};

pub use crate::tracks::track_info::TrackInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTrack {
    pub info: TrackInfo,
    pub data: AudioBuffer,
}
use audiolib::audio_buffer::AudioBuffer;
use serde::{Deserialize, Serialize};

use crate::raw_track::TrackInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub track_id: String,
    pub track_info: TrackInfo,
    pub data: AudioBuffer,
}

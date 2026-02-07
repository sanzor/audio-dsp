use audiolib::audio_buffer::AudioBuffer;
use serde::{Deserialize, Serialize};

use crate::{db::TrackId, tracks::track_info::TrackInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub track_id: TrackId,
    pub track_info: TrackInfo,
    pub data: AudioBuffer,
}

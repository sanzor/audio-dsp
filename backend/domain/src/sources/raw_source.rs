use audiolib::audio_buffer::AudioBuffer;
use serde::{Deserialize, Serialize};

pub use crate::sources::source_info::SourceInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawSource {
    pub info: SourceInfo,
    pub data: AudioBuffer,
}

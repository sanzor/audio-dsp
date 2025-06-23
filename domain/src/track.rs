use audiolib::audio_buffer::AudioBuffer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub info: TrackInfo,
    pub data: AudioBuffer,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    pub name: String,
}

pub struct TrackRef<'a> {
    pub inner: &'a Track,
}

pub struct TrackRefMut<'a> {
    pub inner: &'a mut Track,
}

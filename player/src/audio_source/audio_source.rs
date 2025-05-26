use audiolib::Channels;

use crate::AudioFrame;

pub trait AudioSource: Send + Sync {
    fn channels(&self) -> Channels;
    fn sample_rate(&self) -> u64;
    fn next_frame(&mut self) -> Option<AudioFrame>;
}
pub trait SeekableAudioSource: AudioSource {
    fn seek(&mut self, position: u64) -> bool;
}

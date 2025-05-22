use crate::AudioFrame;

pub trait AudioSink {
    fn write_frame(&mut self, frame: AudioFrame) -> Result<(), String>;
}

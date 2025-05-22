use crate::{audio_sink::AudioSink, AudioFrame};

pub struct TestSink {
    pub written: Vec<AudioFrame>,
}

impl AudioSink for TestSink {
    fn write_frame(&mut self, frame: AudioFrame) -> Result<(), String> {
        self.written.push(frame);
        Ok(())
    }
}

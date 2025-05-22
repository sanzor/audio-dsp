use super::AudioSink;

pub struct StdSink {}

impl AudioSink for StdSink {
    fn write_frame(&mut self, frame: crate::AudioFrame) -> Result<(), String> {
        println!("{:?}", frame);
        Ok(())
    }
}

use std::sync::{Arc, Mutex};

use crate::{audio_sink::AudioSink, AudioFrame};

pub struct TestSink {
    pub written: Arc<Mutex<Vec<AudioFrame>>>,
}

impl AudioSink for TestSink {
    fn write_frame(&mut self, frame: &AudioFrame) -> Result<(), String> {
        let collection = &mut *self.written.try_lock().map_err(|e| e.to_string())?;
        collection.push(frame.clone());
        Ok(())
    }
}

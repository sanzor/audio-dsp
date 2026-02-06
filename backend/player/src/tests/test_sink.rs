use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use crate::{sink::AudioSink, AudioFrame};

pub struct TestSink {
    pub written: Arc<Mutex<Vec<AudioFrame>>>,
}
#[async_trait::async_trait]
impl AudioSink for TestSink {
    fn write_frame<'a>(
        &'a mut self,
        frame: AudioFrame,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            let collection = &mut *self.written.try_lock().map_err(|e| e.to_string())?;
            collection.push(frame.clone());
            Ok(())
        })
    }
}

use std::{collections::VecDeque, future::Future, pin::Pin, sync::Arc};


use tokio::sync::Mutex;

use crate::{audio_sink::AudioSink, AudioFrame};

pub struct QueueSink {
    pub queue: Arc<Mutex<VecDeque<AudioFrame>>>,
}

impl AudioSink for QueueSink {
    fn write_frame<'a>(
        &'a mut self,
        frame: AudioFrame,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            let mut guard = self.queue.lock().await;
            guard.push_back(frame);
            Ok(())
        })
    }
}

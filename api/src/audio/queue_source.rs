use std::{collections::VecDeque, future::Future, pin::Pin, sync::Arc};

use player::{audio_source::audio_source::AudioSource, AudioFrame};
use tokio::sync::Mutex;



pub struct QueueSource {
    pub queue: Arc<Mutex<VecDeque<AudioFrame>>>,
}

impl AudioSource for QueueSource {
    fn next_frame<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Option<AudioFrame>> + Send + 'a>> {
        let queue = Arc::clone(&self.queue);
        Box::pin(async move {
            let mut guard = queue.lock().await;
            guard.pop_front()
        })
    }
}

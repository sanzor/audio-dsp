use std::{collections::VecDeque, sync::Arc};

use tokio::sync::Mutex;

use crate::{audio_sink::AudioSink, AudioFrame};

pub struct QueueSink {
    pub queue: Arc<Mutex<VecDeque<AudioFrame>>>,
}

pub trait ASink{
    fn write_frame(&mut self, frame:AudioFrame)->impl std::future::Future<Output = Result<(),String>> + Send;
}
impl ASink for QueueSink {
   async fn write_frame(&mut self, frame:AudioFrame)->Result<(),String>{
    
        let mut q = self.queue.lock().await;
        q.push_back(frame);
        Ok(())
    }
}
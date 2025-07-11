use std::{collections::VecDeque, future::Future, pin::Pin, sync::Arc};

use tokio::sync::Mutex;

use crate::{audio_sink::AudioSink, AudioFrame};

pub struct QueueSink {
    pub queue: Arc<Mutex<VecDeque<AudioFrame>>>,
}


impl AudioSink for QueueSink {
    async fn write_frame(&mut self,frame: AudioFrame) -> Result<(),String> {
       
        let mut q = self.queue.lock().await;
        q.push_back(frame);
        Ok(())   
        
        
    }
}
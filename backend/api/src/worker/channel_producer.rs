use async_trait::async_trait;
use tokio::sync::mpsc;

use super::producer::Producer;

pub struct ChannelProducer<T> {
    sender: mpsc::Sender<T>,
}

impl<T> ChannelProducer<T> {
    pub fn new(sender: mpsc::Sender<T>) -> Self {
        Self { sender }
    }
}

#[async_trait]
impl<T: Send + Sync + 'static> Producer<T> for ChannelProducer<T> {
    async fn produce(&self, event: T) -> Result<(), String> {
        self.sender.send(event).await.map_err(|e| e.to_string())
    }
}

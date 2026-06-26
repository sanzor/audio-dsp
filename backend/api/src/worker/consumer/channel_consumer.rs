use async_trait::async_trait;
use tokio::sync::{mpsc, Mutex};

use super::consumer::Consumer;

pub struct ChannelConsumer<T> {
    receiver: Mutex<mpsc::Receiver<T>>,
}

impl<T: Send> ChannelConsumer<T> {
    pub fn new(receiver: mpsc::Receiver<T>) -> Self {
        Self { receiver: Mutex::new(receiver) }
    }
}

#[async_trait]
impl<T: Send + 'static> Consumer<T> for ChannelConsumer<T> {
    async fn consume(&self) -> Result<T, String> {
        self.receiver
            .lock()
            .await
            .recv()
            .await
            .ok_or_else(|| "channel closed".to_string())
    }
}

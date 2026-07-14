use async_trait::async_trait;
use tokio::sync::mpsc;

use super::consumer::Consumer;

pub struct ChannelConsumer<T> {
    receiver: mpsc::Receiver<T>,
}

impl<T: Send> ChannelConsumer<T> {
    pub fn new(receiver: mpsc::Receiver<T>) -> Self {
        Self { receiver }
    }
}

#[async_trait]
impl<T: Send + 'static> Consumer<T> for ChannelConsumer<T> {
    async fn consume(&mut self) -> Result<T, String> {
        self.receiver
            .recv()
            .await
            .ok_or_else(|| "channel closed".to_string())
    }
}

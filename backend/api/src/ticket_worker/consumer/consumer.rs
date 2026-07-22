use async_trait::async_trait;

#[async_trait]
pub trait Consumer<T: Send>: Send {
    async fn consume(&mut self) -> Result<T, String>;
}

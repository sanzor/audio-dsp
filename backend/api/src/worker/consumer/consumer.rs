use async_trait::async_trait;

#[async_trait]
pub trait Consumer<T: Send>: Send + Sync {
    async fn consume(&self) -> Result<T, String>;
}

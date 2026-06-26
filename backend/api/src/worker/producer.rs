use async_trait::async_trait;

#[async_trait]
pub trait Producer<T: Send>: Send + Sync {
    async fn produce(&self, event: T) -> Result<(), String>;
}

use async_trait::async_trait;

#[async_trait]
pub trait SyncWorkerProcessor: Send + Sync {
    async fn sync(&self) -> Result<(), String>;
}

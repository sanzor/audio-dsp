use domain::db::db_transform::{DbTransformBinary, TransformId};

#[async_trait::async_trait]
pub trait TransformStorageProvider: Send + Sync {
    async fn get_transform_binary(&self, transform_id: TransformId) -> Result<Vec<u8>, String>;
    async fn get_transform_binaries(&self, transform_ids: &[TransformId]) -> Result<Vec<DbTransformBinary>, String>;
    async fn write_transform_binary(&self, transform_id: TransformId, binary: &[u8]) -> Result<(), String>;
}

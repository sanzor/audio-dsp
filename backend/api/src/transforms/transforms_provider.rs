use domain::db::{
    db_transform::{DbTransform, DbTransformBinary, DbTransformDefinition, TransformId},
    ticket::db_resource::ResourceId,
};

use crate::domain::service_error::ServiceError;

#[async_trait::async_trait]
pub trait TransformsProvider: Send + Sync {
    async fn list_transform_summaries(&self, offset: i64, limit: i64) -> Result<(Vec<DbTransform>, i64), ServiceError>;
    async fn get_transform_definition(&self, id: TransformId) -> Result<DbTransformDefinition, ServiceError>;
    async fn get_transform_definitions(&self, ids: &[TransformId]) -> Result<Vec<DbTransformDefinition>, ServiceError>;
    async fn get_transform_binary(&self, id: TransformId) -> Result<Vec<u8>, ServiceError>;
    async fn get_transform_binaries(&self, ids: &[TransformId]) -> Result<Vec<DbTransformBinary>, ServiceError>;
    async fn create_transform(&self, name: String, description: Option<String>, icon: Option<String>) -> Result<DbTransformDefinition, ServiceError>;
    /// Bucket 2 — save. Always overwrites source_code; if `resource_id` is
    /// given, also attaches that compile resource's binary/metadata.
    async fn save_transform_state(&self, id: TransformId, source_code: String, resource_id: Option<ResourceId>) -> Result<DbTransformDefinition, ServiceError>;
    /// Bucket 3 — publish. Bundles whatever's currently saved (bucket 2) into
    /// the live artifact. Fails if nothing has been saved with a binary yet.
    async fn publish_transform(&self, id: TransformId) -> Result<DbTransformDefinition, ServiceError>;
    async fn delete_transform(&self, id: TransformId) -> Result<(), ServiceError>;
}

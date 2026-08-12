use domain::db::{DbTransformGrant, TransformId};

use crate::transform_grants::transform_grants_provider::CreateGrantParams;

#[async_trait::async_trait]
pub trait TransformGrantsDataProvider: Send + Sync {
    async fn create_grant(&self, params: CreateGrantParams) -> Result<DbTransformGrant, String>;
    async fn delete_grant(&self, transform_id: TransformId, grant_id: i64) -> Result<bool, String>;
    async fn list_grants(&self, transform_id: TransformId) -> Result<Vec<DbTransformGrant>, String>;
    async fn has_access(&self, transform_id: TransformId, user_id: i32) -> Result<bool, String>;
}

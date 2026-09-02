use domain::{
    db::{
        db_transform::{DbTransform, TransformId},
        WorkspaceId,
    },
    domain_user::UserId,
};

use crate::domain::service_error::ServiceError;

#[async_trait::async_trait]
pub trait TransformsProvider: Send + Sync {
    async fn list_transform_summaries(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbTransform>, i64), ServiceError>;
    /// Catalog for one workspace — see `TransformsDataProvider::list_transforms_for_workspace_and_user`.
    async fn get_transforms_for_workspace_and_user(
        &self,
        user_id: UserId,
        workspace_id: WorkspaceId,
    ) -> Result<Vec<DbTransform>, ServiceError>;
    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, ServiceError>;
    async fn get_transforms(&self, ids: &[TransformId]) -> Result<Vec<DbTransform>, ServiceError>;
    /// Cheap point-lookup for ownership checks.
    async fn get_transform_owner(&self, id: TransformId) -> Result<UserId, ServiceError>;
    /// All transform ids the user may read — see `TransformsDataProvider`'s
    /// doc comment on the same method.
    async fn list_accessible_transform_ids(
        &self,
        user_id: UserId,
    ) -> Result<Vec<TransformId>, ServiceError>;

    async fn delete_transform(&self, id: TransformId) -> Result<(), ServiceError>;
}

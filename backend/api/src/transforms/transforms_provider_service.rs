use std::sync::Arc;

use crate::domain::service_error::ServiceError;
use domain::db::{
    db_transform::{DbTransform, TransformId},
    WorkspaceId,
};
use domain::domain_user::UserId;

use super::{
    data_provider::transforms_data_provider::TransformsDataProvider,
    transforms_provider::TransformsProvider,
};

pub struct TransformsProviderService {
    data: Arc<dyn TransformsDataProvider>,
}

impl TransformsProviderService {
    pub fn new(data: Arc<dyn TransformsDataProvider>) -> Self {
        Self { data }
    }
}

#[async_trait::async_trait]
impl TransformsProvider for TransformsProviderService {
    async fn list_transform_summaries(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbTransform>, i64), ServiceError> {
        self.data
            .list_transform_summaries(offset, limit)
            .await
            .map_err(ServiceError::from)
    }

    async fn get_transforms_for_workspace_and_user(
        &self,
        user_id: UserId,
        workspace_id: WorkspaceId,
    ) -> Result<Vec<DbTransform>, ServiceError> {
        self.data
            .get_transforms_for_workspace_and_user(user_id, workspace_id)
            .await
            .map_err(ServiceError::from)
    }

    async fn get_transform(&self, id: TransformId) -> Result<DbTransform, ServiceError> {
        self.data
            .get_transform(id)
            .await
            .map_err(ServiceError::from)
    }

    async fn get_transforms(&self, ids: &[TransformId]) -> Result<Vec<DbTransform>, ServiceError> {
        let transforms = self.data.get_transforms(ids).await?;
        let found: std::collections::HashSet<TransformId> =
            transforms.iter().map(|t| t.transform_id).collect();
        let missing: Vec<TransformId> = ids
            .iter()
            .copied()
            .filter(|id| !found.contains(id))
            .collect();
        if missing.is_empty() {
            Ok(transforms)
        } else {
            Err(ServiceError::NotFound)
        }
    }

    async fn get_transform_owner(&self, id: TransformId) -> Result<UserId, ServiceError> {
        self.data
            .get_transform_owner(id)
            .await
            .map_err(ServiceError::from)
    }

    async fn list_accessible_transform_ids(
        &self,
        user_id: UserId,
    ) -> Result<Vec<TransformId>, ServiceError> {
        self.data
            .list_accessible_transform_ids(user_id)
            .await
            .map_err(ServiceError::from)
    }

    async fn delete_transform(&self, id: TransformId) -> Result<(), ServiceError> {
        self.data
            .delete_transform(id)
            .await
            .map_err(ServiceError::from)
    }
}

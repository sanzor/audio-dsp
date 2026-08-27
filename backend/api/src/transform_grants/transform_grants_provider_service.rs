use std::sync::Arc;

use domain::db::{DbTransformGrant, TransformId};

use super::{
    data_provider::transform_grants_data_provider::TransformGrantsDataProvider,
    transform_grants_provider::{CreateGrantParams, TransformGrantsProvider},
};

pub struct TransformGrantsProviderService {
    data_provider: Arc<dyn TransformGrantsDataProvider>,
}

impl TransformGrantsProviderService {
    pub fn new(data_provider: Arc<dyn TransformGrantsDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl TransformGrantsProvider for TransformGrantsProviderService {
    async fn create_grant(&self, params: CreateGrantParams) -> Result<DbTransformGrant, String> {
        self.data_provider.create_grant(params).await
    }

    async fn delete_grant(&self, transform_id: TransformId, grant_id: i64) -> Result<bool, String> {
        self.data_provider.delete_grant(transform_id, grant_id).await
    }

    async fn list_grants(&self, transform_id: TransformId) -> Result<Vec<DbTransformGrant>, String> {
        self.data_provider.list_grants(transform_id).await
    }

    async fn has_access(&self, transform_id: TransformId, user_id: domain::domain_user::UserId) -> Result<bool, String> {
        self.data_provider.has_access(transform_id, user_id).await
    }
}

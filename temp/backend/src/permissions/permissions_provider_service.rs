use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::domain::db::db_permission::PermissionId;
use crate::domain::service_error::ServiceError;
use crate::permissions::create_permission_params::CreatePermissionParams;
use crate::permissions::data_provider::permissions_data_provider::PermissionsDataProvider;
use crate::permissions::update_permission_params::UpdatePermissionParams;
use crate::permissions::{permissions_provider::PermissionsProvider, DbPermission};

pub struct PermissionsProviderService {
    data_provider: Arc<dyn PermissionsDataProvider>,
}

impl PermissionsProviderService {
    pub fn new(data_provider: Arc<dyn PermissionsDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait]
impl PermissionsProvider for PermissionsProviderService {
    async fn create_permission(
        &self,
        params: CreatePermissionParams,
    ) -> Result<DbPermission, ServiceError> {
        info!(key = %params.key, "create permission requested");
        Ok(self
            .data_provider
            .create_permission(params.key, params.description)
            .await?)
    }

    async fn update_permission(
        &self,
        permission_id: PermissionId,
        params: UpdatePermissionParams,
    ) -> Result<Option<DbPermission>, ServiceError> {
        info!(permission_id, "update permission requested");
        Ok(self
            .data_provider
            .update_permission(permission_id, params.description)
            .await?)
    }

    async fn delete_permission(&self, permission_id: PermissionId) -> Result<bool, ServiceError> {
        info!(permission_id, "delete permission requested");
        Ok(self.data_provider.delete_permission(permission_id).await?)
    }

    async fn get_permission(
        &self,
        permission_id: PermissionId,
    ) -> Result<Option<DbPermission>, ServiceError> {
        info!(permission_id, "get permission requested");
        Ok(self.data_provider.get_permission(permission_id).await?)
    }

    async fn list_permissions(&self) -> Result<Vec<DbPermission>, ServiceError> {
        info!("list permissions requested");
        Ok(self.data_provider.list_permissions().await?)
    }
}

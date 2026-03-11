use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::domain::db::db_permission::PermissionId;
use crate::domain::db::db_role::RoleId;
use crate::domain::db::db_role_permission::DbRolePermission;
use crate::domain::service_error::ServiceError;
use crate::role_permissions::create_role_permission_params::CreateRolePermissionParams;
use crate::role_permissions::data_provider::role_permissions_data_provider::RolePermissionsDataProvider;
use crate::role_permissions::role_permissions_provider::RolePermissionsProvider;

pub struct RolePermissionsProviderService {
    data_provider: Arc<dyn RolePermissionsDataProvider>,
}

impl RolePermissionsProviderService {
    pub fn new(data_provider: Arc<dyn RolePermissionsDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait]
impl RolePermissionsProvider for RolePermissionsProviderService {
    async fn create_role_permission(
        &self,
        params: CreateRolePermissionParams,
    ) -> Result<DbRolePermission, ServiceError> {
        info!(
            role_id = params.role_id,
            permission_id = params.permission_id,
            "create role permission requested"
        );
        Ok(self
            .data_provider
            .create_role_permission(params.role_id, params.permission_id)
            .await?)
    }

    async fn delete_role_permission(
        &self,
        role_id: RoleId,
        permission_id: PermissionId,
    ) -> Result<bool, ServiceError> {
        info!(role_id, permission_id, "delete role permission requested");
        Ok(self
            .data_provider
            .delete_role_permission(role_id, permission_id)
            .await?)
    }

    async fn get_role_permission(
        &self,
        role_id: RoleId,
        permission_id: PermissionId,
    ) -> Result<Option<DbRolePermission>, ServiceError> {
        info!(role_id, permission_id, "get role permission requested");
        Ok(self
            .data_provider
            .get_role_permission(role_id, permission_id)
            .await?)
    }

    async fn list_role_permissions(
        &self,
        role_id: Option<RoleId>,
        permission_id: Option<PermissionId>,
    ) -> Result<Vec<DbRolePermission>, ServiceError> {
        info!(role_id = ?role_id, permission_id = ?permission_id, "list role permissions requested");
        Ok(self
            .data_provider
            .list_role_permissions(role_id, permission_id)
            .await?)
    }
}

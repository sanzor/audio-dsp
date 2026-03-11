use async_trait::async_trait;

use crate::domain::db::db_permission::PermissionId;
use crate::domain::service_error::ServiceError;
use crate::permissions::{
    create_permission_params::CreatePermissionParams,
    update_permission_params::UpdatePermissionParams, DbPermission,
};

#[async_trait]
pub trait PermissionsProvider: Send + Sync {
    async fn create_permission(
        &self,
        params: CreatePermissionParams,
    ) -> Result<DbPermission, ServiceError>;
    async fn update_permission(
        &self,
        permission_id: PermissionId,
        params: UpdatePermissionParams,
    ) -> Result<Option<DbPermission>, ServiceError>;
    async fn delete_permission(&self, permission_id: PermissionId) -> Result<bool, ServiceError>;
    async fn get_permission(
        &self,
        permission_id: PermissionId,
    ) -> Result<Option<DbPermission>, ServiceError>;
    async fn list_permissions(&self) -> Result<Vec<DbPermission>, ServiceError>;
}

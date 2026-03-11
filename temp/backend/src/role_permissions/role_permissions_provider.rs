use async_trait::async_trait;

use crate::domain::db::db_permission::PermissionId;
use crate::domain::db::db_role::RoleId;
use crate::domain::db::db_role_permission::DbRolePermission;
use crate::domain::service_error::ServiceError;
use crate::role_permissions::create_role_permission_params::CreateRolePermissionParams;

#[async_trait]
pub trait RolePermissionsProvider: Send + Sync {
    async fn create_role_permission(
        &self,
        params: CreateRolePermissionParams,
    ) -> Result<DbRolePermission, ServiceError>;
    async fn delete_role_permission(
        &self,
        role_id: RoleId,
        permission_id: PermissionId,
    ) -> Result<bool, ServiceError>;
    async fn get_role_permission(
        &self,
        role_id: RoleId,
        permission_id: PermissionId,
    ) -> Result<Option<DbRolePermission>, ServiceError>;
    async fn list_role_permissions(
        &self,
        role_id: Option<RoleId>,
        permission_id: Option<PermissionId>,
    ) -> Result<Vec<DbRolePermission>, ServiceError>;
}

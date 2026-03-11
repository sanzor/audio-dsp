use async_trait::async_trait;

use crate::domain::{
    data_error::DataError,
    db::{db_permission::PermissionId, db_role::RoleId, db_role_permission::DbRolePermission},
};

#[async_trait]
pub trait RolePermissionsDataProvider: Send + Sync {
    async fn create_role_permission(
        &self,
        role_id: RoleId,
        permission_id: PermissionId,
    ) -> Result<DbRolePermission, DataError>;
    async fn delete_role_permission(
        &self,
        role_id: RoleId,
        permission_id: PermissionId,
    ) -> Result<bool, DataError>;
    async fn get_role_permission(
        &self,
        role_id: RoleId,
        permission_id: PermissionId,
    ) -> Result<Option<DbRolePermission>, DataError>;
    async fn list_role_permissions(
        &self,
        role_id: Option<RoleId>,
        permission_id: Option<PermissionId>,
    ) -> Result<Vec<DbRolePermission>, DataError>;
}

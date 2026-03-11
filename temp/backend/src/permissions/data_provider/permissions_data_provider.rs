use async_trait::async_trait;

use crate::{
    domain::{data_error::DataError, db::db_permission::PermissionId},
    permissions::DbPermission,
};

#[async_trait]
pub trait PermissionsDataProvider: Send + Sync {
    async fn create_permission(
        &self,
        key: String,
        description: Option<String>,
    ) -> Result<DbPermission, DataError>;
    async fn update_permission(
        &self,
        permission_id: PermissionId,
        description: Option<String>,
    ) -> Result<Option<DbPermission>, DataError>;
    async fn delete_permission(&self, permission_id: PermissionId) -> Result<bool, DataError>;
    async fn get_permission(
        &self,
        permission_id: PermissionId,
    ) -> Result<Option<DbPermission>, DataError>;
    async fn list_permissions(&self) -> Result<Vec<DbPermission>, DataError>;
}

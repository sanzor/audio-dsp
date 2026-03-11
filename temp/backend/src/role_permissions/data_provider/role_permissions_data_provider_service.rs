use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::data_error::DataError;
use crate::domain::db::db_permission::PermissionId;
use crate::domain::db::db_role::RoleId;
use crate::{
    domain::db::db_role_permission::DbRolePermission,
    role_permissions::data_provider::role_permissions_data_provider::RolePermissionsDataProvider,
};

pub struct RolePermissionsDataProviderService {
    pool: PgPool,
}

impl RolePermissionsDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RolePermissionsDataProvider for RolePermissionsDataProviderService {
    async fn create_role_permission(
        &self,
        role_id: RoleId,
        permission_id: PermissionId,
    ) -> Result<DbRolePermission, DataError> {
        let role_id = i64::try_from(role_id)
            .map_err(|_| DataError::Internal("role id out of range".into()))?;
        let permission_id = i64::try_from(permission_id)
            .map_err(|_| DataError::Internal("permission id out of range".into()))?;
        let record = sqlx::query_as::<_, DbRolePermission>(
            "INSERT INTO role_permissions (role_id, permission_id) \
             VALUES ($1, $2) \
             ON CONFLICT (role_id, permission_id) DO UPDATE SET role_id = EXCLUDED.role_id \
             RETURNING role_id, permission_id",
        )
        .bind(role_id)
        .bind(permission_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    async fn delete_role_permission(
        &self,
        role_id: RoleId,
        permission_id: PermissionId,
    ) -> Result<bool, DataError> {
        let role_id = i64::try_from(role_id)
            .map_err(|_| DataError::Internal("role id out of range".into()))?;
        let permission_id = i64::try_from(permission_id)
            .map_err(|_| DataError::Internal("permission id out of range".into()))?;
        let result =
            sqlx::query("DELETE FROM role_permissions WHERE role_id = $1 AND permission_id = $2")
                .bind(role_id)
                .bind(permission_id)
                .execute(&self.pool)
                .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_role_permission(
        &self,
        role_id: RoleId,
        permission_id: PermissionId,
    ) -> Result<Option<DbRolePermission>, DataError> {
        let role_id = i64::try_from(role_id)
            .map_err(|_| DataError::Internal("role id out of range".into()))?;
        let permission_id = i64::try_from(permission_id)
            .map_err(|_| DataError::Internal("permission id out of range".into()))?;
        let record = sqlx::query_as::<_, DbRolePermission>(
            "SELECT role_id, permission_id FROM role_permissions WHERE role_id = $1 AND permission_id = $2",
        )
        .bind(role_id)
        .bind(permission_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn list_role_permissions(
        &self,
        role_id: Option<RoleId>,
        permission_id: Option<PermissionId>,
    ) -> Result<Vec<DbRolePermission>, DataError> {
        let role_id = role_id
            .map(i64::try_from)
            .transpose()
            .map_err(|_| DataError::Internal("role id out of range".into()))?;
        let permission_id = permission_id
            .map(i64::try_from)
            .transpose()
            .map_err(|_| DataError::Internal("permission id out of range".into()))?;

        let records = sqlx::query_as::<_, DbRolePermission>(
            "SELECT role_id, permission_id FROM role_permissions \
             WHERE ($1::BIGINT IS NULL OR role_id = $1) \
               AND ($2::BIGINT IS NULL OR permission_id = $2) \
             ORDER BY role_id, permission_id",
        )
        .bind(role_id)
        .bind(permission_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }
}

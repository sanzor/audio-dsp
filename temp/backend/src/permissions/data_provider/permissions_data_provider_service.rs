use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::data_error::DataError;
use crate::domain::db::db_permission::PermissionId;
use crate::permissions::data_provider::permissions_data_provider::PermissionsDataProvider;
use crate::permissions::DbPermission;

pub struct PermissionsDataProviderService {
    pool: PgPool,
}

impl PermissionsDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PermissionsDataProvider for PermissionsDataProviderService {
    async fn create_permission(
        &self,
        key: String,
        description: Option<String>,
    ) -> Result<DbPermission, DataError> {
        let record = sqlx::query_as::<_, DbPermission>(
            "INSERT INTO permissions (key, description) \
             VALUES ($1, $2) \
             ON CONFLICT (key) DO UPDATE SET description = EXCLUDED.description \
             RETURNING id, key, description",
        )
        .bind(key)
        .bind(description)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    async fn update_permission(
        &self,
        permission_id: PermissionId,
        description: Option<String>,
    ) -> Result<Option<DbPermission>, DataError> {
        let permission_id = i64::try_from(permission_id)
            .map_err(|_| DataError::Internal("permission id out of range".into()))?;
        let record = sqlx::query_as::<_, DbPermission>(
            "UPDATE permissions SET description = $2 WHERE id = $1 RETURNING id, key, description",
        )
        .bind(permission_id)
        .bind(description)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn delete_permission(&self, permission_id: PermissionId) -> Result<bool, DataError> {
        let permission_id = PermissionId::try_from(permission_id)
            .map_err(|_| DataError::Internal("permission id out of range".into()))?;
        let result = sqlx::query("DELETE FROM permissions WHERE id = $1")
            .bind(permission_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_permission(
        &self,
        permission_id: PermissionId,
    ) -> Result<Option<DbPermission>, DataError> {
        let permission_id = PermissionId::try_from(permission_id)
            .map_err(|_| DataError::Internal("permission id out of range".into()))?;
        let record = sqlx::query_as::<_, DbPermission>(
            "SELECT id, key, description FROM permissions WHERE id = $1",
        )
        .bind(permission_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn list_permissions(&self) -> Result<Vec<DbPermission>, DataError> {
        let records = sqlx::query_as::<_, DbPermission>(
            "SELECT id, key, description FROM permissions ORDER BY key",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }
}

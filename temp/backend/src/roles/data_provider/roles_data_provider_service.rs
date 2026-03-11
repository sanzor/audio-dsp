use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::data_error::DataError;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_role::RoleId;
use crate::roles::data_provider::roles_data_provider::RolesDataProvider;
use crate::roles::DbRole;

pub struct RolesDataProviderService {
    pool: PgPool,
}

impl RolesDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RolesDataProvider for RolesDataProviderService {
    async fn create_role(
        &self,
        org_id: OrganizationId,
        name: String,
        is_system: bool,
    ) -> Result<DbRole, DataError> {
        let org_id = OrganizationId::try_from(org_id)
            .map_err(|_| DataError::Internal("org id out of range".into()))?;
        let record = sqlx::query_as::<_, DbRole>(
            "INSERT INTO roles (org_id, name, is_system) \
             VALUES ($1, $2, $3) \
             ON CONFLICT (org_id, name) DO UPDATE SET is_system = EXCLUDED.is_system \
             RETURNING id, org_id, name, is_system",
        )
        .bind(org_id)
        .bind(name)
        .bind(is_system)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    async fn update_role(
        &self,
        role_id: RoleId,
        name: String,
    ) -> Result<Option<DbRole>, DataError> {
        let role_id = RoleId::try_from(role_id)
            .map_err(|_| DataError::Internal("role id out of range".into()))?;
        let record = sqlx::query_as::<_, DbRole>(
            "UPDATE roles SET name = $2 WHERE id = $1 RETURNING id, org_id, name, is_system",
        )
        .bind(role_id)
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn delete_role(&self, role_id: RoleId) -> Result<bool, DataError> {
        let role_id = RoleId::try_from(role_id)
            .map_err(|_| DataError::Internal("role id out of range".into()))?;
        let result = sqlx::query("DELETE FROM roles WHERE id = $1")
            .bind(role_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_role(&self, role_id: RoleId) -> Result<Option<DbRole>, DataError> {
        let role_id = RoleId::try_from(role_id)
            .map_err(|_| DataError::Internal("role id out of range".into()))?;
        let record = sqlx::query_as::<_, DbRole>(
            "SELECT id, org_id, name, is_system FROM roles WHERE id = $1",
        )
        .bind(role_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn list_roles(&self, org_id: Option<OrganizationId>) -> Result<Vec<DbRole>, DataError> {
        let org_id = org_id
            .map(OrganizationId::try_from)
            .transpose()
            .map_err(|_| DataError::Internal("org id out of range".into()))?;
        let records = sqlx::query_as::<_, DbRole>(
            "SELECT id, org_id, name, is_system FROM roles \
             WHERE ($1::BIGINT IS NULL OR org_id = $1) \
             ORDER BY org_id, name",
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }
}

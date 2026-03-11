use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::data_error::DataError;
use crate::{
    access_control::data_provider::access_control_data_provider::AccessControlDataProvider,
    domain::db::{db_organization::OrganizationId, db_user::UserId},
};

pub struct AccessControlDataProviderService {
    pool: PgPool,
}

impl AccessControlDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccessControlDataProvider for AccessControlDataProviderService {
    async fn has_permission(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        permission_key: &str,
    ) -> Result<bool, DataError> {
        let user_id = i64::try_from(user_id)
            .map_err(|_| DataError::Internal("user id out of range".into()))?;
        let org_id =
            i64::try_from(org_id).map_err(|_| DataError::Internal("org id out of range".into()))?;

        let record = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (\
                SELECT 1\
                FROM membership_roles mr\
                JOIN role_permissions rp ON rp.role_id = mr.role_id\
                JOIN permissions p ON p.id = rp.permission_id\
                WHERE mr.user_id = $1 AND mr.org_id = $2 AND p.key = $3\
            )",
        )
        .bind(user_id)
        .bind(org_id)
        .bind(permission_key)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    async fn permissions_for_user(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<Vec<String>, DataError> {
        let user_id = i64::try_from(user_id)
            .map_err(|_| DataError::Internal("user id out of range".into()))?;
        let org_id =
            i64::try_from(org_id).map_err(|_| DataError::Internal("org id out of range".into()))?;

        let records = sqlx::query_scalar::<_, String>(
            "SELECT DISTINCT p.key \
            FROM membership_roles mr \
            JOIN role_permissions rp ON rp.role_id = mr.role_id \
            JOIN permissions p ON p.id = rp.permission_id \
            WHERE mr.user_id = $1 AND mr.org_id = $2 \
            ORDER BY p.key",
        )
        .bind(user_id)
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }
}

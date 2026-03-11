use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::data_error::DataError;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::memberships::data_provider::memberships_data_provider::MembershipsDataProvider;
use crate::memberships::DbMembership;

pub struct MembershipsDataProviderService {
    pool: PgPool,
}

impl MembershipsDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MembershipsDataProvider for MembershipsDataProviderService {
    async fn create_membership(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<DbMembership, DataError> {
        let user_id = UserId::try_from(user_id)
            .map_err(|_| DataError::Internal("user id out of range".into()))?;
        let org_id = OrganizationId::try_from(org_id)
            .map_err(|_| DataError::Internal("org id out of range".into()))?;

        let mut tx = self.pool.begin().await?;

        let membership = sqlx::query_as::<_, DbMembership>(
            "INSERT INTO memberships (user_id, org_id) \
             VALUES ($1, $2) \
             ON CONFLICT (user_id, org_id) DO UPDATE SET org_id = EXCLUDED.org_id \
             RETURNING user_id, org_id",
        )
        .bind(user_id)
        .bind(org_id)
        .fetch_one(&mut *tx)
        .await?;

        // Assign the default "dev" role if the membership currently has no roles.
        // Role/permissions are seeded/migrated; this code only links the membership to it.
        let has_any_role = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (\
                SELECT 1 FROM membership_roles WHERE user_id = $1 AND org_id = $2\
            )",
        )
        .bind(user_id)
        .bind(org_id)
        .fetch_one(&mut *tx)
        .await?;

        if !has_any_role {
            let member_role_id = sqlx::query_scalar::<_, i64>(
                "SELECT id FROM roles WHERE org_id = $1 AND name = 'dev' LIMIT 1",
            )
            .bind(org_id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| DataError::Internal("default role 'dev' not found for org".into()))?;

            sqlx::query(
                "INSERT INTO membership_roles (user_id, org_id, role_id) \
                 VALUES ($1, $2, $3) \
                 ON CONFLICT (user_id, org_id, role_id) DO NOTHING",
            )
            .bind(user_id)
            .bind(org_id)
            .bind(member_role_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(membership)
    }

    async fn delete_membership(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<bool, DataError> {
        let user_id = UserId::try_from(user_id)
            .map_err(|_| DataError::Internal("user id out of range".into()))?;
        let org_id = OrganizationId::try_from(org_id)
            .map_err(|_| DataError::Internal("org id out of range".into()))?;
        let result = sqlx::query("DELETE FROM memberships WHERE user_id = $1 AND org_id = $2")
            .bind(user_id)
            .bind(org_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_membership(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<Option<DbMembership>, DataError> {
        let user_id = UserId::try_from(user_id)
            .map_err(|_| DataError::Internal("user id out of range".into()))?;
        let org_id = OrganizationId::try_from(org_id)
            .map_err(|_| DataError::Internal("org id out of range".into()))?;
        let record = sqlx::query_as::<_, DbMembership>(
            "SELECT user_id, org_id FROM memberships WHERE user_id = $1 AND org_id = $2",
        )
        .bind(user_id)
        .bind(org_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn list_memberships(
        &self,
        user_id: Option<UserId>,
        org_id: Option<OrganizationId>,
    ) -> Result<Vec<DbMembership>, DataError> {
        let user_id = user_id
            .map(i64::try_from)
            .transpose()
            .map_err(|_| DataError::Internal("user id out of range".into()))?;
        let org_id = org_id
            .map(i64::try_from)
            .transpose()
            .map_err(|_| DataError::Internal("org id out of range".into()))?;

        let records = sqlx::query_as::<_, DbMembership>(
            "SELECT user_id, org_id FROM memberships \
             WHERE ($1::BIGINT IS NULL OR user_id = $1) \
               AND ($2::BIGINT IS NULL OR org_id = $2) \
             ORDER BY org_id, user_id",
        )
        .bind(user_id)
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }
}

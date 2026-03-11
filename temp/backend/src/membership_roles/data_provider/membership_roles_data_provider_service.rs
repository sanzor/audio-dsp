use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::data_error::DataError;
use crate::{
    domain::db::{db_organization::OrganizationId, db_role::RoleId, db_user::UserId},
    membership_roles::{
        data_provider::membership_roles_data_provider::MembershipRolesDataProvider,
        DbMembershipRole,
    },
};

pub struct MembershipRolesDataProviderService {
    pool: PgPool,
}

impl MembershipRolesDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MembershipRolesDataProvider for MembershipRolesDataProviderService {
    async fn create_membership_role(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        role_id: RoleId,
    ) -> Result<DbMembershipRole, DataError> {
        let user_id = i64::try_from(user_id)
            .map_err(|_| DataError::Internal("user id out of range".into()))?;
        let org_id =
            i64::try_from(org_id).map_err(|_| DataError::Internal("org id out of range".into()))?;
        let role_id = i64::try_from(role_id)
            .map_err(|_| DataError::Internal("role id out of range".into()))?;
        let record = sqlx::query_as::<_, DbMembershipRole>(
            "INSERT INTO membership_roles (user_id, org_id, role_id) \
             VALUES ($1, $2, $3) \
             ON CONFLICT (user_id, org_id, role_id) DO UPDATE SET role_id = EXCLUDED.role_id \
             RETURNING user_id, org_id, role_id",
        )
        .bind(user_id)
        .bind(org_id)
        .bind(role_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    async fn delete_membership_role(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        role_id: RoleId,
    ) -> Result<bool, DataError> {
        let user_id = i64::try_from(user_id)
            .map_err(|_| DataError::Internal("user id out of range".into()))?;
        let org_id =
            i64::try_from(org_id).map_err(|_| DataError::Internal("org id out of range".into()))?;
        let role_id = i64::try_from(role_id)
            .map_err(|_| DataError::Internal("role id out of range".into()))?;
        let result = sqlx::query(
            "DELETE FROM membership_roles WHERE user_id = $1 AND org_id = $2 AND role_id = $3",
        )
        .bind(user_id)
        .bind(org_id)
        .bind(role_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_membership_role(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        role_id: RoleId,
    ) -> Result<Option<DbMembershipRole>, DataError> {
        let user_id = i64::try_from(user_id)
            .map_err(|_| DataError::Internal("user id out of range".into()))?;
        let org_id =
            i64::try_from(org_id).map_err(|_| DataError::Internal("org id out of range".into()))?;
        let role_id = i64::try_from(role_id)
            .map_err(|_| DataError::Internal("role id out of range".into()))?;
        let record = sqlx::query_as::<_, DbMembershipRole>(
            "SELECT user_id, org_id, role_id FROM membership_roles \
             WHERE user_id = $1 AND org_id = $2 AND role_id = $3",
        )
        .bind(user_id)
        .bind(org_id)
        .bind(role_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn list_membership_roles(
        &self,
        user_id: Option<UserId>,
        org_id: Option<OrganizationId>,
        role_id: Option<RoleId>,
    ) -> Result<Vec<DbMembershipRole>, DataError> {
        let user_id = user_id
            .map(i64::try_from)
            .transpose()
            .map_err(|_| DataError::Internal("user id out of range".into()))?;
        let org_id = org_id
            .map(i64::try_from)
            .transpose()
            .map_err(|_| DataError::Internal("org id out of range".into()))?;
        let role_id = role_id
            .map(i64::try_from)
            .transpose()
            .map_err(|_| DataError::Internal("role id out of range".into()))?;

        let records = sqlx::query_as::<_, DbMembershipRole>(
            "SELECT user_id, org_id, role_id FROM membership_roles \
             WHERE ($1::BIGINT IS NULL OR user_id = $1) \
               AND ($2::BIGINT IS NULL OR org_id = $2) \
               AND ($3::BIGINT IS NULL OR role_id = $3) \
             ORDER BY org_id, user_id, role_id",
        )
        .bind(user_id)
        .bind(org_id)
        .bind(role_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }
}

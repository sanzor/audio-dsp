use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::data_error::DataError;
use crate::domain::db::db_membership::DbMembership;
use crate::domain::db::db_organization::{DbOrganization, OrganizationId};
use crate::domain::db::db_role::DbRole;
use crate::domain::db::db_user::UserId;
use crate::organizations::create_organization_params::CreateOrganizationParams;
use crate::organizations::create_organization_result::CreateOrganizationWithOwnerResult;
use crate::organizations::data_provider::organization_data_provider::OrganizationDataProvider;
use crate::organizations::update_organization_params::UpdateOrganizationParams;

pub struct OrganizationDataProviderService {
    pool: PgPool,
}

impl OrganizationDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrganizationDataProvider for OrganizationDataProviderService {
    async fn create_organization(
        &self,
        params: CreateOrganizationParams,
    ) -> Result<DbOrganization, DataError> {
        let record = sqlx::query_as::<_, DbOrganization>(
            "INSERT INTO organizations (name, slug, billing_email, stripe_customer_id, status) \
             VALUES ($1, $2, $3, $4, COALESCE($5, 'active')) \
             RETURNING id, name, slug, billing_email, stripe_customer_id, status",
        )
        .bind(params.name)
        .bind(params.slug)
        .bind(params.billing_email)
        .bind(params.stripe_customer_id)
        .bind(params.status)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    async fn create_organization_with_owner(
        &self,
        params: CreateOrganizationParams,
        user_id: UserId,
    ) -> Result<CreateOrganizationWithOwnerResult, DataError> {
        let mut tx = self.pool.begin().await?;

        let organization = sqlx::query_as::<_, DbOrganization>(
            "INSERT INTO organizations (name, slug, billing_email, stripe_customer_id, status) \
             VALUES ($1, $2, $3, $4, COALESCE($5, 'active')) \
             RETURNING id, name, slug, billing_email, stripe_customer_id, status",
        )
        .bind(&params.name)
        .bind(&params.slug)
        .bind(&params.billing_email)
        .bind(&params.stripe_customer_id)
        .bind(&params.status)
        .fetch_one(&mut *tx)
        .await?;

        let membership = sqlx::query_as::<_, DbMembership>(
            "INSERT INTO memberships (user_id, org_id) \
             VALUES ($1, $2) \
             ON CONFLICT (user_id, org_id) DO UPDATE SET org_id = EXCLUDED.org_id \
             RETURNING user_id, org_id",
        )
        .bind(user_id)
        .bind(organization.id)
        .fetch_one(&mut *tx)
        .await?;

        let owner_role = sqlx::query_as::<_, DbRole>(
            "INSERT INTO roles (org_id, name, is_system) \
             VALUES ($1, 'owner', true) \
             ON CONFLICT (org_id, name) DO UPDATE SET is_system = EXCLUDED.is_system \
             RETURNING id, org_id, name, is_system",
        )
        .bind(organization.id)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO role_permissions (role_id, permission_id) \
             SELECT $1, id FROM permissions \
             ON CONFLICT (role_id, permission_id) DO NOTHING",
        )
        .bind(owner_role.id)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO membership_roles (user_id, org_id, role_id) \
             VALUES ($1, $2, $3) \
             ON CONFLICT (user_id, org_id, role_id) DO UPDATE SET role_id = EXCLUDED.role_id",
        )
        .bind(user_id)
        .bind(organization.id)
        .bind(owner_role.id)
        .execute(&mut *tx)
        .await?;

        // Ensure the default "dev" role exists for the org with dev-friendly permissions.
        // This role is assigned automatically when a user joins an org.
        let member_role = sqlx::query_as::<_, DbRole>(
            "INSERT INTO roles (org_id, name, is_system) \
             VALUES ($1, 'dev', true) \
             ON CONFLICT (org_id, name) DO UPDATE SET is_system = EXCLUDED.is_system \
             RETURNING id, org_id, name, is_system",
        )
        .bind(organization.id)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO role_permissions (role_id, permission_id) \
             SELECT $1, p.id \
             FROM permissions p \
             WHERE p.key IN (\
                'api_keys:create', 'api_keys:read', 'api_keys:update', 'api_keys:delete', \
                'products:read', \
                'checkout:create', \
                'purchased_products:read'\
             ) \
             ON CONFLICT (role_id, permission_id) DO NOTHING",
        )
        .bind(member_role.id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(CreateOrganizationWithOwnerResult {
            organization,
            membership,
            owner_role,
        })
    }

    async fn update_organization(
        &self,
        org_id: OrganizationId,
        params: UpdateOrganizationParams,
    ) -> Result<Option<DbOrganization>, DataError> {
        let org_id =
            i64::try_from(org_id).map_err(|_| DataError::Internal("org id out of range".into()))?;
        let record = sqlx::query_as::<_, DbOrganization>(
            "UPDATE organizations \
             SET name = COALESCE($2, name), \
                 slug = COALESCE($3, slug), \
                 billing_email = COALESCE($4, billing_email), \
                 stripe_customer_id = COALESCE($5, stripe_customer_id), \
                 status = COALESCE($6, status), \
                 updated_at = NOW() \
             WHERE id = $1 \
             RETURNING id, name, slug, billing_email, stripe_customer_id, status",
        )
        .bind(org_id)
        .bind(params.name)
        .bind(params.slug)
        .bind(params.billing_email)
        .bind(params.stripe_customer_id)
        .bind(params.status)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn delete_organization(&self, org_id: OrganizationId) -> Result<bool, DataError> {
        let org_id =
            i64::try_from(org_id).map_err(|_| DataError::Internal("org id out of range".into()))?;
        let result = sqlx::query("DELETE FROM organizations WHERE id = $1")
            .bind(org_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_organization(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<DbOrganization>, DataError> {
        let org_id =
            i64::try_from(org_id).map_err(|_| DataError::Internal("org id out of range".into()))?;
        let record = sqlx::query_as::<_, DbOrganization>(
            "SELECT id, name, slug, billing_email, stripe_customer_id, status \
             FROM organizations WHERE id = $1",
        )
        .bind(org_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(record)
    }

    async fn get_all_organizations(&self) -> Result<Vec<DbOrganization>, DataError> {
        let records = sqlx::query_as::<_, DbOrganization>(
            "SELECT id, name, slug, billing_email, stripe_customer_id, status \
             FROM organizations ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }
}

use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;

use crate::domain::data_error::DataError;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::Tier;
use crate::subscriptions::data_provider::subscriptions_data_provider::SubscriptionsDataProvider;
use crate::subscriptions::DbSubscription;

pub struct SubscriptionsDataProviderService {
    pool: PgPool,
}

impl SubscriptionsDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SubscriptionsDataProvider for SubscriptionsDataProviderService {
    async fn create_subscription(
        &self,
        org_id: OrganizationId,
        tier: Tier,
        stripe_subscription_id: Option<String>,
        status: String,
        current_period_end: Option<String>,
    ) -> Result<DbSubscription, DataError> {
        let org_id = i64::try_from(org_id)
            .map_err(|_| DataError::Internal("org id out of range".to_string()))?;
        let record = sqlx::query_as::<_, DbSubscription>(
            "INSERT INTO subscriptions (org_id, tier, stripe_subscription_id, status, current_period_end) \
             VALUES ($1, $2, $3, $4, $5::timestamptz) \
             ON CONFLICT (org_id) DO UPDATE \
             SET tier = EXCLUDED.tier, stripe_subscription_id = EXCLUDED.stripe_subscription_id, \
                 status = EXCLUDED.status, current_period_end = EXCLUDED.current_period_end \
             RETURNING id, org_id, tier, stripe_subscription_id, status, current_period_end::text",
        )
        .bind(org_id)
        .bind(tier)
        .bind(stripe_subscription_id)
        .bind(status)
        .bind(current_period_end)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "create subscription failed");
            DataError::from(e)
        })?;

        Ok(record)
    }

    async fn update_subscription(
        &self,
        org_id: OrganizationId,
        tier: Option<Tier>,
        stripe_subscription_id: Option<String>,
        status: Option<String>,
        current_period_end: Option<String>,
    ) -> Result<Option<DbSubscription>, DataError> {
        let org_id = i64::try_from(org_id)
            .map_err(|_| DataError::Internal("org id out of range".to_string()))?;
        let record = sqlx::query_as::<_, DbSubscription>(
            "UPDATE subscriptions \
             SET tier = COALESCE($2, tier), \
                 stripe_subscription_id = COALESCE($3, stripe_subscription_id), \
                 status = COALESCE($4, status), \
                 current_period_end = COALESCE($5::timestamptz, current_period_end) \
             WHERE org_id = $1 \
             RETURNING id, org_id, tier, stripe_subscription_id, status, current_period_end::text",
        )
        .bind(org_id)
        .bind(tier)
        .bind(stripe_subscription_id)
        .bind(status)
        .bind(current_period_end)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "update subscription failed");
            DataError::from(e)
        })?;

        Ok(record)
    }

    async fn delete_subscription(&self, org_id: OrganizationId) -> Result<bool, DataError> {
        let org_id = i64::try_from(org_id)
            .map_err(|_| DataError::Internal("org id out of range".to_string()))?;
        let result = sqlx::query("DELETE FROM subscriptions WHERE org_id = $1")
            .bind(org_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!(error = %e, "delete subscription failed");
                DataError::from(e)
            })?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_subscription(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<DbSubscription>, DataError> {
        let org_id = i64::try_from(org_id)
            .map_err(|_| DataError::Internal("org id out of range".to_string()))?;
        let record = sqlx::query_as::<_, DbSubscription>(
            "SELECT id, org_id, tier, stripe_subscription_id, status, current_period_end::text \
             FROM subscriptions WHERE org_id = $1",
        )
        .bind(org_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "get subscription failed");
            DataError::from(e)
        })?;

        Ok(record)
    }

    async fn list_subscriptions(&self) -> Result<Vec<DbSubscription>, DataError> {
        let records = sqlx::query_as::<_, DbSubscription>(
            "SELECT id, org_id, tier, stripe_subscription_id, status, current_period_end::text \
             FROM subscriptions ORDER BY org_id",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "list subscriptions failed");
            DataError::from(e)
        })?;

        Ok(records)
    }

    async fn find_by_stripe_subscription_id(
        &self,
        stripe_sub_id: &str,
    ) -> Result<Option<DbSubscription>, DataError> {
        sqlx::query_as::<_, DbSubscription>(
            "SELECT id, org_id, tier, stripe_subscription_id, status, current_period_end::text \
             FROM subscriptions WHERE stripe_subscription_id = $1",
        )
        .bind(stripe_sub_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(stripe_sub_id, error = %e, "find subscription by stripe_sub_id failed");
            DataError::from(e)
        })
    }
}

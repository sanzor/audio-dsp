use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;
use crate::domain::data_error::DataError;
use crate::domain::db::db_subscription::{DbSubscription, SubscriptionId};
use crate::subscriptions::create_subscription_params::CreateSubscriptionParams;
use crate::subscriptions::data_provider::subscriptions_data_provider::SubscriptionsDataProvider;

const SELECT: &str = "SELECT id, user_id, tier, is_active, started_at::text, expires_at::text FROM subscriptions";

pub struct SubscriptionsDataProviderService {
    pool: PgPool,
}
impl SubscriptionsDataProviderService {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

#[async_trait]
impl SubscriptionsDataProvider for SubscriptionsDataProviderService {
    async fn create_subscription(&self, params: CreateSubscriptionParams) -> Result<DbSubscription, DataError> {
        sqlx::query_as::<_, DbSubscription>(
            "INSERT INTO subscriptions (user_id, tier, is_active, expires_at) \
             VALUES ($1, $2, true, $3) \
             RETURNING id, user_id, tier, is_active, started_at::text, expires_at::text"
        )
        .bind(params.user_id)
        .bind(&params.tier)
        .bind(&params.expires_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| { error!(error = %e, "create subscription failed"); DataError::from(e) })
    }

    async fn get_subscription(&self, id: SubscriptionId) -> Result<Option<DbSubscription>, DataError> {
        sqlx::query_as::<_, DbSubscription>(&format!("{SELECT} WHERE id = $1"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| { error!(id, error = %e, "get subscription failed"); DataError::from(e) })
    }

    async fn get_active_subscription_for_user(&self, user_id: i32) -> Result<Option<DbSubscription>, DataError> {
        sqlx::query_as::<_, DbSubscription>(&format!("{SELECT} WHERE user_id = $1 AND is_active = true ORDER BY started_at DESC LIMIT 1"))
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| { error!(error = %e, "get active subscription failed"); DataError::from(e) })
    }

    async fn deactivate_subscription(&self, id: SubscriptionId) -> Result<bool, DataError> {
        sqlx::query("UPDATE subscriptions SET is_active = false WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected() > 0)
            .map_err(|e| { error!(id, error = %e, "deactivate subscription failed"); DataError::from(e) })
    }

    async fn list_by_user(&self, user_id: i32) -> Result<Vec<DbSubscription>, DataError> {
        sqlx::query_as::<_, DbSubscription>(&format!("{SELECT} WHERE user_id = $1 ORDER BY started_at DESC"))
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| { error!(error = %e, "list subscriptions failed"); DataError::from(e) })
    }
}

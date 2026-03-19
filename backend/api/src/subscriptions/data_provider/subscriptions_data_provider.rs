use async_trait::async_trait;
use crate::domain::data_error::DataError;
use crate::domain::db::db_subscription::{DbSubscription, SubscriptionId};
use crate::subscriptions::create_subscription_params::CreateSubscriptionParams;

#[async_trait]
pub trait SubscriptionsDataProvider: Send + Sync {
    async fn create_subscription(&self, params: CreateSubscriptionParams) -> Result<DbSubscription, DataError>;
    async fn get_subscription(&self, id: SubscriptionId) -> Result<Option<DbSubscription>, DataError>;
    async fn get_active_subscription_for_user(&self, user_id: i64) -> Result<Option<DbSubscription>, DataError>;
    async fn deactivate_subscription(&self, id: SubscriptionId) -> Result<bool, DataError>;
    async fn list_by_user(&self, user_id: i64) -> Result<Vec<DbSubscription>, DataError>;
}

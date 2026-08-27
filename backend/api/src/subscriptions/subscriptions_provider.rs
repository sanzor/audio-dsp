use async_trait::async_trait;
use crate::domain::db::db_subscription::{DbSubscription, SubscriptionId};
use crate::domain::service_error::ServiceError;
use crate::subscriptions::create_subscription_params::CreateSubscriptionParams;

#[async_trait]
pub trait SubscriptionsProvider: Send + Sync {
    async fn create_subscription(&self, params: CreateSubscriptionParams) -> Result<DbSubscription, ServiceError>;
    async fn get_subscription(&self, id: SubscriptionId) -> Result<Option<DbSubscription>, ServiceError>;
    async fn get_active_subscription_for_user(&self, user_id: domain::domain_user::UserId) -> Result<Option<DbSubscription>, ServiceError>;
    async fn deactivate_subscription(&self, id: SubscriptionId) -> Result<bool, ServiceError>;
    async fn list_by_user(&self, user_id: domain::domain_user::UserId) -> Result<Vec<DbSubscription>, ServiceError>;
}

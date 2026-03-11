use async_trait::async_trait;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::service_error::ServiceError;
use crate::subscriptions::create_subscription_params::CreateSubscriptionParams;
use crate::subscriptions::update_subscription_params::UpdateSubscriptionParams;
use crate::subscriptions::DbSubscription;

#[async_trait]
pub trait SubscriptionsProvider: Send + Sync {
    async fn create_subscription(
        &self,
        params: CreateSubscriptionParams,
    ) -> Result<DbSubscription, ServiceError>;
    async fn update_subscription(
        &self,
        org_id: OrganizationId,
        params: UpdateSubscriptionParams,
    ) -> Result<Option<DbSubscription>, ServiceError>;
    async fn delete_subscription(&self, org_id: OrganizationId) -> Result<bool, ServiceError>;
    async fn get_subscription(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<DbSubscription>, ServiceError>;
    async fn list_subscriptions(&self) -> Result<Vec<DbSubscription>, ServiceError>;
    async fn find_by_stripe_subscription_id(
        &self,
        stripe_sub_id: &str,
    ) -> Result<Option<DbSubscription>, ServiceError>;
}

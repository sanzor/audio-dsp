use crate::domain::db::db_subscription::{DbSubscription, SubscriptionId};
use crate::domain::service_error::ServiceError;
use crate::subscriptions::create_subscription_params::CreateSubscriptionParams;
use crate::subscriptions::data_provider::subscriptions_data_provider::SubscriptionsDataProvider;
use crate::subscriptions::subscriptions_provider::SubscriptionsProvider;
use async_trait::async_trait;
use std::sync::Arc;

pub struct SubscriptionsProviderService {
    data_provider: Arc<dyn SubscriptionsDataProvider>,
}
impl SubscriptionsProviderService {
    pub fn new(data_provider: Arc<dyn SubscriptionsDataProvider>) -> Self {
        Self { data_provider }
    }
}
#[async_trait]
impl SubscriptionsProvider for SubscriptionsProviderService {
    async fn create_subscription(
        &self,
        params: CreateSubscriptionParams,
    ) -> Result<DbSubscription, ServiceError> {
        self.data_provider
            .create_subscription(params)
            .await
            .map_err(ServiceError::from)
    }
    async fn get_subscription(
        &self,
        id: SubscriptionId,
    ) -> Result<Option<DbSubscription>, ServiceError> {
        self.data_provider
            .get_subscription(id)
            .await
            .map_err(ServiceError::from)
    }
    async fn get_active_subscription_for_user(
        &self,
        user_id: domain::domain_user::UserId,
    ) -> Result<Option<DbSubscription>, ServiceError> {
        self.data_provider
            .get_active_subscription_for_user(user_id)
            .await
            .map_err(ServiceError::from)
    }
    async fn deactivate_subscription(&self, id: SubscriptionId) -> Result<bool, ServiceError> {
        self.data_provider
            .deactivate_subscription(id)
            .await
            .map_err(ServiceError::from)
    }
    async fn list_by_user(
        &self,
        user_id: domain::domain_user::UserId,
    ) -> Result<Vec<DbSubscription>, ServiceError> {
        self.data_provider
            .list_by_user(user_id)
            .await
            .map_err(ServiceError::from)
    }
}

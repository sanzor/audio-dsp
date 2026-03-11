use async_trait::async_trait;

use crate::domain::data_error::DataError;
use crate::domain::Tier;
use crate::{domain::db::db_organization::OrganizationId, subscriptions::DbSubscription};

#[async_trait]
pub trait SubscriptionsDataProvider: Send + Sync {
    async fn create_subscription(
        &self,
        org_id: OrganizationId,
        tier: Tier,
        stripe_subscription_id: Option<String>,
        status: String,
        current_period_end: Option<String>,
    ) -> Result<DbSubscription, DataError>;
    async fn update_subscription(
        &self,
        org_id: OrganizationId,
        tier: Option<Tier>,
        stripe_subscription_id: Option<String>,
        status: Option<String>,
        current_period_end: Option<String>,
    ) -> Result<Option<DbSubscription>, DataError>;
    async fn delete_subscription(&self, org_id: OrganizationId) -> Result<bool, DataError>;
    async fn get_subscription(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<DbSubscription>, DataError>;
    async fn list_subscriptions(&self) -> Result<Vec<DbSubscription>, DataError>;
    async fn find_by_stripe_subscription_id(
        &self,
        stripe_sub_id: &str,
    ) -> Result<Option<DbSubscription>, DataError>;
}

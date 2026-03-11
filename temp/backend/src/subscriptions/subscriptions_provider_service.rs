use std::sync::Arc;

use async_trait::async_trait;
use tracing::{error, info, warn};

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::service_error::ServiceError;
use crate::stripe::stripe_service::StripeService;
use crate::subscriptions::create_subscription_params::CreateSubscriptionParams;
use crate::subscriptions::data_provider::subscriptions_data_provider::SubscriptionsDataProvider;
use crate::subscriptions::update_subscription_params::UpdateSubscriptionParams;
use crate::subscriptions::{subscriptions_provider::SubscriptionsProvider, DbSubscription};
use crate::tier_configs::tier_configs_provider::TierConfigsProvider;

pub struct SubscriptionsProviderService {
    data_provider: Arc<dyn SubscriptionsDataProvider>,
    stripe_service: Arc<StripeService>,
    tier_configs_provider: Arc<dyn TierConfigsProvider>,
}

impl SubscriptionsProviderService {
    pub fn new(
        data_provider: Arc<dyn SubscriptionsDataProvider>,
        stripe_service: Arc<StripeService>,
        tier_configs_provider: Arc<dyn TierConfigsProvider>,
    ) -> Self {
        Self {
            data_provider,
            stripe_service,
            tier_configs_provider,
        }
    }

    /// Syncs a tier change to Stripe before the DB is updated.
    /// Returns early (Ok) if there is nothing to sync (no Stripe sub, same tier, no price configured).
    async fn sync_tier_to_stripe(
        &self,
        org_id: OrganizationId,
        new_tier: &crate::domain::Tier,
    ) -> Result<(), ServiceError> {
        let current = match self.data_provider.get_subscription(org_id).await? {
            Some(sub) if sub.tier != *new_tier => sub,
            _ => return Ok(()),
        };

        let stripe_sub_id = match current.stripe_subscription_id {
            Some(ref id) => id.clone(),
            None => return Ok(()),
        };

        let tier_str = new_tier.to_string();
        let config = match self.tier_configs_provider.get(&tier_str).await? {
            Some(c) => c,
            None => {
                warn!(org_id, tier = %new_tier, "tier config not found — skipping Stripe sync");
                return Ok(());
            }
        };

        let price_id = match config.stripe_price_id {
            Some(ref id) => id.clone(),
            None => {
                warn!(org_id, tier = %new_tier, "tier has no stripe_price_id — skipping Stripe sync");
                return Ok(());
            }
        };

        self.stripe_service
            .change_subscription_price(&stripe_sub_id, &price_id)
            .await
            .map_err(|e| {
                error!(org_id, error = %e, "failed to update subscription price in Stripe");
                ServiceError::from(e)
            })
    }
}

#[async_trait]
impl SubscriptionsProvider for SubscriptionsProviderService {
    async fn create_subscription(
        &self,
        params: CreateSubscriptionParams,
    ) -> Result<DbSubscription, ServiceError> {
        info!(org_id = params.org_id, "create subscription requested");
        self.data_provider
            .create_subscription(
                params.org_id,
                params.tier,
                params.stripe_subscription_id,
                params.status,
                params.current_period_end,
            )
            .await
            .map_err(ServiceError::from)
    }

    async fn update_subscription(
        &self,
        org_id: OrganizationId,
        params: UpdateSubscriptionParams,
    ) -> Result<Option<DbSubscription>, ServiceError> {
        info!(org_id, "update subscription requested");

        if let Some(ref new_tier) = params.tier {
            self.sync_tier_to_stripe(org_id, new_tier).await?;
        }

        self.data_provider
            .update_subscription(
                org_id,
                params.tier,
                params.stripe_subscription_id,
                params.status,
                params.current_period_end,
            )
            .await
            .map_err(ServiceError::from)
    }

    async fn delete_subscription(&self, org_id: OrganizationId) -> Result<bool, ServiceError> {
        info!(org_id, "delete subscription requested");

        if let Ok(Some(current)) = self.data_provider.get_subscription(org_id).await {
            if let Some(ref stripe_sub_id) = current.stripe_subscription_id {
                self.stripe_service
                    .cancel_subscription(stripe_sub_id)
                    .await
                    .map_err(|e| {
                        error!(org_id, error = %e, "failed to cancel subscription in Stripe");
                        ServiceError::from(e)
                    })?;
            }
        }

        self.data_provider
            .delete_subscription(org_id)
            .await
            .map_err(ServiceError::from)
    }

    async fn get_subscription(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<DbSubscription>, ServiceError> {
        info!(org_id, "get subscription requested");
        self.data_provider
            .get_subscription(org_id)
            .await
            .map_err(ServiceError::from)
    }

    async fn list_subscriptions(&self) -> Result<Vec<DbSubscription>, ServiceError> {
        info!("list subscriptions requested");
        self.data_provider
            .list_subscriptions()
            .await
            .map_err(ServiceError::from)
    }

    async fn find_by_stripe_subscription_id(
        &self,
        stripe_sub_id: &str,
    ) -> Result<Option<DbSubscription>, ServiceError> {
        self.data_provider
            .find_by_stripe_subscription_id(stripe_sub_id)
            .await
            .map_err(ServiceError::from)
    }
}

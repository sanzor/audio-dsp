use std::sync::Arc;

use async_trait::async_trait;
use deadpool_redis::redis::aio::MultiplexedConnection;
use deadpool_redis::redis::AsyncCommands;
use tracing::{error, warn};

use crate::billing_mode::billing_mode::BillingMode;
use crate::billing_mode::billing_mode_provider::BillingModeProvider;
use crate::billing_mode::data_provider::billing_mode_data_provider::BillingModeDataProvider;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::Tier;
use crate::rate_limits_config::rate_limit_config_input::RateLimitConfigInput;
use crate::rate_limits_config::rate_limits_config_provider::RateLimitsConfigProvider;
use crate::subscriptions::create_subscription_params::CreateSubscriptionParams;
use crate::subscriptions::subscriptions_provider::SubscriptionsProvider;
use crate::tier_configs::tier_configs_provider::TierConfigsProvider;

pub struct BillingModeProviderService {
    data_provider: Arc<dyn BillingModeDataProvider>,
    redis: MultiplexedConnection,
    subscriptions_provider: Arc<dyn SubscriptionsProvider>,
    tier_configs_provider: Arc<dyn TierConfigsProvider>,
    rate_limits_config_provider: Arc<dyn RateLimitsConfigProvider>,
}

impl BillingModeProviderService {
    pub fn new(
        data_provider: Arc<dyn BillingModeDataProvider>,
        redis: MultiplexedConnection,
        subscriptions_provider: Arc<dyn SubscriptionsProvider>,
        tier_configs_provider: Arc<dyn TierConfigsProvider>,
        rate_limits_config_provider: Arc<dyn RateLimitsConfigProvider>,
    ) -> Self {
        Self {
            data_provider,
            redis,
            subscriptions_provider,
            tier_configs_provider,
            rate_limits_config_provider,
        }
    }

    fn redis_key(org_id: OrganizationId) -> String {
        format!("organization:{}", org_id)
    }

    async fn init_free_subscription(&self, org_id: OrganizationId) {
        match self.subscriptions_provider.get_subscription(org_id).await {
            Ok(Some(_)) => return, // already has a subscription
            Err(e) => {
                warn!(org_id, error = %e, "could not check existing subscription, skipping free tier init");
                return;
            }
            Ok(None) => {}
        }

        if let Err(e) = self
            .subscriptions_provider
            .create_subscription(CreateSubscriptionParams {
                org_id,
                tier: Tier::Free,
                stripe_subscription_id: None,
                status: "active".to_string(),
                current_period_end: None,
            })
            .await
        {
            warn!(org_id, error = %e, "failed to create default free subscription");
            return;
        }

        match self.tier_configs_provider.get("free").await {
            Ok(Some(cfg)) => {
                let limit = cfg.token_limit as u64;
                let window_size_secs = cfg.window_size_secs as u64;
                if let Err(e) = self
                    .rate_limits_config_provider
                    .upsert_config(
                        org_id,
                        RateLimitConfigInput::TokenWindow { limit, window_size_secs },
                    )
                    .await
                {
                    warn!(org_id, error = %e, "failed to set free tier rate limit config");
                }
            }
            Ok(None) => warn!(org_id, "free tier config not found, rate limit not set"),
            Err(e) => warn!(org_id, error = %e, "failed to fetch free tier config"),
        }
    }
}

#[async_trait]
impl BillingModeProvider for BillingModeProviderService {
    async fn get_mode(&self, org_id: OrganizationId) -> Result<Option<BillingMode>, String> {
        let record = self.data_provider.get_mode(org_id).await?;
        record
            .map(|r| BillingMode::try_from(r.mode.as_str()))
            .transpose()
    }

    async fn upsert_mode(
        &self,
        org_id: OrganizationId,
        mode: BillingMode,
    ) -> Result<BillingMode, String> {
        let mode_str = mode.as_str();

        let key = Self::redis_key(org_id);
        let mut conn = self.redis.clone();
        conn.hset(&key, "billing_mode", mode_str)
            .await
            .map_err(|e| {
                error!(error = %e, "failed to update billing mode in redis");
                e.to_string()
            })?;

        self.data_provider.upsert_mode(org_id, mode_str).await?;

        if mode == BillingMode::Subscription {
            self.init_free_subscription(org_id).await;
        }

        Ok(mode)
    }
}

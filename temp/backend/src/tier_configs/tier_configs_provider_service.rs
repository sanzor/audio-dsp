use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::db::db_tier_config::DbTierConfig;
use crate::domain::service_error::ServiceError;
use crate::tier_configs::data_provider::tier_configs_data_provider::TierConfigsDataProvider;
use crate::tier_configs::data_provider::tier_configs_data_provider::UpdateTierConfigParams as DataUpdateParams;
use crate::tier_configs::tier_configs_provider::{TierConfigsProvider, UpdateTierConfigParams};

pub struct TierConfigsProviderService {
    data_provider: Arc<dyn TierConfigsDataProvider>,
}

impl TierConfigsProviderService {
    pub fn new(data_provider: Arc<dyn TierConfigsDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait]
impl TierConfigsProvider for TierConfigsProviderService {
    async fn list(&self) -> Result<Vec<DbTierConfig>, ServiceError> {
        self.data_provider.list().await.map_err(ServiceError::from)
    }

    async fn get(&self, tier: &str) -> Result<Option<DbTierConfig>, ServiceError> {
        self.data_provider
            .get(tier)
            .await
            .map_err(ServiceError::from)
    }

    async fn update(
        &self,
        tier: &str,
        params: UpdateTierConfigParams,
    ) -> Result<Option<DbTierConfig>, ServiceError> {
        let data_params = DataUpdateParams {
            token_limit: params.token_limit,
            window_size_secs: params.window_size_secs,
            stripe_price_id: params.stripe_price_id,
            price_cents: params.price_cents,
            currency: params.currency,
            display_name: params.display_name,
            description: params.description,
        };
        self.data_provider
            .update(tier, &data_params)
            .await
            .map_err(ServiceError::from)
    }
}

use crate::domain::db::db_tier_config::DbTierConfig;
use crate::domain::service_error::ServiceError;
use crate::domain::tier::Tier;
use crate::tier_configs::data_provider::tier_configs_data_provider::TierConfigsDataProvider;
use crate::tier_configs::tier_configs_provider::TierConfigsProvider;
use crate::tier_configs::update_tier_config_params::UpdateTierConfigParams;
use async_trait::async_trait;
use std::sync::Arc;

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
    async fn get_tier_config(&self, tier: &Tier) -> Result<Option<DbTierConfig>, ServiceError> {
        self.data_provider
            .get_tier_config(tier)
            .await
            .map_err(ServiceError::from)
    }
    async fn update_tier_config(
        &self,
        tier: &Tier,
        params: UpdateTierConfigParams,
    ) -> Result<Option<DbTierConfig>, ServiceError> {
        self.data_provider
            .update_tier_config(tier, params)
            .await
            .map_err(ServiceError::from)
    }
    async fn list_tier_configs(&self) -> Result<Vec<DbTierConfig>, ServiceError> {
        self.data_provider
            .list_tier_configs()
            .await
            .map_err(ServiceError::from)
    }
}

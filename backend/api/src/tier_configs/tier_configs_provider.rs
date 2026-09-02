use crate::domain::db::db_tier_config::DbTierConfig;
use crate::domain::service_error::ServiceError;
use crate::domain::tier::Tier;
use crate::tier_configs::update_tier_config_params::UpdateTierConfigParams;
use async_trait::async_trait;

#[async_trait]
pub trait TierConfigsProvider: Send + Sync {
    async fn get_tier_config(&self, tier: &Tier) -> Result<Option<DbTierConfig>, ServiceError>;
    async fn update_tier_config(
        &self,
        tier: &Tier,
        params: UpdateTierConfigParams,
    ) -> Result<Option<DbTierConfig>, ServiceError>;
    async fn list_tier_configs(&self) -> Result<Vec<DbTierConfig>, ServiceError>;
}

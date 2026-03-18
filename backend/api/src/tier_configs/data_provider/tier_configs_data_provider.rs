use async_trait::async_trait;
use crate::domain::data_error::DataError;
use crate::domain::db::db_tier_config::DbTierConfig;
use crate::domain::tier::Tier;
use crate::tier_configs::update_tier_config_params::UpdateTierConfigParams;

#[async_trait]
pub trait TierConfigsDataProvider: Send + Sync {
    async fn get_tier_config(&self, tier: &Tier) -> Result<Option<DbTierConfig>, DataError>;
    async fn update_tier_config(&self, tier: &Tier, params: UpdateTierConfigParams) -> Result<Option<DbTierConfig>, DataError>;
    async fn list_tier_configs(&self) -> Result<Vec<DbTierConfig>, DataError>;
}

use async_trait::async_trait;

use crate::domain::data_error::DataError;
use crate::domain::db::db_tier_config::DbTierConfig;

pub struct UpdateTierConfigParams {
    pub token_limit: Option<i64>,
    pub window_size_secs: Option<i64>,
    pub stripe_price_id: Option<String>,
    pub price_cents: Option<i64>,
    pub currency: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

#[async_trait]
pub trait TierConfigsDataProvider: Send + Sync {
    async fn list(&self) -> Result<Vec<DbTierConfig>, DataError>;
    async fn get(&self, tier: &str) -> Result<Option<DbTierConfig>, DataError>;
    async fn update(
        &self,
        tier: &str,
        params: &UpdateTierConfigParams,
    ) -> Result<Option<DbTierConfig>, DataError>;
}

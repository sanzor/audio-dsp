use async_trait::async_trait;

use crate::domain::db::db_tier_config::DbTierConfig;
use crate::domain::service_error::ServiceError;

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
pub trait TierConfigsProvider: Send + Sync {
    async fn list(&self) -> Result<Vec<DbTierConfig>, ServiceError>;
    async fn get(&self, tier: &str) -> Result<Option<DbTierConfig>, ServiceError>;
    async fn update(
        &self,
        tier: &str,
        params: UpdateTierConfigParams,
    ) -> Result<Option<DbTierConfig>, ServiceError>;
}

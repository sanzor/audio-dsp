use sqlx::FromRow;

use crate::domain::Tier;

#[derive(Clone, Debug, FromRow)]
pub struct DbTierConfig {
    pub tier: Tier,
    pub token_limit: i64,
    pub window_size_secs: i64,
    pub stripe_price_id: Option<String>,
    pub price_cents: i64,
    pub currency: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;

use crate::domain::data_error::DataError;
use crate::domain::db::db_tier_config::DbTierConfig;
use crate::tier_configs::data_provider::tier_configs_data_provider::TierConfigsDataProvider;
use crate::tier_configs::data_provider::tier_configs_data_provider::UpdateTierConfigParams;

pub struct TierConfigsDataProviderService {
    pool: PgPool,
}

impl TierConfigsDataProviderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TierConfigsDataProvider for TierConfigsDataProviderService {
    async fn list(&self) -> Result<Vec<DbTierConfig>, DataError> {
        sqlx::query_as::<_, DbTierConfig>(
            "SELECT tier, token_limit, window_size_secs, stripe_price_id, price_cents, \
             currency, display_name, description, updated_at \
             FROM tier_configs ORDER BY tier",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "list tier_configs failed");
            DataError::from(e)
        })
    }

    async fn get(&self, tier: &str) -> Result<Option<DbTierConfig>, DataError> {
        sqlx::query_as::<_, DbTierConfig>(
            "SELECT tier, token_limit, window_size_secs, stripe_price_id, price_cents, \
             currency, display_name, description, updated_at \
             FROM tier_configs WHERE tier = $1",
        )
        .bind(tier)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "get tier_config failed");
            DataError::from(e)
        })
    }

    async fn update(
        &self,
        tier: &str,
        params: &UpdateTierConfigParams,
    ) -> Result<Option<DbTierConfig>, DataError> {
        sqlx::query_as::<_, DbTierConfig>(
            "UPDATE tier_configs SET
               token_limit      = COALESCE($2, token_limit),
               window_size_secs = COALESCE($3, window_size_secs),
               stripe_price_id  = COALESCE($4, stripe_price_id),
               price_cents      = COALESCE($5, price_cents),
               currency         = COALESCE($6, currency),
               display_name     = COALESCE($7, display_name),
               description      = COALESCE($8, description),
               updated_at       = NOW()
             WHERE tier = $1
             RETURNING tier, token_limit, window_size_secs, stripe_price_id, price_cents,
                       currency, display_name, description, updated_at",
        )
        .bind(tier)
        .bind(params.token_limit)
        .bind(params.window_size_secs)
        .bind(params.stripe_price_id.as_deref())
        .bind(params.price_cents)
        .bind(params.currency.as_deref())
        .bind(params.display_name.as_deref())
        .bind(params.description.as_deref())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!(error = %e, "update tier_config failed");
            DataError::from(e)
        })
    }
}

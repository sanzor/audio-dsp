use std::sync::Arc;

use async_trait::async_trait;
use deadpool_redis::redis::aio::MultiplexedConnection;
use deadpool_redis::redis::AsyncCommands;
use rust_shared::rate_limit_config::RateLimitConfig;
use tracing::error;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_rate_limit_config::DbRateLimitConfig;
use crate::rate_limits_config::data_provider::rate_limits_data_provider::RateLimitsDataProvider;
use crate::rate_limits_config::rate_limit_config_input::RateLimitConfigInput;
use crate::rate_limits_config::rate_limit_config_response::RateLimitConfigResponse;
use crate::rate_limits_config::rate_limits_config_provider::RateLimitsConfigProvider;

pub struct RateLimitsProviderService {
    data_provider: Arc<dyn RateLimitsDataProvider>,
    redis: MultiplexedConnection,
}

impl RateLimitsProviderService {
    pub fn new(
        data_provider: Arc<dyn RateLimitsDataProvider>,
        redis: MultiplexedConnection,
    ) -> Self {
        Self {
            data_provider,
            redis,
        }
    }

    fn redis_key(org_id: OrganizationId) -> String {
        format!("organization:{}", org_id)
    }

    // Read a redis hash field that is stored as a bulk string and parse it into u64.
    // Missing / invalid values become 0 (keeps refill path resilient).
}

fn parse_stored_config(record: DbRateLimitConfig) -> Result<RateLimitConfig, String> {
    serde_json::from_value(record.config).map_err(|e| e.to_string())
}

#[async_trait]
impl RateLimitsConfigProvider for RateLimitsProviderService {
    async fn get_config(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<RateLimitConfigResponse>, String> {
        let stored = self.data_provider.get_config(org_id).await?;
        let parsed =
            stored
                .map(parse_stored_config)
                .transpose()?
                .map(|config| RateLimitConfigResponse {
                    config: config.into(),
                });
        Ok(parsed)
    }

    async fn upsert_config(
        &self,
        org_id: OrganizationId,
        config: RateLimitConfigInput,
    ) -> Result<RateLimitConfigResponse, String> {
        let stored: RateLimitConfig = config.clone().into();
        let config_json = serde_json::to_value(&stored).map_err(|e| e.to_string())?;
        let config_string = serde_json::to_string(&stored).map_err(|e| e.to_string())?;

        let key = Self::redis_key(org_id);
        let mut conn = self.redis.clone();
        conn.hset(&key, "config", config_string)
            .await
            .map_err(|e| {
                error!(error = %e, "failed to update rate limit config in redis");
                e.to_string()
            })?;

        let stored_db = self
            .data_provider
            .upsert_config(org_id, config_json)
            .await?;

        let stored_config = parse_stored_config(stored_db)?;
        Ok(RateLimitConfigResponse {
            config: stored_config.into(),
        })
    }
}

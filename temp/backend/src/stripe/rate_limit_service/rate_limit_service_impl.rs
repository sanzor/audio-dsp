use async_trait::async_trait;
use deadpool_redis::redis::aio::MultiplexedConnection;
use deadpool_redis::redis::AsyncCommands;
use std::sync::Arc;
use tracing::warn;

use crate::domain::db::db_organization::OrganizationId;
use crate::rate_limits_config::data_provider::rate_limits_data_provider::RateLimitsDataProvider;
use crate::rate_limits_config::{RateLimitRefillInput, RateLimitRefillResponse};
use crate::stripe::rate_limit_service::rate_limit_service::RateLimitService;
use crate::usage::data_provider::usage_data_provider::UsageDataProvider;

pub struct RateLimitServiceImpl {
    rate_limits_data_provider: Arc<dyn RateLimitsDataProvider>,
    usage_data_provider: Arc<dyn UsageDataProvider>,
    redis: MultiplexedConnection,
}

impl RateLimitServiceImpl {
    pub fn new(
        rate_limits_data_provider: Arc<dyn RateLimitsDataProvider>,
        usage_data_provider: Arc<dyn UsageDataProvider>,
        redis: MultiplexedConnection,
    ) -> Self {
        Self {
            rate_limits_data_provider,
            usage_data_provider,
            redis,
        }
    }

    fn redis_key(org_id: OrganizationId) -> String {
        format!("organization:{}", org_id)
    }

    async fn hget_u64(
        conn: &mut MultiplexedConnection,
        key: &str,
        field: &str,
    ) -> Result<u64, String> {
        let raw: Option<String> = conn.hget(key, field).await.map_err(|e| e.to_string())?;
        let value = raw.as_deref().unwrap_or("0").parse::<u64>().unwrap_or(0);
        Ok(value)
    }
}

#[async_trait]
impl RateLimitService for RateLimitServiceImpl {
    async fn apply_refill(
        &self,
        org_id: OrganizationId,
        action: RateLimitRefillInput,
    ) -> Result<RateLimitRefillResponse, String> {
        let key = Self::redis_key(org_id);
        let mut conn = self.redis.clone();

        let response = match action {
            RateLimitRefillInput::BucketAdd { amount } => {
                let delta = i64::try_from(amount).map_err(|_| "amount too large".to_string())?;

                let new_value: i64 = conn
                    .hincr(&key, "tokens", delta)
                    .await
                    .map_err(|e| e.to_string())?;

                let _ = self
                    .rate_limits_data_provider
                    .create_event(org_id, "bucket_add", Some(delta))
                    .await?;

                if let Err(e) = self.usage_data_provider.update_bucket_tokens(org_id, new_value).await {
                    warn!(org_id, error = %e, "failed to update bucket_tokens in DB after BucketAdd");
                }

                RateLimitRefillResponse {
                    action: RateLimitRefillInput::BucketAdd { amount },
                    current_value: Some(new_value.max(0) as u64),
                    refunded: None,
                    window_start: None,
                }
            }

            RateLimitRefillInput::BucketReset => {
                conn.hset::<_, _, _, ()>(&key, "tokens", 0_i64)
                    .await
                    .map_err(|e| e.to_string())?;

                self.rate_limits_data_provider
                    .create_event(org_id, "bucket_reset", None)
                    .await?;

                if let Err(e) = self.usage_data_provider.update_bucket_tokens(org_id, 0).await {
                    warn!(org_id, error = %e, "failed to update bucket_tokens in DB after BucketReset");
                }

                RateLimitRefillResponse {
                    action: RateLimitRefillInput::BucketReset,
                    current_value: Some(0),
                    refunded: None,
                    window_start: None,
                }
            }

            RateLimitRefillInput::WindowReset => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                conn.hset::<_, _, _, ()>(&key, "window_counter", 0_i64)
                    .await
                    .map_err(|e| e.to_string())?;
                conn.hset::<_, _, _, ()>(&key, "window_start", now)
                    .await
                    .map_err(|e| e.to_string())?;

                self.rate_limits_data_provider
                    .create_event(org_id, "window_reset", None)
                    .await?;

                RateLimitRefillResponse {
                    action: RateLimitRefillInput::WindowReset,
                    current_value: Some(0),
                    refunded: None,
                    window_start: Some(now),
                }
            }

            RateLimitRefillInput::WindowRefund { amount } => {
                // Read as String -> parse into u64 to avoid Option<String> vs Option<u64> decoding issues.
                let current = Self::hget_u64(&mut conn, &key, "window_counter").await?;

                let new_value = current.saturating_sub(amount);

                conn.hset::<_, _, _, ()>(&key, "window_counter", new_value as i64)
                    .await
                    .map_err(|e| e.to_string())?;

                let refunded = current - new_value;

                let amount_i64 =
                    i64::try_from(amount).map_err(|_| "amount too large".to_string())?;
                self.rate_limits_data_provider
                    .create_event(org_id, "window_refund", Some(amount_i64))
                    .await?;

                RateLimitRefillResponse {
                    action: RateLimitRefillInput::WindowRefund { amount },
                    current_value: Some(new_value),
                    refunded: Some(refunded),
                    window_start: None,
                }
            }
        };

        Ok(response)
    }
}

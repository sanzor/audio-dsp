use async_trait::async_trait;
use rust_shared::rate_limit_config::RateLimitConfig;

use crate::{
    domain::db::db_organization::OrganizationId,
    rate_limits_config::{
        rate_limit_config_input::RateLimitConfigInput,
        rate_limit_config_response::RateLimitConfigResponse,
    },
};

impl From<RateLimitConfigInput> for RateLimitConfig {
    fn from(value: RateLimitConfigInput) -> Self {
        match value {
            RateLimitConfigInput::TokenBucket { limit } => RateLimitConfig::TokenBucket { limit },
            RateLimitConfigInput::TokenWindow {
                window_size_secs,
                limit,
            } => RateLimitConfig::TokenWindow {
                window_size_secs,
                limit,
            },
            RateLimitConfigInput::Default => RateLimitConfig::Default,
        }
    }
}

impl From<RateLimitConfig> for RateLimitConfigInput {
    fn from(value: RateLimitConfig) -> Self {
        match value {
            RateLimitConfig::TokenBucket { limit } => RateLimitConfigInput::TokenBucket { limit },
            RateLimitConfig::TokenWindow {
                window_size_secs,
                limit,
            } => RateLimitConfigInput::TokenWindow {
                window_size_secs,
                limit,
            },
            RateLimitConfig::Default => RateLimitConfigInput::Default,
        }
    }
}

#[async_trait]
pub trait RateLimitsConfigProvider: Send + Sync {
    async fn get_config(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<RateLimitConfigResponse>, String>;
    async fn upsert_config(
        &self,
        org_id: OrganizationId,
        config: RateLimitConfigInput,
    ) -> Result<RateLimitConfigResponse, String>;
}

use rust_shared::rate_limit_config::RateLimitConfig;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RateLimitConfigInput {
    TokenBucket { limit: u64 },
    TokenWindow { window_size_secs: u64, limit: u64 },
    Default,
}

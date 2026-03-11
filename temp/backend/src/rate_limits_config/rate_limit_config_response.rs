use rust_shared::rate_limit_config::RateLimitConfig;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RateLimitConfigResponse {
    pub config: RateLimitConfig,
}

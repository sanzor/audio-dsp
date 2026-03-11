use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::rate_limits_config::rate_limit_refill_input::RateLimitRefillInput;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RateLimitRefillResponse {
    pub action: RateLimitRefillInput,
    pub current_value: Option<u64>,
    pub refunded: Option<u64>,
    pub window_start: Option<u64>,
}

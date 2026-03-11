use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::rate_limits_config::rate_limit_refill_input::RateLimitRefillInput;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ApplyRateLimitRefillInput {
    pub action: RateLimitRefillInput,
}

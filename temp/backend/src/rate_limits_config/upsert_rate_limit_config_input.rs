use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::rate_limits_config::rate_limit_config_input::RateLimitConfigInput;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpsertRateLimitConfigInput {
    pub config: RateLimitConfigInput,
}

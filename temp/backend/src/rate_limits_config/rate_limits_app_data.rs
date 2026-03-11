use std::sync::Arc;

use crate::rate_limits_config::rate_limits_config_provider::RateLimitsConfigProvider;

pub struct RateLimitsAppData {
    pub rate_limits_provider: Arc<dyn RateLimitsConfigProvider>,
}

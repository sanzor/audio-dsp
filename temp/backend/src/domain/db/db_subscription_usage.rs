use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::rate_limit_config_type::RateLimitConfigType;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DbSubscriptionUsage {
    pub org_id: OrganizationId,
    pub rate_limit_type: RateLimitConfigType,
    /// Configured token limit
    pub limit: Option<i64>,
    /// Window duration in seconds (TokenWindow only)
    pub window_size_secs: Option<i64>,
    /// Current tokens remaining in bucket (TokenBucket only)
    pub bucket_tokens: Option<i64>,
    /// Tokens consumed in the current window (TokenWindow only)
    pub window_counter: Option<i64>,
    /// Unix timestamp (seconds) when the current window started (TokenWindow only)
    pub window_start: Option<i64>,
}

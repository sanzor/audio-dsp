use async_trait::async_trait;

use crate::domain::db::db_organization::OrganizationId;
use crate::rate_limits_config::{RateLimitRefillInput, RateLimitRefillResponse};

#[async_trait]
pub trait RateLimitService: Send + Sync {
    async fn apply_refill(
        &self,
        org_id: OrganizationId,
        action: RateLimitRefillInput,
    ) -> Result<RateLimitRefillResponse, String>;
}

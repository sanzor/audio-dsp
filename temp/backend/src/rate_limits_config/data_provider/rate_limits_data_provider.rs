use async_trait::async_trait;

use crate::domain::db::{db_organization::OrganizationId, db_rate_limit_config::DbRateLimitConfig};

#[async_trait]
pub trait RateLimitsDataProvider: Send + Sync {
    async fn get_config(&self, org_id: OrganizationId)
        -> Result<Option<DbRateLimitConfig>, String>;
    async fn upsert_config(
        &self,
        org_id: OrganizationId,
        config: serde_json::Value,
    ) -> Result<DbRateLimitConfig, String>;
    async fn create_event(
        &self,
        org_id: OrganizationId,
        event_type: &str,
        tokens: Option<i64>,
    ) -> Result<(), String>;
}

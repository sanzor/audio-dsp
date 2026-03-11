use async_trait::async_trait;

use crate::domain::data_error::DataError;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::usage_snapshot::UsageSnapshot;
use crate::jobs::usage_worker::usage_worker_processor::RateLimitUsageSnapshot;

#[async_trait]
pub trait UsageDataProvider: Send + Sync {
    async fn get_usage(&self, org_id: OrganizationId)
        -> Result<Option<UsageSnapshot>, DataError>;

    async fn upsert_subscription_usage(
        &self,
        org_id: OrganizationId,
        snapshot: RateLimitUsageSnapshot,
    ) -> Result<(), String>;

    async fn update_bucket_tokens(
        &self,
        org_id: OrganizationId,
        bucket_tokens: i64,
    ) -> Result<(), String>;
}

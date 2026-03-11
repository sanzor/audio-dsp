use async_trait::async_trait;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::usage_snapshot::UsageSnapshot;
use crate::domain::service_error::ServiceError;

#[async_trait]
pub trait UsageProvider: Send + Sync {
    async fn get_usage(
        &self,
        org_id: OrganizationId,
    ) -> Result<Option<UsageSnapshot>, ServiceError>;
}

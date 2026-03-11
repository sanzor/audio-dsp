use async_trait::async_trait;

use crate::domain::db::db_billing_mode::DbBillingModeConfig;
use crate::domain::db::db_organization::OrganizationId;

#[async_trait]
pub trait BillingModeDataProvider: Send + Sync {
    async fn get_mode(&self, org_id: OrganizationId)
        -> Result<Option<DbBillingModeConfig>, String>;

    async fn upsert_mode(
        &self,
        org_id: OrganizationId,
        mode: &str,
    ) -> Result<DbBillingModeConfig, String>;
}

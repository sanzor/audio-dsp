use async_trait::async_trait;

use crate::billing_mode::billing_mode::BillingMode;
use crate::domain::db::db_organization::OrganizationId;

#[async_trait]
pub trait BillingModeProvider: Send + Sync {
    async fn get_mode(&self, org_id: OrganizationId) -> Result<Option<BillingMode>, String>;
    async fn upsert_mode(
        &self,
        org_id: OrganizationId,
        mode: BillingMode,
    ) -> Result<BillingMode, String>;
}

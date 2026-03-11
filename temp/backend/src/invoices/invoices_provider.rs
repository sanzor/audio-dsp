use async_trait::async_trait;

use crate::domain::db::db_invoice::{DbInvoice, InvoiceId};
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::service_error::ServiceError;
use crate::invoices::create_invoice_params::CreateInvoiceParams;

#[async_trait]
pub trait InvoicesProvider: Send + Sync {
    async fn create_invoice(&self, params: CreateInvoiceParams) -> Result<DbInvoice, ServiceError>;
    async fn get_invoice(&self, id: InvoiceId) -> Result<Option<DbInvoice>, ServiceError>;
    async fn delete_invoice(&self, id: InvoiceId) -> Result<bool, ServiceError>;

    /// Tenant: paginated list scoped to one org.
    async fn list_by_org_paginated(
        &self,
        org_id: OrganizationId,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbInvoice>, i64), ServiceError>;

    /// Admin: paginated list, optionally filtered by org.
    async fn list_all_paginated(
        &self,
        org_id: Option<OrganizationId>,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbInvoice>, i64), ServiceError>;
}

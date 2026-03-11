use async_trait::async_trait;

use crate::domain::data_error::DataError;
use crate::domain::db::db_invoice::{DbInvoice, InvoiceId};
use crate::domain::db::db_organization::OrganizationId;
use crate::invoices::create_invoice_params::CreateInvoiceParams;

#[async_trait]
pub trait InvoicesDataProvider: Send + Sync {
    async fn create_invoice(&self, params: CreateInvoiceParams) -> Result<DbInvoice, DataError>;
    async fn get_invoice(&self, id: InvoiceId) -> Result<Option<DbInvoice>, DataError>;
    async fn delete_invoice(&self, id: InvoiceId) -> Result<bool, DataError>;

    /// Tenant: list invoices for a specific org with pagination.
    async fn list_by_org_paginated(
        &self,
        org_id: OrganizationId,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbInvoice>, i64), DataError>;

    /// Admin: list all invoices with an optional org filter and pagination.
    async fn list_all_paginated(
        &self,
        org_id: Option<OrganizationId>,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbInvoice>, i64), DataError>;
}

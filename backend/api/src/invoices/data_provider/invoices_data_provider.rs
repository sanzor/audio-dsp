use crate::domain::data_error::DataError;
use crate::domain::db::db_invoice::{DbInvoice, InvoiceId};
use crate::invoices::create_invoice_params::CreateInvoiceParams;
use async_trait::async_trait;

#[async_trait]
pub trait InvoicesDataProvider: Send + Sync {
    async fn create_invoice(&self, params: CreateInvoiceParams) -> Result<DbInvoice, DataError>;
    async fn get_invoice(&self, id: InvoiceId) -> Result<Option<DbInvoice>, DataError>;
    async fn delete_invoice(&self, id: InvoiceId) -> Result<bool, DataError>;
    async fn list_by_user_paginated(
        &self,
        user_id: domain::domain_user::UserId,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbInvoice>, i64), DataError>;
    async fn list_all_paginated(
        &self,
        user_id: Option<domain::domain_user::UserId>,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DbInvoice>, i64), DataError>;
}

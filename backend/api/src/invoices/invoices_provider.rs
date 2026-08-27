use async_trait::async_trait;
use crate::domain::db::db_invoice::{DbInvoice, InvoiceId};
use crate::domain::service_error::ServiceError;
use crate::invoices::create_invoice_params::CreateInvoiceParams;

#[async_trait]
pub trait InvoicesProvider: Send + Sync {
    async fn create_invoice(&self, params: CreateInvoiceParams) -> Result<DbInvoice, ServiceError>;
    async fn get_invoice(&self, id: InvoiceId) -> Result<Option<DbInvoice>, ServiceError>;
    async fn delete_invoice(&self, id: InvoiceId) -> Result<bool, ServiceError>;
    async fn list_by_user_paginated(&self, user_id: domain::domain_user::UserId, offset: i64, limit: i64) -> Result<(Vec<DbInvoice>, i64), ServiceError>;
    async fn list_all_paginated(&self, user_id: Option<domain::domain_user::UserId>, offset: i64, limit: i64) -> Result<(Vec<DbInvoice>, i64), ServiceError>;
}

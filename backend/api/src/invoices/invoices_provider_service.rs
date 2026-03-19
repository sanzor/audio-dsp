use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::db::db_invoice::{DbInvoice, InvoiceId};
use crate::domain::service_error::ServiceError;
use crate::invoices::create_invoice_params::CreateInvoiceParams;
use crate::invoices::data_provider::invoices_data_provider::InvoicesDataProvider;
use crate::invoices::invoices_provider::InvoicesProvider;

pub struct InvoicesProviderService {
    data_provider: Arc<dyn InvoicesDataProvider>,
}

impl InvoicesProviderService {
    pub fn new(data_provider: Arc<dyn InvoicesDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait]
impl InvoicesProvider for InvoicesProviderService {
    async fn create_invoice(&self, params: CreateInvoiceParams) -> Result<DbInvoice, ServiceError> {
        self.data_provider.create_invoice(params).await.map_err(ServiceError::from)
    }
    async fn get_invoice(&self, id: InvoiceId) -> Result<Option<DbInvoice>, ServiceError> {
        self.data_provider.get_invoice(id).await.map_err(ServiceError::from)
    }
    async fn delete_invoice(&self, id: InvoiceId) -> Result<bool, ServiceError> {
        self.data_provider.delete_invoice(id).await.map_err(ServiceError::from)
    }
    async fn list_by_user_paginated(&self, user_id: i64, offset: i64, limit: i64) -> Result<(Vec<DbInvoice>, i64), ServiceError> {
        self.data_provider.list_by_user_paginated(user_id, offset, limit).await.map_err(ServiceError::from)
    }
    async fn list_all_paginated(&self, user_id: Option<i64>, offset: i64, limit: i64) -> Result<(Vec<DbInvoice>, i64), ServiceError> {
        self.data_provider.list_all_paginated(user_id, offset, limit).await.map_err(ServiceError::from)
    }
}

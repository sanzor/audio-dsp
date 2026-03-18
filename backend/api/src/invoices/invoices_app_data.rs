use std::sync::Arc;
use crate::invoices::invoices_provider::InvoicesProvider;

#[derive(Clone)]
pub struct InvoicesAppData {
    pub invoices_provider: Arc<dyn InvoicesProvider>,
}

use crate::invoices::invoices_provider::InvoicesProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct InvoicesAppData {
    pub invoices_provider: Arc<dyn InvoicesProvider>,
}

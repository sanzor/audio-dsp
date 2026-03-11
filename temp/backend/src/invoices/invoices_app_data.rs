use std::sync::Arc;

use crate::invoices::invoices_provider::InvoicesProvider;

pub struct InvoicesAppData {
    pub invoices_provider: Arc<dyn InvoicesProvider>,
}

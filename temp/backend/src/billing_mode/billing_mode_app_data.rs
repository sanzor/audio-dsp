use std::sync::Arc;

use crate::billing_mode::billing_mode_provider::BillingModeProvider;

pub struct BillingModeAppData {
    pub billing_mode_provider: Arc<dyn BillingModeProvider>,
}

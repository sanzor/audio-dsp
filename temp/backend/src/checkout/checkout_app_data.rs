use std::sync::Arc;

use crate::checkout::checkout_provider::CheckoutProvider;

pub struct CheckoutAppData {
    pub checkout_provider: Arc<dyn CheckoutProvider>,
}

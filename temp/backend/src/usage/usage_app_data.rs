use std::sync::Arc;

use crate::usage::usage_provider::UsageProvider;

#[derive(Clone)]
pub struct UsageAppData {
    pub usage_provider: Arc<dyn UsageProvider>,
}

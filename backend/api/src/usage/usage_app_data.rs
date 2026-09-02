use crate::usage::usage_provider::UsageProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct UsageAppData {
    pub usage_provider: Arc<dyn UsageProvider>,
}

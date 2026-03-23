use std::sync::Arc;
use super::transforms_provider::TransformsProvider;

#[derive(Clone)]
pub struct TransformsAppData {
    pub transforms_service: Arc<dyn TransformsProvider>,
}

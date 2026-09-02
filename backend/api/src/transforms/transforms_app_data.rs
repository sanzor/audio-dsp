use super::transforms_provider::TransformsProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct TransformsAppData {
    pub transforms_service: Arc<dyn TransformsProvider>,
}

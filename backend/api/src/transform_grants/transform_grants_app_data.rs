use std::sync::Arc;

use super::transform_grants_provider::TransformGrantsProvider;

#[derive(Clone)]
pub struct TransformGrantsAppData {
    pub transform_grants_service: Arc<dyn TransformGrantsProvider>,
}

use std::sync::Arc;
use super::transform_drafts_provider::TransformDraftsProvider;

#[derive(Clone)]
pub struct TransformDraftsAppData {
    pub transform_drafts_service: Arc<dyn TransformDraftsProvider>,
}

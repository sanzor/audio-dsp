use super::transform_drafts_provider::TransformDraftsProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct TransformDraftsAppData {
    pub transform_drafts_service: Arc<dyn TransformDraftsProvider>,
}

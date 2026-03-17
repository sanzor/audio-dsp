use std::sync::Arc;

use super::tracks_provider::TracksProvider;

#[derive(Clone)]
pub struct TracksAppData {
    pub tracks_service: Arc<dyn TracksProvider>,
}

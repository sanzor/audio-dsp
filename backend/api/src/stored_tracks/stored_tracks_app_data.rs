use std::sync::Arc;

use crate::tracks::tracks_provider::TracksProvider;

#[derive(Clone)]
pub struct StoredTracksAppData {
    pub tracks_service: Arc<dyn TracksProvider>,
}

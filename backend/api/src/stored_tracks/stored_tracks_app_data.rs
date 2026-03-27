use std::sync::Arc;

use super::stored_tracks_provider::StoredTracksProvider;

#[derive(Clone)]
pub struct StoredTracksAppData {
    pub stored_tracks_service: Arc<dyn StoredTracksProvider>,
}

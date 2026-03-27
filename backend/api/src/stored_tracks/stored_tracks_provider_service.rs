use std::sync::Arc;

use domain::{db::db_track::TrackId, tracks::stored_track::StoredTrack};

use super::{
    data_provider::stored_tracks_data_provider::StoredTracksDataProvider,
    stored_tracks_provider::StoredTracksProvider,
};

pub struct StoredTracksProviderService {
    data: Arc<dyn StoredTracksDataProvider>,
}

impl StoredTracksProviderService {
    pub fn new(data: Arc<dyn StoredTracksDataProvider>) -> Self {
        Self { data }
    }
}

#[async_trait::async_trait]
impl StoredTracksProvider for StoredTracksProviderService {
    async fn get_stored_track(&self, track_id: &TrackId) -> Result<StoredTrack, String> {
        self.data.get_stored_track(track_id).await
    }
}

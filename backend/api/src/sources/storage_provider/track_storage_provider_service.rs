use std::sync::Arc;

use domain::{
    db::db_track::TrackId,
    tracks::track_bundle::TrackPayload,
};

use crate::stored_tracks::data_provider::stored_tracks_data_provider::StoredTracksDataProvider;

use super::track_storage_provider::TrackStorageProvider;

pub struct TrackStorageProviderService {
    data: Arc<dyn StoredTracksDataProvider>,
}

impl TrackStorageProviderService {
    pub fn new(data: Arc<dyn StoredTracksDataProvider>) -> Self {
        Self { data }
    }
}

#[async_trait::async_trait]
impl TrackStorageProvider for TrackStorageProviderService {
    async fn get_track_payload(&self, track_id: &TrackId) -> Result<TrackPayload, String> {
        let stored_track = self.data.get_stored_track(track_id).await?;
        Ok(TrackPayload {
            canonical_audio: stored_track.canonical_audio,
        })
    }

    async fn insert_track_payload(&self, track_id: &TrackId, payload: TrackPayload) -> Result<(), String> {
        self.data.insert_track_to_storage(track_id, payload.canonical_audio).await
    }
}

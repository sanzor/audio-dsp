use domain::db::db_track::TrackId;
use domain::tracks::stored_track::StoredTrack;

#[async_trait::async_trait]
pub trait StoredTracksDataProvider: Send + Sync {
    async fn get_stored_track(&self, track_id: &TrackId) -> Result<StoredTrack, String>;
    async fn insert_track_to_storage(&self, track_id: &TrackId, canonical_audio: Vec<u8>) -> Result<(), String>;
}

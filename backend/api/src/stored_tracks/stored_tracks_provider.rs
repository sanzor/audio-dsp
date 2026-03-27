use domain::{db::db_track::TrackId, tracks::stored_track::StoredTrack};

#[async_trait::async_trait]
pub trait StoredTracksProvider: Send + Sync {
    async fn get_stored_track(&self, track_id: &TrackId) -> Result<StoredTrack, String>;
}

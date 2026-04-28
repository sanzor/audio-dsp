use domain::{
    db::db_track::TrackId,
    tracks::track_bundle::TrackPayload,
};

#[async_trait::async_trait]
pub trait TrackStorageProvider: Send + Sync {
    async fn get_track_payload(&self, track_id: &TrackId) -> Result<TrackPayload, String>;
    async fn insert_track_payload(&self, track_id: &TrackId, payload: TrackPayload) -> Result<(), String>;
}

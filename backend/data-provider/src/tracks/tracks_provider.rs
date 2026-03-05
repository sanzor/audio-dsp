use domain::{
    db::db_track::{DbTrack, TrackId},
    tracks::{raw_track::RawTrack, track_subtree::TrackSubtree},
    update_track_info_params::UpdateTrackInfoParams,
};

#[async_trait::async_trait]
pub trait TracksProvider: Send + Sync {
    async fn get_track(&self, track_id: &TrackId) -> Result<DbTrack, String>;
    async fn get_all_tracks(&self) -> Result<Vec<DbTrack>, String>;
    async fn delete_track(&self, track_id: &TrackId) -> Result<(), String>;
    async fn upsert_track(&self, track: RawTrack) -> Result<DbTrack, String>;
    async fn copy_track(&self, source_track_id: &TrackId, new_name: &str) -> Result<DbTrack, String>;
    async fn update_track_info(
        &self,
        track_id: &TrackId,
        params: UpdateTrackInfoParams,
    ) -> Result<DbTrack, String>;
    async fn get_track_subtree(&self, track_id: &TrackId) -> Result<TrackSubtree, String>;
}

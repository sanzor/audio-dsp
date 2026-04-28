use domain::{
    db::db_track::TrackId,
    raw_track::RawTrack,
    tracks::track_bundle::TrackBundle,
    track_meta::TrackMeta,
    update_track_info_params::UpdateTrackInfoParams,
};

#[async_trait::async_trait]
pub trait TracksProvider: Send + Sync {
    async fn get_track_meta(&self, track_id: &TrackId) -> Result<TrackMeta, String>;
    async fn get_track(&self, track_id: &TrackId) -> Result<TrackBundle, String>;
    async fn get_tracks(&self) -> Result<Vec<TrackBundle>, String>;
    async fn get_all_track_metas(&self) -> Result<Vec<TrackMeta>, String>;
    async fn insert_track(&self, track: RawTrack, project_id: i32) -> Result<TrackBundle, String>;
    async fn delete_track(&self, track_id: &TrackId) -> Result<(), String>;
    async fn copy_track(&self, track_id: &TrackId, copy_name: String) -> Result<TrackMeta, String>;
    async fn update_track_info(
        &self,
        track_id: &TrackId,
        params: UpdateTrackInfoParams,
    ) -> Result<TrackMeta, String>;
}

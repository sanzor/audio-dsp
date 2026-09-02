use domain::{
    db::db_track::TrackId, raw_track::RawTrack, track_meta::TrackMeta,
    tracks::track_bundle::TrackBundle, update_track_info_params::UpdateTrackInfoParams,
};

#[async_trait::async_trait]
pub trait TracksProvider: Send + Sync {
    async fn get_track_meta(
        &self,
        track_id: &TrackId,
        workspace_id: i32,
    ) -> Result<TrackMeta, String>;
    async fn get_track(&self, track_id: &TrackId, workspace_id: i32)
        -> Result<TrackBundle, String>;
    async fn get_tracks(&self, workspace_id: i32) -> Result<Vec<TrackBundle>, String>;
    async fn get_all_track_metas(&self, workspace_id: i32) -> Result<Vec<TrackMeta>, String>;
    async fn insert_track(&self, track: RawTrack, workspace_id: i32)
        -> Result<TrackBundle, String>;
    async fn delete_track(&self, track_id: &TrackId, workspace_id: i32) -> Result<(), String>;
    async fn copy_track(
        &self,
        track_id: &TrackId,
        copy_name: String,
        workspace_id: i32,
    ) -> Result<TrackMeta, String>;
    async fn update_track_info(
        &self,
        track_id: &TrackId,
        params: UpdateTrackInfoParams,
        workspace_id: i32,
    ) -> Result<TrackMeta, String>;
}

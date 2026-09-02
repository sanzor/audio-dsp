use domain::{
    db::db_track::{DbTrack, TrackId},
    raw_track::TrackInfo,
    update_track_info_params::UpdateTrackInfoParams,
};

#[async_trait::async_trait]
pub trait TracksDataProvider: Send + Sync {
    async fn get_track(&self, track_id: &TrackId, workspace_id: i32) -> Result<DbTrack, String>;
    async fn get_all_track_metas(&self, workspace_id: i32) -> Result<Vec<DbTrack>, String>;
    async fn delete_track(&self, track_id: &TrackId, workspace_id: i32) -> Result<(), String>;
    async fn insert_track(
        &self,
        track_info: TrackInfo,
        workspace_id: i32,
    ) -> Result<DbTrack, String>;
    async fn copy_track(
        &self,
        source_track_id: &TrackId,
        new_name: &str,
        workspace_id: i32,
    ) -> Result<DbTrack, String>;
    async fn update_track_info(
        &self,
        track_id: &TrackId,
        params: UpdateTrackInfoParams,
        workspace_id: i32,
    ) -> Result<DbTrack, String>;
}

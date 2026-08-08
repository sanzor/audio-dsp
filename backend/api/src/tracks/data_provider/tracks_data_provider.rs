use domain::{
    db::db_track::{DbTrack, TrackId},
    raw_track::TrackInfo,
    update_track_info_params::UpdateTrackInfoParams,
};

#[async_trait::async_trait]
pub trait TracksDataProvider: Send + Sync {
    async fn get_track(&self, track_id: &TrackId, project_id: i32) -> Result<DbTrack, String>;
    async fn get_all_track_metas(&self, project_id: i32) -> Result<Vec<DbTrack>, String>;
    async fn delete_track(&self, track_id: &TrackId, project_id: i32) -> Result<(), String>;
    async fn insert_track(&self, track_info: TrackInfo, project_id: i32) -> Result<DbTrack, String>;
    async fn copy_track(&self, source_track_id: &TrackId, new_name: &str, project_id: i32) -> Result<DbTrack, String>;
    async fn update_track_info(
        &self,
        track_id: &TrackId,
        params: UpdateTrackInfoParams,
        project_id: i32,
    ) -> Result<DbTrack, String>;
}

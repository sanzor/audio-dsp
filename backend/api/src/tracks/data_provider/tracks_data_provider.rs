use domain::{
    db::db_track::{DbTrack, DbTrackMeta, TrackId},
    raw_track::RawTrack,
    update_track_info_params::UpdateTrackInfoParams,
};

#[async_trait::async_trait]
pub trait TracksDataProvider: Send + Sync {
    async fn get_track(&self, track_id: &TrackId) -> Result<DbTrack, String>;
    async fn get_all_track_metas(&self) -> Result<Vec<DbTrackMeta>, String>;
    async fn delete_track(&self, track_id: &TrackId) -> Result<(), String>;
    async fn upsert_track(&self, track: RawTrack, project_id: i32) -> Result<DbTrack, String>;
    async fn copy_track(&self, source_track_id: &TrackId, new_name: &str) -> Result<DbTrack, String>;
    async fn update_track_info(
        &self,
        track_id: &TrackId,
        params: UpdateTrackInfoParams,
    ) -> Result<DbTrack, String>;
}

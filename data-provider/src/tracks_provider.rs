use domain::{
    raw_track::RawTrack, stored_track::StoredTrack, track::Track, track_meta::TrackMeta, update_track_info_params::UpdateTrackInfoParams
};
use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::get_all_track_infos_result::GetAllTrackInfosResult;

pub struct LocalTrackStoreProvider {
    pub tracks: Mutex<HashMap<String, StoredTrack>>,
}
#[async_trait::async_trait]
pub trait TracksProvider: Send + Sync {
    async fn get_track_meta(&self, track_name: &str) -> Result<TrackMeta, String>;
    async fn get_stored_track(&self, track_id: &str) -> Result<StoredTrack, String>;
    async fn get_all_track_infos(&self) -> Result<GetAllTrackInfosResult, String>;
    async fn delete_track(&self, track_name: &str) -> Result<(), String>;
    async fn upsert_track(&self, track: RawTrack) -> Result<TrackMeta, String>;
    async fn copy_track(&self, source_track_id: &str, new_name: &str) -> Result<TrackMeta, String>;
    async fn update_track_info(
        &self,
        track_id: &str,
        updated_track_info: UpdateTrackInfoParams,
    ) -> Result<TrackMeta, String>;
}
unsafe impl Send for LocalTrackStoreProvider {}
unsafe impl Sync for LocalTrackStoreProvider {}

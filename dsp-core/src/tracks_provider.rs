use domain::track::{Track, TrackInfo, TrackRef, TrackRefMut};
use std::collections::HashMap;

use crate::get_all_tracks_result::GetAllTrackInfosResult;

pub struct LocalTrackStoreProvider {
    pub tracks: HashMap<String, Track>,
}
#[async_trait::async_trait]
pub trait TracksProvider {
    async fn get_track_info(&self, track_name: &str) -> Result<TrackInfo, String>;
    async fn get_track_ref(&self, track_name: &str) -> Result<TrackRef, String>;
    async fn get_track_ref_mut(&mut self, track_name: &str) -> Result<TrackRefMut, String>;
    async fn get_track_copy(&self, track_name: &str) -> Result<Track, String>;
    async fn get_all_track_infos(&self) -> Result<GetAllTrackInfosResult, String>;
    async fn delete_track(&mut self, track_name: &str) -> Result<(), String>;
    async fn upsert_track(&mut self, track: Track) -> Result<(), String>;
    async fn update_track_info(&mut self, track_info: TrackInfo) -> Result<(), String>;
}

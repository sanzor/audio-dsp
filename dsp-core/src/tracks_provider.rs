use domain::{
    raw_track::{RawTrack, TrackInfo, TrackRef, TrackRefMut},
    track::Track,
    track_meta::TrackMeta,
};
use std::collections::HashMap;

use crate::get_all_tracks_result::GetAllTrackInfosResult;

pub struct LocalTrackStoreProvider {
    pub tracks: HashMap<String, Track>,
}
#[async_trait::async_trait]
pub trait TracksProvider {
    async fn get_track_meta(&self, track_name: &str) -> Result<TrackMeta, String>;
    async fn get_track_ref(&self, track_name: &str) -> Result<TrackRef, String>;
    async fn get_track_ref_mut(&mut self, track_name: &str) -> Result<TrackRefMut, String>;
    async fn get_track_copy(&self, track_name: &str) -> Result<RawTrack, String>;
    async fn get_all_track_infos(&self) -> Result<GetAllTrackInfosResult, String>;
    async fn delete_track(&mut self, track_name: &str) -> Result<(), String>;
    async fn upsert_track(&mut self, track: RawTrack) -> Result<TrackMeta, String>;
    async fn update_track_info(&mut self, track_id:&str,track_info: TrackInfo) -> Result<TrackMeta, String>;
}

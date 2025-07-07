use std::collections::HashMap;

use domain::track::{Track, TrackInfo, TrackRef, TrackRefMut};

use crate::{get_all_tracks_result::GetAllTrackInfosResult, tracks_provider::{LocalTrackStoreProvider, TracksProvider}};

impl LocalTrackStoreProvider {
    pub fn new() -> LocalTrackStoreProvider {
        LocalTrackStoreProvider {
            tracks: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl TracksProvider for LocalTrackStoreProvider {
    async fn get_track_info(&self, track_name: &str) -> Result<TrackInfo, String> {
        let info = self
            .tracks
            .get(track_name)
            .ok_or_else(|| "err".to_string())
            .map(|track| track.info.clone());

        info
    }
    async fn update_track_info(&mut self, track_info: TrackInfo) -> Result<(), String> {
        let mut track=match self.tracks.remove(&track_info.name){
            None=>return Err("Could not find track".into()),
            Some(i)=>i
        };
        track.info=track_info;
        Ok(())
    }
    async fn get_track_ref(&self, track_name: &str) -> Result<TrackRef, String> {
        self.tracks
            .get(track_name)
            .ok_or_else(|| "".into())
            .map(|track| TrackRef { inner: track })
    }

    async fn get_track_ref_mut(&mut self, track_name: &str) -> Result<TrackRefMut, String> {
        self.tracks
            .get_mut(track_name)
            .ok_or_else(|| "".into())
            .map(|track| TrackRefMut { inner: track })
    }

    async fn get_track_copy(&self, track_name: &str) -> Result<Track, String> {
        self.tracks
            .get(track_name)
            .ok_or_else(|| "".into())
            .map(|track| track.clone())
    }
    async fn get_all_track_infos(&self) -> Result<GetAllTrackInfosResult,String> {
        let mut hash_map=HashMap::new();
        for (key,track) in self.tracks.iter(){
            hash_map.insert(key.to_string(), track.info.clone());
        }
        Ok(GetAllTrackInfosResult{track_infos:hash_map})
    }

    async fn delete_track(&mut self, track_name: &str) -> Result<(), String> {
        self.tracks
            .remove(track_name)
            .ok_or_else(|| "could not find key".into())
            .map(|v| ())
    }
    async fn upsert_track(&mut self, track: Track) -> Result<(), String> {
        match self.tracks.insert(track.info.name.clone(), track) {
            None => Ok(()),
            Some(_) => Err("Key already exists".into()),
        }
    }
}

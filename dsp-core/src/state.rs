use domain::track::{Track, TrackInfo, TrackRef, TrackRefMut};
use std::{collections::HashMap, sync::Arc};


pub type TracksState = TrackStoreProvider;
pub struct TrackStoreProvider {
    pub tracks: HashMap<String, Track>,
}
#[async_trait::async_trait]
pub trait TrackStoreOperations{
    async fn get_track_info(&self,track_name:&str)->Result<TrackInfo,String>;
    async fn get_track_ref(&self, track_name: &str) -> Result<TrackRef, String>;
    async fn get_track_ref_mut(&mut self, track_name: &str) -> Result<TrackRefMut, String>;
    async fn get_track_copy(&self, track_name: &str) -> Result<Track, String>;
    async fn get_all_tracks(&self) -> Vec<TrackInfo>;
    async fn delete_track(&mut self, track_name: &str) -> Result<(), String>;
    async fn upsert_track(&mut self, track: Track) -> Result<(), String>;
    async fn update_track_info(&mut self, track_info:TrackInfo)->Result<(),String>;
}

impl TrackStoreProvider{
     pub fn new() -> TrackStoreProvider {
        TrackStoreProvider {
            tracks: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl TrackStoreOperations for TrackStoreProvider {
   
    async fn get_track_info(&self, track_name: &str) -> Result<TrackInfo, String> {
        let info = self
            .tracks
            .get(track_name)
            .ok_or_else(|| "err".to_string())
            .map(|track| track.info.clone());

        info
    }
    async fn update_track_info(&mut self, track_info:TrackInfo)->Result<(),String>{
        todo!()
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
    async fn get_all_tracks(&self) -> Vec<TrackInfo> {
        self.tracks
            .iter()
            .map(|(_, track)| track.info.clone())
            .collect()
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

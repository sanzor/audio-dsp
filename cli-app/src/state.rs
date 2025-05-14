use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::track::{Track, TrackInfo, TrackRef, TrackRefMut};

pub type SharedState = Arc<RwLock<State>>;
pub struct State {
    tracks: HashMap<String, Track>,
}
pub fn create_shared_state() -> SharedState {
    Arc::new(RwLock::new(State::new()))
}
impl State {
    pub fn new() -> State {
        State {
            tracks: HashMap::new(),
        }
    }
    pub fn get_track_info(&self, name: &str) -> Option<TrackInfo> {
        self.tracks.get(name).map(|t| t.info.clone())
    }

    pub fn get_track_ref(&self, name: &str) -> Option<TrackRef> {
        self.tracks.get(name).map(|tr| TrackRef { inner: tr })
    }

    pub fn get_track_ref_mut(&mut self, name: &str) -> Option<TrackRefMut> {
        self.tracks
            .get_mut(name)
            .map(|track_mut| TrackRefMut { inner: track_mut })
    }

    pub fn get_track_copy(&self, name: &str) -> Option<Track> {
        self.tracks.get(name).map(|t| t.clone())
    }
    pub fn tracks(&self) -> Vec<TrackInfo> {
        self.tracks.values().map(|t| t.info.clone()).collect()
    }

    pub fn delete_track(&mut self, name: &str) -> Result<(), String> {
        let result = match self.tracks.remove(name) {
            Some(deleted_track) => Ok(()),
            None => Err("Could not find key".to_string()),
        };
        return result;
    }
    pub fn upsert_track(&mut self, track: Track) -> Result<(), String> {
        let track_name = track.info.name.clone();
        self.tracks.insert(track_name, track);
        Ok(())
    }
}

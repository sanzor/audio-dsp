use std::{collections::HashMap, sync::Arc};

use actix::Addr;
use dsp_domain::track::{Track, TrackInfo, TrackRef, TrackRefMut};
use tokio::sync::Mutex;

use crate::actors::audio_player_actor::AudioPlayerActor;

pub type SharedState = State;
pub struct State {
    pub tracks: HashMap<String, Track>,
    pub players: HashMap<String, Addr<AudioPlayerActor>>,
}
pub fn create_state() -> Arc<Mutex<SharedState>> {
    Arc::new(Mutex::new(State::new()))
}

impl State {
    pub fn new() -> State {
        State {
            tracks: HashMap::new(),
            players: HashMap::new(),
        }
    }
    pub async fn get_track_info(&self, track_name: &str) -> Result<TrackInfo, String> {
        let info = self
            .tracks
            .get(track_name)
            .ok_or_else(|| "err".to_string())
            .map(|track| track.info.clone());

        info
    }

    pub async fn get_track_ref(&self, track_name: &str) -> Result<TrackRef, String> {
        self.tracks
            .get(track_name)
            .ok_or_else(|| "".into())
            .map(|track| TrackRef { inner: track })
    }

    pub async fn get_track_ref_mut(&mut self, track_name: &str) -> Result<TrackRefMut, String> {
        self.tracks
            .get_mut(track_name)
            .ok_or_else(|| "".into())
            .map(|track| TrackRefMut { inner: track })
    }

    pub async fn get_track_copy(&self, track_name: &str) -> Result<Track, String> {
        self.tracks
            .get(track_name)
            .ok_or_else(|| "".into())
            .map(|track| track.clone())
    }
    pub async fn get_all_tracks(&self) -> Vec<TrackInfo> {
        self.tracks
            .iter()
            .map(|(_, track)| track.info.clone())
            .collect()
    }

    pub async fn delete_track(&mut self, track_name: &str) -> Result<(), String> {
        self.tracks
            .remove(track_name)
            .ok_or_else(|| "could not find key".into())
            .map(|v| ())
    }
    pub async fn upsert_track(&mut self, track: Track) -> Result<(), String> {
        self.tracks
            .insert(track.info.name.clone(), track)
            .ok_or_else(|| "could not insert".into())
            .map(|_| ())
    }
}

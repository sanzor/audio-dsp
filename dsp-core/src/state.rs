use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use dsp_domain::track::{self, Track, TrackInfo, TrackRef, TrackRefMut};
use player::player_ref::PlayerRef;

use crate::{player_registry::local_player_registry::LocalPlayerRegistry, user_registry::{user_registry::UserRegistry, LocalUserRegistry}};

pub type SharedState = Arc<State>;
pub(crate) struct State {
    tracks: Arc<dyn UserRegistry>,
    player_refs: Arc<dyn PlayerRef>,
}
pub fn create_shared_state() -> SharedState {
    Arc::new(State::new())
}
impl State {
    pub fn new() -> State {
        State {
            tracks: Arc::new(LocalUserRegistry::new()),
            player_refs:Arc::new(LocalPlayerRegistry::new())
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

    pub fn upsert_player_ref(&mut self,player_ref:Box<impl PlayerRef+'static>)->Result<(),String>{
        self.player_refs.insert(player_ref.id().to_string(),player_ref);
        Ok(())
    }

    pub fn get_player_ref(&mut self,id:String)->Result<(),String>{
        self.player_refs.get(id.as_ref()).ok_or_else(||"Could not find id");
        Ok(())
    }
}

use std::collections::HashMap;

use actix::Addr;
use dsp_domain::track::{Track, TrackInfo, TrackRef, TrackRefMut};
use player::player_ref::AudioPlayerRef;

use crate::actors::audio_player_actor::AudioPlayerActor;

pub type SharedState = State;
pub(crate) struct State {
    pub tracks: HashMap<String, Track>,
    pub players: HashMap<String, Addr<AudioPlayerActor>>,
}
pub fn create_state() -> State {
    State::new()
}

impl State {
    pub fn new() -> State {
        State {
            tracks: HashMap::new(),
            players: HashMap::new(),
        }
    }
    pub async fn get_track_info(
        &self,
        track_name: &str,
    ) -> Result<TrackInfo, String> {
        let info = self
            .tracks
            .get(track_name)
            .ok_or_else(|| "err".to_string())
            .map(|track| track.info.clone());

        info
    }

    pub async fn get_track_ref(
        &self,
        track_name: &str,
    ) -> Result<TrackRef, String> {
            self.tracks.get(track_name).ok_or_else(|| "".into())
            .map(|track| TrackRef{inner:track})
            
    }

    pub async fn get_track_ref_mut(
        &self,
        user_name: &str,
        track_name: &str,
    ) -> Result<TrackRefMut, String> {
        self.user_registry
            .get_track_ref_mut(user_name, track_name)
            .await
    }

    pub async fn get_track_copy(&self, user_name: &str, track_name: &str) -> Result<Track, String> {
        self.user_registry
            .get_track_copy(user_name, track_name)
            .await
    }
    pub async fn get_tracks_for_user(&self, user_name: &str) -> Vec<TrackInfo> {
        self.user_registry.get_tracks_for_user(user_name).await
    }

    pub async fn delete_track(&self, user_id: &str, track_name: &str) -> Result<(), String> {
        self.user_registry.delete_track(user_id, track_name).await
    }
    pub async fn upsert_track(&self, user_name: &str, track: Track) -> Result<(), String> {
        self.user_registry.add_track(user_name, track).await
    }

    pub async fn upsert_audio_player_ref(
        &self,
        player_ref: Box<impl AudioPlayerRef>,
    ) -> Result<(), String> {
        let v = self
            .audio_player_registry
            .upsert(&player_ref.id(), player_ref)
            .await;
        v
    }

    pub async fn get_audio_player_ref(&self, id: &str) -> Result<Box<dyn AudioPlayerRef>, String> {
        self.audio_player_registry.get_by_id(&(id.as_ref())).await
    }
}

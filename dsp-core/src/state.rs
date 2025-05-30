use std::sync::Arc;

use dsp_domain::{
    track::{Track, TrackInfo, TrackRef, TrackRefMut},
};
use player::player_ref::AudioPlayerRef;

use crate::{
    player_registry::{local_player_registry::LocalAudioPlayerRegistry, player_registry::AudioPlayerRegistry},
    user_registry::{user_registry::UserRegistry, LocalUserRegistry},
};

pub type SharedState = Arc<State>;
pub(crate) struct State {
    user_registry: Arc<dyn UserRegistry>,
    audio_player_registry: Arc<dyn AudioPlayerRegistry>,
}
pub fn create_shared_state() -> SharedState {
    Arc::new(State::new())
}

impl State {
    pub fn new() -> State {
        State {
            user_registry: Arc::new(LocalUserRegistry::new()),
            audio_player_registry: Arc::new(LocalAudioPlayerRegistry::new()),
        }
    }
    pub async fn get_track_info(
        &self,
        user_name: &str,
        track_name: &str,
    ) -> Result<TrackInfo, String> {
        let info = self
            .user_registry
            .get_user_track_info(user_name, track_name)
            .await;
        info
    }

    pub async fn get_track_ref(&self, user_name: &str,track_name:&str) -> Result<TrackRef,String> {
        self.user_registry
            .get_track_ref(user_name, track_name)
            .await
    }

    pub async fn get_track_ref_mut(&self, user_name: &str,track_name:&str) -> Result<TrackRefMut,String> {
        self.user_registry
             .get_track_ref_mut(user_name, track_name)
             .await
            
    }

    pub async fn get_track_copy(&self, user_name: &str,track_name:&str) -> Result<Track,String> {
        self.user_registry.get_track_copy(user_name,track_name).await
    }
    pub async fn tracks(&self,user_name:&str) -> Vec<TrackInfo> {
        self.user_registry
            .get_tracks_for_user(user_name).await
            
    }

    pub async fn delete_track(&self, name: &str) -> Result<(), String> {
        let result = match self.user_registry.remove(name).await {
            Some(_) => Ok(()),
            None => Err("Could not find key".to_string()),
        };
        return result;
    }
    pub async fn upsert_track(&self,user_name:&str,track: Track) -> Result<(), String> {
        
        self.user_registry.add_track(user_name,track).await
    }

    pub async fn upsert_player_ref(
        &self,
        player_ref: Box<impl AudioPlayerRef + 'static>,
    ) -> Result<(), String> {
        self.audio_player_registry
            .insert(player_ref.id().to_string(), player_ref);
        Ok(())
    }

    pub fn get_player_ref(&self, id: String) -> Result<(), String> {
        self.audio_player_registry
            .get(id.as_ref())
            .ok_or_else(|| "Could not find id");
        Ok(())
    }
}

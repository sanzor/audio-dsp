use std::{collections::HashMap, sync::Arc};

use kameo::actor::{ActorRef, Spawn};
use tokio::sync::Mutex;

use crate::audio_player_actor::{
    actor::AudioPlayerActor, create_audio_player_actor_params::CreateAudioPlayerActorParams,
};

pub struct AudioPlayerRegistry {
    players: Arc<Mutex<HashMap<String, ActorRef<AudioPlayerActor>>>>,
}

impl AudioPlayerRegistry {
    pub fn new() -> Self {
        Self {
            players: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get(&self, key: &str) -> Option<ActorRef<AudioPlayerActor>> {
        self.players.lock().await.get(key).cloned()
    }

    pub async fn insert(&self, key: String, params: CreateAudioPlayerActorParams) -> ActorRef<AudioPlayerActor> {
        let actor = AudioPlayerActor::spawn(AudioPlayerActor::new(params));
        self.players.lock().await.insert(key, actor.clone());
        actor
    }

    pub async fn remove(&self, key: &str) -> Option<ActorRef<AudioPlayerActor>> {
        self.players.lock().await.remove(key)
    }
}

impl Default for AudioPlayerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

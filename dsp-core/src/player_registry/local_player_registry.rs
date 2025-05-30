use std::{collections::HashMap, sync::Mutex};

use async_trait::async_trait;
use player::player_ref::AudioPlayerRef;

use super::player_registry::AudioPlayerRegistry;

pub struct LocalAudioPlayerRegistry {
    players: Mutex<HashMap<String, Box<dyn AudioPlayerRef>>>,
}

#[async_trait]
impl AudioPlayerRegistry for LocalAudioPlayerRegistry {
    async fn upsert(&self, id: &str, player: Box<dyn AudioPlayerRef>) -> Result<(), String> {
        todo!()
    }
    async fn get_by_id(&self, id:  &str) -> Result<Box<dyn AudioPlayerRef>,String> {
        todo!()
    }

    async fn get_all_ids(&self) -> Vec<String> {
        todo!()
    }

    async fn remove(&self, id:  &str) -> Result<Box<dyn AudioPlayerRef>,String> {
        todo!()
    }
}

impl LocalAudioPlayerRegistry {
    pub fn new() -> LocalAudioPlayerRegistry {
        LocalAudioPlayerRegistry {
            players: Mutex::new(HashMap::new()),
        }
    }
}

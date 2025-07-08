use std::collections::HashMap;

use kameo::actor::ActorRef;

use crate::{
    audio_player_actor::audio_player_actor::AudioPlayerActor,
    user_actor::{
        get_all_players_result::GetAllPlayersResult, get_player_result::GetPlayerResult,
        players_provider::PlayersProvider,
    },
};

type Players = HashMap<String, ActorRef<AudioPlayerActor>>;
pub struct LocalPlayerProvider {
    pub players: Players,
}
impl LocalPlayerProvider {
    pub fn new() -> LocalPlayerProvider {
        LocalPlayerProvider {
            players: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl PlayersProvider for LocalPlayerProvider {
    async fn get_all(&self) -> Result<GetAllPlayersResult, String> {
        todo!()
    }
    async fn get(&self, player_id: String) -> Result<GetPlayerResult, String> {
        todo!()
    }
    async fn store(
        &mut self,
        player_id: String,
        player_ref: ActorRef<AudioPlayerActor>,
    ) -> Result<(), String> {
        todo!()
    }
    async fn remove(&mut self, player_id: String) -> Result<ActorRef<AudioPlayerActor>, String> {
        todo!()
    }
}

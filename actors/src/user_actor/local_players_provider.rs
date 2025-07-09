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
        let result: Vec<GetPlayerResult> = self
            .players
            .iter()
            .map(|(k, v)| GetPlayerResult {
                player_id: k.to_string(),
                player_ref: v.clone(),
            })
            .collect();
        Ok(GetAllPlayersResult { items: result })
    }
    async fn get(&self, player_id: String) -> Result<GetPlayerResult, String> {
        match self.players.get(&player_id) {
            Some(pl) => Ok(GetPlayerResult {
                player_ref: pl.clone(),
                player_id: player_id,
            }),
            None => Err("Could not find player".into()),
        }
    }
    async fn store(
        &mut self,
        player_id: String,
        player_ref: ActorRef<AudioPlayerActor>,
    ) -> Result<(), String> {
        self.players.insert(player_id, player_ref);
        Ok(())
    }
    async fn remove(&mut self, player_id: String) -> Result<ActorRef<AudioPlayerActor>, String> {
        match self.players.remove(&player_id) {
            Some(pl) => Ok(pl),
            None => Err("Could not remoove".into()),
        }
    }
}

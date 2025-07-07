use std::collections::HashMap;

use kameo::actor::ActorRef;

use crate::audio_player_actor::audio_player_actor::AudioPlayerActor;


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

impl PlayersProvider for LocalPlayersProvider {
    async fn get_all_players(&self) -> Result<GetAllPlayersResult, String>{

    }
    async fn get_player(&self, player_id: String) -> Result<GetPlayersResult, String>{

    }
    async fn store(&mut self, params: CreatePlayerParams) -> Result<CreatePlayerResult, String>{

    }
    async fn remove(&mut self, player_id: String) -> Result<(), String> {}
}

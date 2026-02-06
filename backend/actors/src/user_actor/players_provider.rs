use kameo::actor::ActorRef;

use crate::{
    audio_player_actor::audio_player_actor::AudioPlayerActor,
    user_actor::{get_all_players_result::GetAllPlayersResult, get_player_result::GetPlayerResult},
};

#[async_trait::async_trait]
pub trait PlayersProvider {
    async fn get(&self, player_id: String) -> Result<GetPlayerResult, String>;
    async fn get_all(&self) -> Result<GetAllPlayersResult, String>;
    async fn store(
        &mut self,
        player_id: String,
        player_ref: ActorRef<AudioPlayerActor>,
    ) -> Result<(), String>;
    async fn remove(&mut self, player_id: String) -> Result<ActorRef<AudioPlayerActor>, String>;
}

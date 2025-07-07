
use domain::actors::messages::user_to_player::{get_player_state::GetPlayerStateResult, play::PlayResult};
use kameo::actor::ActorRef;

use crate::{
    audio_player_actor::audio_player_actor::AudioPlayerActor,
    user_actor::{
        get_all_players_result::GetAllPlayersResult,
        get_player_result::{GetPlayerResult},
    },
};

#[async_trait::async_trait]
pub trait PlayersProvider {
    async fn get(&self, player_id: String) -> Result<GetPlayerResult, String>;
    async fn get_all(&self) -> Result<GetAllPlayersResult, String>;
    async fn store(
        &mut self,
        player_id: String,
        track: ActorRef<AudioPlayerActor>,
    ) -> Result<CreatePlayerResult, String>;
    async fn remove(&mut self, player_id: String) -> Result<ActorRef<AudioPlayerActor>, String>;
    async fn play(&mut self, player_id: String) -> Result<PlayResult, String>;
    async fn pause(&mut self, player_id: String) -> Result<PlayResult, String>;
    async fn stop(&mut self, player_id: String) -> Result<PlayResult, String>;
    async fn seek(&mut self, player_id: String, position: u32) -> Result<PlayResult, String>;
    async fn get_player_state(&self, player_id: String) -> Result<GetPlayerStateResult, String>;
}

use kameo::actor::ActorRef;

use crate::audio_player_actor::audio_player_actor::AudioPlayerActor;

#[async_trait::async_trait]
pub trait PlayersProvider{
    async fn get(&mut self,player_id:String)->Result<ActorRef<AudioPlayerActor,String>>;
    async fn create(&mut self,player_id:String,track:ActorRef<AudioPlayerActor>)->Result<CreatePlayerResult,String>;
    async fn remove(&mut self,player_id:String)->Result<(),String>;
    async fn play(&mut self,player_id:String)->Result<PlayResult,String>;
    async fn pause(&mut self,player_id:String)->Result<PlayResult,String>;
    async fn stop(&mut self,player_id:String)->Result<PlayResult,String>;
    async fn seek(&mut self,player_id:String,position:u32)->Result<PlayResult,String>;
    async fn get_player_state(&self,player_id:String)->Result<GetPlayerStateResult,String>;
}
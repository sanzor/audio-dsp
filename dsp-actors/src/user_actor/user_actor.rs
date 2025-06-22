use dsp_core::{command_processor::CommandProcessor, state::TracksState};
use kameo::{
    actor::ActorRef,
    prelude::{Context, Message},
    spawn, Actor,
};
use player::audio_sink::cpal_sink::CpalSink;
use std::{collections::HashMap, hash::Hash};

use crate::{audio_player_actor::{
    audio_player_actor::AudioPlayerActor, audio_player_actor_params::AudioPlayerActorParams, state_request::StateRequest,
}, user_actor::get_state_result::GetStateResult};
use dsp_domain::{
    audio_player_message::AudioPlayerMessage, audio_player_message_result::AudioPlayerMessageResult, dsp_message::DspMessage, track::TrackInfo, tracks_message_result::TracksMessageResult, user::User
};

type Players = HashMap<String, ActorRef<AudioPlayerActor>>;

#[derive(Actor)]
#[actor(name = "UserActor")]
pub struct UserActor {
    processor: CommandProcessor,
    track_state: TracksState,
    players: Players,
}

impl UserActor {
    pub fn new(processor: CommandProcessor, tracks: TracksState, players: Players) -> UserActor {
        UserActor {
            processor: processor,
            track_state: tracks,
            players: players,
        }
    }
}

impl Message<DspMessage> for UserActor {
    type Reply = Result<TracksMessageResult, String>;

    // async move variant
    async fn handle(
        &mut self,
        msg: DspMessage,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let v = self
            .processor
            .process_crud_command(msg, &mut self.track_state)
            .await;
        v
    }
}

impl Message<AudioPlayerMessage> for UserActor {
    type Reply = Result<AudioPlayerMessageResult, String>;
    async fn handle(
        &mut self,
        msg: AudioPlayerMessage,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let c = match msg {
            AudioPlayerMessage::Play { track_id } => self.handle_play(track_id).await,
            AudioPlayerMessage::Pause { track_id } => self.handle_pause(track_id).await,
            AudioPlayerMessage::Stop { track_id } => self.handle_stop(track_id).await,
            AudioPlayerMessage::State{track_id}=>{
                self.handle_get_player_state(track_id).await
            }
        };
        c
    }
}

impl UserActor {
    pub async fn handle_play(
        &mut self,
        track_id: Option<String>,
    ) -> Result<AudioPlayerMessageResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let player=self.players.get(&track_id);
        match player.cloned(){
            None=>self.handle_new_player(&track_id).await,
            Some(p)=>self.handle_play_existing_player(&p).await
        }
    }
    pub async fn handle_pause(
        &mut self,
        track_id: Option<String>,
    ) -> Result<AudioPlayerMessageResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let track_ref = self.track_state.get_track_ref(&track_id).await?;
        todo!()
    }
    pub async fn handle_stop(
        &mut self,
        track_id: Option<String>,
    ) -> Result<AudioPlayerMessageResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let track_ref = self.track_state.get_track_ref(&track_id).await?;
        todo!()
    }
    pub async fn handle_get_player_state(& self,track_id:Option<String>)->Result<AudioPlayerMessageResult, String>{
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let player=self.players.get(&track_id);
        match player.cloned(){
            None=>Err("no such player exists".to_string()),
            Some(p)=>{
                let x=p.ask(StateRequest{}).await.map_err(|e|e.to_string())?;
                Ok(AudioPlayerMessageResult{output:serde_json::to_string(&x).unwrap(),should_exit:false})
            }
        }
    }

    pub async fn handle_get_state(&mut self)->Result<GetStateResult,String>{
        let mut player_list:HashMap<String, AudioPlayerActorStateResult>=HashMap::new();
        let mut track_list:HashMap<String,TrackInfo>=HashMap::new();
        for (key,player_ref) in self.players.iter(){
            let player_state=self.handle_get_player_state(Some(key.into())).await?;
            player_list.insert(key.into(),player_state);
        }
        for (key, track) in self.track_state.tracks.iter(){
            track_list.insert(key.into(), track.info.clone());
        }
        
        Ok(GetStateResult{players:player_list,tracks:track_list})
    }
    
    pub async handle_get_player_state(&mut self,track_id:&str)->Result<AudioPlayerActorStateResult,String>{

    }



    async fn handle_new_player(&mut self,track_id:&str)->Result<AudioPlayerMessageResult,String>{
        let track_ref = self.track_state.get_track_ref(track_id).await?;
        let sink = Box::new(CpalSink::new()?);
        let params = AudioPlayerActorParams {
            track: track_ref.inner.clone(),
            cursor: 0,
            sink: sink,
        };
        let player_actor = spawn(AudioPlayerActor::new(params));
        self.players.insert(k, v)
    }
    async fn handle_play_existing_player(&mut self,player_ref:&ActorRef<AudioPlayerActor>)->Result<AudioPlayerMessageResult,String>{
        todo!()
    }
}


use std::time::Duration;

use actix::ActorContext;
use dsp_domain::audio_player_message_result::AudioPlayerMessageResult;
use dsp_domain::track::Track;
use kameo::{message::{Context, Message}, Actor};
use player::audio_sink::AudioSink;
use tokio::time::Interval;
use crate::{audio_player_actor::audio_player_actor_params::AudioPlayerActorParams, AudioPlayerMessage};

enum AudioPlayerState {
    Paused,
    Playing,
}
#[derive(Actor)]
#[actor(name="AudioPlayerActor")]
pub struct AudioPlayerActor {
    sink: Box<dyn AudioSink + Send  + 'static>,
    state: AudioPlayerState,
    cursor: usize,
    track: Track,
}

impl Message<AudioPlayerMessage> for AudioPlayerActor {
    type Reply = Result<AudioPlayerMessageResult,String>;

    async fn handle(
        &mut self,
        msg: AudioPlayerMessage,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        
        match msg {
             AudioPlayerMessage::Play => self.handle_play().await,
             AudioPlayerMessage::Pause =>self.handle_pause().await,
             AudioPlayerMessage::Stop =>{
                 let v=ctx.actor_ref().stop_gracefully()
                .await.map(|_|AudioPlayerMessageResult{output:"".to_string(),should_exit:true}).map_err(|x|x.to_string());
                 v
             },
             AudioPlayerMessage::Seek { position } => {
                self.handle_seek(position).await
            }
        }
    }
}
impl AudioPlayerActor{
    pub fn new(params:AudioPlayerActorParams)->AudioPlayerActor{
        
     
        AudioPlayerActor { sink: params.sink, state: AudioPlayerState::Paused, cursor: params.cursor, track: params.track }
    }

    fn start_streaming_task(ctx:&mut ActorContext<Self>){
            let interval=tokio::time::interval(Duration::from_millis(200));
            let task=tokio::task::spawn(async move{
            loop{
                interval.tick().await;
                
            }
        });
    }
    pub async fn handle_play(&mut self) -> Result<AudioPlayerMessageResult, String> {
        if matches!(self.state,AudioPlayerState::Paused){
            self.state=AudioPlayerState::Playing
        }
        Ok(AudioPlayerMessageResult{output:"".to_string(),should_exit:false})
    }
    pub async fn handle_pause(&mut self) -> Result<AudioPlayerMessageResult, String> {
        if matches!(self.state,AudioPlayerState::Playing){
            self.state=AudioPlayerState::Paused
        }
        Ok(AudioPlayerMessageResult{output:"".to_string(),should_exit:false})
    }
    pub async fn handle_seek(&mut self, position: u32) -> Result<AudioPlayerMessageResult, String> {
        if matches!(self.state,AudioPlayerState::Playing){
            self.state=AudioPlayerState::Paused
        }
        self.cursor=position as usize;
        self.state=AudioPlayerState::Paused;    
        Ok(AudioPlayerMessageResult{output:"".to_string(),should_exit:false})
    }
}

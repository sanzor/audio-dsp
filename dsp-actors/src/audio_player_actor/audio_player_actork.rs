
use audiolib::audio_buffer::AudioBuffer;
use dsp_domain::audio_player_message_result::AudioPlayerMessageResult;
use dsp_domain::track::{Track, TrackInfo};
use kameo::actor::ActorRef;
use kameo::message::{Context, Message};
use kameo::Actor;
use player::audio_sink::cpal_sink::CpalSink;
use player::audio_sink::AudioSink;

use crate::AudioPlayerMessage;

enum AudioPlayerState {
    Paused,
    Playing,
}
#[derive(Actor)]
struct AudioPlayerActor {
    sink: Box<dyn AudioSink + Send + Sync + 'static>,
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
                ctx.
             },
             AudioPlayerMessage::Seek { position } => {
                self.handle_seek(position).await
            }
        }
    }
}
impl AudioPlayerActor{
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
        let v=AudioPlayerActor::spawn(AudioPlayerActor{
            sink:CpalSink::new()?,
            cursor:0,
            track:Track { info:TrackInfo{name:"".to_string() , data: AudioBuffer{channels:audiolib::Channels::Mono}}}});
        if matches!(self.state,AudioPlayerState::Playing){
            self.state=AudioPlayerState::Paused
        }
        self.cursor=position as usize;
        Ok(AudioPlayerMessageResult{output:"".to_string(),should_exit:false})
    }
}

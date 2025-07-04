use crate::audio_player_actor::audio_player_actor_params::AudioPlayerActorParams;
use audiolib::Channels;
use domain::actors::{
    player_command::PlayerCommand, player_command_result::PlayerCommandResult,
    player_state::AudioPlayerState, player_state_query::PlayerStateQuery,
    player_state_query_result::PlayerStateQueryResult,
};
use domain::track::Track;
use kameo::{
    actor::ActorRef,
    message::{Context, Message},
};
use player::{audio_sink::AudioSink, AudioFrame};
use std::time::Duration;

#[async_trait::async_trait]
pub trait PlayerOperations{
    async fn play(&mut self)->Result<PlayResult,String>;
    async fn pause(&mut self)->Result<PlayResult,String>;
    async fn stop(&mut self)->Result<PlayResult,String>;
    async fn seek(&mut self,position:u32)->Result<PlayResult,String>;
    async fn get_player_state(&self)->Result<GetPlayerStateResult,String>;
}

pub struct AudioPlayerActor {
    sink: Box<dyn AudioSink + Send + Sync>,
    state: AudioPlayerState,
    cursor: usize,
    frames_written: usize,
    track: Track,
}
impl kameo::Actor for AudioPlayerActor {
    type Error = String;
    async fn on_start(
        &mut self,
        actor_ref: kameo::prelude::ActorRef<Self>,
    ) -> Result<(), Self::Error> {
        let v = self.start_streaming_task(actor_ref);
        Ok(())
    }
}
struct PlayFrame {}
impl Message<PlayerCommand> for AudioPlayerActor {
    type Reply = Result<PlayerCommandResult, String>;

    async fn handle(
        &mut self,
        msg: PlayerCommand,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        match msg {
            PlayerCommand::Play => self.handle_play().await,
            PlayerCommand::Pause => self.handle_pause().await,
            PlayerCommand::Stop => {
                let v = ctx
                    .actor_ref()
                    .stop_gracefully()
                    .await
                    .map(|_| PlayerCommandResult {
                        output: "".to_string(),
                        should_exit: true,
                    })
                    .map_err(|x| x.to_string());
                v
            }
            PlayerCommand::Seek { position } => self.handle_seek(position).await,
        }
    }
}

impl Message<PlayFrame> for AudioPlayerActor {
    type Reply = Result<(), String>;

    async fn handle(
        &mut self,
        msg: PlayFrame,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        if !matches!(self.state, AudioPlayerState::Playing) {
            return Ok(()); //
        }
        if let Some(frame) = self.get_frame(self.cursor) {
            self.sink.write_frame(&frame).await?;
            self.cursor += 1;
            self.frames_written += 1;
            Ok(())
        } else {
            self.state = AudioPlayerState::Paused;
            self.cursor = 0;
            Ok(())
        }
    }
}
impl Message<PlayerStateQuery> for AudioPlayerActor {
    type Reply = Result<PlayerStateQueryResult, String>;

    async fn handle(
        &mut self,
        msg: PlayerStateQuery,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        Ok((PlayerStateQueryResult {
            cursor: self.cursor,
            state: self.state.clone(),
            written: self.frames_written,
        }))
    }
}
impl AudioPlayerActor {
    pub fn new(params: AudioPlayerActorParams) -> AudioPlayerActor {
        AudioPlayerActor {
            sink: params.sink,
            state: AudioPlayerState::Paused,
            cursor: params.cursor,
            track: params.track,
            frames_written: 0,
        }
    }

    fn start_streaming_task(&self, actor_ref: ActorRef<Self>) {
        let mut interval = tokio::time::interval(Duration::from_millis(200));
        let task = tokio::task::spawn(async move {
            loop {
                interval.tick().await;
                let x = actor_ref.tell(PlayFrame {}).await;
            }
        });
    }
    pub async fn handle_play(&mut self) -> Result<PlayerCommandResult, String> {
        if matches!(self.state, AudioPlayerState::Paused) {
            self.state = AudioPlayerState::Playing
        }
        Ok(PlayerCommandResult {
            output: "".to_string(),
            should_exit: false,
        })
    }
    pub async fn handle_pause(&mut self) -> Result<PlayerCommandResult, String> {
        if matches!(self.state, AudioPlayerState::Playing) {
            self.state = AudioPlayerState::Paused
        }
        Ok(PlayerCommandResult {
            output: "".to_string(),
            should_exit: false,
        })
    }
    pub async fn handle_seek(&mut self, position: u32) -> Result<PlayerCommandResult, String> {
        if matches!(self.state, AudioPlayerState::Playing) {
            self.state = AudioPlayerState::Paused
        }
        self.cursor = position as usize;
        self.state = AudioPlayerState::Paused;
        Ok(PlayerCommandResult {
            output: "".to_string(),
            should_exit: false,
        })
    }

    fn get_frame(&self, cursor: usize) -> Option<AudioFrame> {
        let frame: Option<Vec<f32>> = match self.track.data.channels {
            Channels::Mono => self.track.data.samples.get(cursor).map(|&s| vec![s]),
            Channels::Stereo => {
                let i = cursor * 2;
                if i + 1 >= self.track.data.samples.len() {
                    None
                } else {
                    Some(vec![
                        self.track.data.samples[i],
                        self.track.data.samples[i + 1],
                    ])
                }
            }
        };
        frame
    }
}

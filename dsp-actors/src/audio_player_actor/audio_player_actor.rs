use crate::{
    audio_player_actor::{audio_player_actor_params::AudioPlayerActorParams, state_reply::StateReply, state_request::StateRequest}, AudioPlayerMessage,
};
use audiolib::Channels;
use dsp_domain::audio_player_message_result::AudioPlayerMessageResult;
use dsp_domain::track::Track;
use kameo::{
    actor::ActorRef,
    message::{Context, Message},
};
use player::{audio_sink::AudioSink, AudioFrame};
use std::time::Duration;

#[derive(Clone)]
pub enum AudioPlayerState {
    Paused,
    Playing,
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
impl Message<AudioPlayerMessage> for AudioPlayerActor {
    type Reply = Result<AudioPlayerMessageResult, String>;

    async fn handle(
        &mut self,
        msg: AudioPlayerMessage,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        match msg {
            AudioPlayerMessage::Play => self.handle_play().await,
            AudioPlayerMessage::Pause => self.handle_pause().await,
            AudioPlayerMessage::Stop => {
                let v = ctx
                    .actor_ref()
                    .stop_gracefully()
                    .await
                    .map(|_| AudioPlayerMessageResult {
                        output: "".to_string(),
                        should_exit: true,
                    })
                    .map_err(|x| x.to_string());
                v
            }
            AudioPlayerMessage::Seek { position } => self.handle_seek(position).await,
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
impl Message<StateRequest> for AudioPlayerActor{
    type Reply=Result<StateReply,String>;

    async fn handle(
        &mut self,
        msg: StateRequest,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        Ok((StateReply { cursor: self.cursor, state: self.state.clone() }))
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
    pub async fn handle_play(&mut self) -> Result<AudioPlayerMessageResult, String> {
        if matches!(self.state, AudioPlayerState::Paused) {
            self.state = AudioPlayerState::Playing
        }
        Ok(AudioPlayerMessageResult {
            output: "".to_string(),
            should_exit: false,
        })
    }
    pub async fn handle_pause(&mut self) -> Result<AudioPlayerMessageResult, String> {
        if matches!(self.state, AudioPlayerState::Playing) {
            self.state = AudioPlayerState::Paused
        }
        Ok(AudioPlayerMessageResult {
            output: "".to_string(),
            should_exit: false,
        })
    }
    pub async fn handle_seek(&mut self, position: u32) -> Result<AudioPlayerMessageResult, String> {
        if matches!(self.state, AudioPlayerState::Playing) {
            self.state = AudioPlayerState::Paused
        }
        self.cursor = position as usize;
        self.state = AudioPlayerState::Paused;
        Ok(AudioPlayerMessageResult {
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

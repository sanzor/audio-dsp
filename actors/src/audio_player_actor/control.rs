use std::time::Duration;

use audiolib::Channels;

use domain::actors::{
    messages::player::{
        pause::{Pause, PauseResult},
        play::{Play, PlayResult},
        seek::{Seek, SeekResult},
        stop::{Stop, StopResult},
    },
    player_state::AudioPlayerState,
};
use kameo::{
    actor::ActorRef,
    prelude::{Context, Message},
};
use player::AudioFrame;

use crate::audio_player_actor::audio_player_actor::AudioPlayerActor;
struct PlayFrame {}
impl Message<Play> for AudioPlayerActor {
    type Reply = Result<PlayResult, String>;

    async fn handle(&mut self, msg: Play, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        if matches!(self.state, AudioPlayerState::Paused) {
            self.state = AudioPlayerState::Playing
        }
        Ok(PlayResult {})
    }
}

impl Message<Pause> for AudioPlayerActor {
    type Reply = Result<PauseResult, String>;

    async fn handle(&mut self, msg: Pause, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        if matches!(self.state, AudioPlayerState::Playing) {
            self.state = AudioPlayerState::Paused
        }
        Ok(PauseResult {})
    }
}

impl Message<Seek> for AudioPlayerActor {
    type Reply = Result<SeekResult, String>;

    async fn handle(&mut self, msg: Seek, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        if matches!(self.state, AudioPlayerState::Playing) {
            self.state = AudioPlayerState::Paused
        }
        self.cursor = msg.position as usize;
        self.state = AudioPlayerState::Paused;
        Ok(SeekResult {})
    }
}

impl Message<Stop> for AudioPlayerActor {
    type Reply = Result<StopResult, String>;

    async fn handle(&mut self, msg: Stop, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        let v = ctx
            .actor_ref()
            .stop_gracefully()
            .await
            .map(|_| StopResult {})
            .map_err(|x| x.to_string());
        v
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
            self.sink.write_frame(frame).await?;
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

impl AudioPlayerActor {
    pub(crate) fn start_streaming_task(&self, actor_ref: ActorRef<Self>) {
        let mut interval = tokio::time::interval(Duration::from_millis(200));
        let task = tokio::task::spawn(async move {
            loop {
                interval.tick().await;
                let x = actor_ref.tell(PlayFrame {}).await;
            }
        });
    }

    fn get_frame(&self, cursor: usize) -> Option<AudioFrame> {
        let frame: Option<Vec<f32>> = match self.track_payload.channels {
            Channels::Mono => self.track_payload.samples.get(cursor).map(|&s| vec![s]),
            Channels::Stereo => {
                let i = cursor * 2;
                if i + 1 >= self.track_payload.samples.len() {
                    None
                } else {
                    Some(vec![
                        self.track_payload.samples[i],
                        self.track_payload.samples[i + 1],
                    ])
                }
            }
        };
        frame
    }
}

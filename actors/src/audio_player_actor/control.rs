use std::time::Duration;

use audiolib::Channels;

use domain::actors::{
    messages::player::{
        pause::{Pause, PauseResult},
        play::{Play, PlayResult},
        remove_sink::{RemoveSink, RemoveSinkResult},
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

use crate::audio_player_actor::{
    actor::AudioPlayerActor,
    attach_sink::{AttachSink, AttachSinkResult},
};
struct PlayFrame {}
impl Message<Play> for AudioPlayerActor {
    type Reply = Result<PlayResult, String>;

    async fn handle(&mut self, _msg: Play, _ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        if matches!(self.state, AudioPlayerState::Paused) {
            self.state = AudioPlayerState::Playing
        }
        Ok(PlayResult {})
    }
}

impl Message<Pause> for AudioPlayerActor {
    type Reply = Result<PauseResult, String>;

    async fn handle(&mut self, _msg: Pause, _ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        if matches!(self.state, AudioPlayerState::Playing) {
            self.state = AudioPlayerState::Paused
        }
        Ok(PauseResult {})
    }
}

impl Message<Seek> for AudioPlayerActor {
    type Reply = Result<SeekResult, String>;

    async fn handle(&mut self, msg: Seek, _ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
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

    async fn handle(&mut self, _msg: Stop, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
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
        _msg: PlayFrame,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        if !matches!(self.state, AudioPlayerState::Playing) {
            return Ok(()); //
        }
        if let Some(frame) = self.get_frame(self.cursor) {
            for (_sink_id, sink) in self.sinks.iter_mut() {
                match sink.write_frame(frame.clone()).await {
                    Ok(_) => (),
                    Err(_e) => {}
                }
                self.cursor += 1;
                self.frames_written += 1;
            }

            Ok(())
        } else {
            self.state = AudioPlayerState::Paused;
            self.cursor = 0;
            Ok(())
        }
    }
}

impl Message<AttachSink> for AudioPlayerActor {
    type Reply = Result<AttachSinkResult, String>;

    async fn handle(
        &mut self,
        msg: AttachSink,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let sink_id = ulid::Ulid::new().to_string();
        match self.sinks.insert(sink_id.clone(), msg.sink) {
            None => Ok(AttachSinkResult { sink_id }),
            Some(_s) => Err("Sink already present".to_string()),
        }
    }
}

impl Message<RemoveSink> for AudioPlayerActor {
    type Reply = Result<RemoveSinkResult, String>;

    async fn handle(
        &mut self,
        msg: RemoveSink,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        match self.sinks.remove(&msg.sink_id) {
            Some(_sink) => Ok(RemoveSinkResult {}),
            None => Err("Could not find sink".to_string()),
        }
    }
}
impl AudioPlayerActor {
    pub(crate) fn start_streaming_task(&self, actor_ref: ActorRef<Self>) {
        let mut interval = tokio::time::interval(Duration::from_millis(200));
        let _task = tokio::task::spawn(async move {
            loop {
                interval.tick().await;
                let _x = actor_ref.tell(PlayFrame {}).await;
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

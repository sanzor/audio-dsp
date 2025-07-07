use crate::audio_player_actor::create_audio_player_actor_params::CreateAudioPlayerActorParams;
use audiolib::Channels;
use domain::actors::messages::player::pause::Pause;
use domain::actors::messages::player::play::Play;
use domain::actors::messages::player::seek::Seek;
use domain::actors::messages::player::stop::Stop;
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

pub struct AudioPlayerActor {
    pub(crate) sink: Box<dyn AudioSink + Send + Sync>,
    pub(crate) state: AudioPlayerState,
    pub(crate) cursor: usize,
    pub(crate) frames_written: usize,
    pub(crate) track: Track,
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

    type Args = Self;
}

impl AudioPlayerActor {
    pub fn new(params: CreateAudioPlayerActorParams) -> AudioPlayerActor {
        AudioPlayerActor {
            sink: params.sink,
            state: AudioPlayerState::Paused,
            cursor: params.cursor,
            track: params.track,
            frames_written: 0,
        }
    }
}

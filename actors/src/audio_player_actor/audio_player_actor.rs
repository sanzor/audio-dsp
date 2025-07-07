use crate::audio_player_actor::create_audio_player_actor_params::CreateAudioPlayerActorParams;


use domain::actors::{
    
    player_state::AudioPlayerState

};
use domain::track::Track;

use player::{audio_sink::AudioSink};

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

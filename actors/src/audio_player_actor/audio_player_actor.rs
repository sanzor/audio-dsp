use crate::audio_player_actor::create_audio_player_actor_params::CreateAudioPlayerActorParams;

use domain::actors::player_state::AudioPlayerState;
use domain::raw_track::RawTrack;
use kameo::prelude::ActorRef;
use player::audio_sink::AudioSink;

pub struct AudioPlayerActor {
    pub(crate) sink: Box<dyn AudioSink + Send + Sync>,
    pub(crate) state: AudioPlayerState,
    pub(crate) cursor: usize,
    pub(crate) frames_written: usize,
    pub(crate) track: RawTrack,
}
impl kameo::Actor for AudioPlayerActor {
    type Error = String;

    type Args = Self;

    async fn on_start(args: Self::Args, actor_ref: ActorRef<Self>) -> Result<Self, Self::Error> {
        let v = args.start_streaming_task(actor_ref);
        Ok(args)
    }
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

use std::{collections::HashMap, sync::Arc};

use crate::audio_player_actor::create_audio_player_actor_params::CreateAudioPlayerActorParams;

use audiolib::{audio_buffer::AudioBuffer, decoded_audio::DecodedAudio};
use domain::{actors::player_state::AudioPlayerState, track_meta::TrackMeta};
use kameo::prelude::ActorRef;
use player::audio_sink::AudioSink;

pub struct AudioPlayerActor {
    pub(crate) state: AudioPlayerState,
    pub(crate) cursor: usize,
    pub(crate) frames_written: usize,
    pub(crate) track_payload: Arc<DecodedAudio>,
    pub(crate) track_meta: TrackMeta,
    pub(crate) sinks: HashMap<String, Box<dyn AudioSink + Sync + Send>>,
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
            state: AudioPlayerState::Paused,
            cursor: params.cursor,
            track_payload: Arc::new(params.track_payload.audio),
            frames_written: 0,
            track_meta: params.track_payload.meta,
            sinks: params.sinks,
        }
    }
}

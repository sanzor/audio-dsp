use std::sync::Arc;

use audiolib::audio_buffer::AudioBuffer;
use domain::track_meta::TrackMeta;
use kameo::actor::ActorRef;
use player::audio_sink::AudioSink;

use crate::audio_player_actor::audio_player_actor::AudioPlayerActor;

pub struct CreateAudioPlayerActorParams {
    pub sink: Box<dyn AudioSink + Sync + Send + 'static>,
    pub cursor: usize,
    pub track_payload: Arc<AudioBuffer>,
    pub meta: TrackMeta,
}

pub struct CreateAudioPlayerActorResult {
    pub audio_actor_ref: ActorRef<AudioPlayerActor>,
}

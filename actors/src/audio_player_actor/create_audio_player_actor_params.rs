use domain::{track::Track};
use kameo::actor::ActorRef;
use player::audio_sink::AudioSink;

use crate::audio_player_actor::audio_player_actor::AudioPlayerActor;

pub struct CreateAudioPlayerActorParams {
    pub sink: Box<dyn AudioSink + Sync + Send + 'static>,
    pub cursor: usize,
    pub track: Track,
}

pub struct CreateAudioPlayerActorResult {
    pub audio_actor_ref: ActorRef<AudioPlayerActor>,
}

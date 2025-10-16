use kameo::actor::ActorRef;
use player::sink::AudioSink;
use std::collections::HashMap;

use crate::audio_player_actor::{
    actor::AudioPlayerActor, player_track_payload::PlayerTrackPayload,
};

pub struct CreateAudioPlayerActorParams {
    pub sinks: HashMap<String, Box<dyn AudioSink + Sync + Send + 'static>>,
    pub cursor: usize,
    pub track_payload: PlayerTrackPayload,
}

pub struct CreateAudioPlayerActorResult {
    pub audio_actor_ref: ActorRef<AudioPlayerActor>,
}

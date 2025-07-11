use std::sync::Arc;

use audiolib::audio_buffer::AudioBuffer;
use domain::{
    actors::messages::player::get_player_state::{GetPlayerState, GetPlayerStateResult},
    raw_track::{RawTrack, TrackInfo},
    track::Track,
    track_meta::TrackMeta,
};

use kameo::{actor::ActorRef, Actor};
use player::audio_sink::AudioSink;

use crate::audio_player_actor::{
    audio_player_actor::AudioPlayerActor,
    create_audio_player_actor_params::CreateAudioPlayerActorParams,
};

pub(crate) fn create_user_actor_with_track(
    meta: TrackMeta,
    buffer: Arc<AudioBuffer>,
    sink: Box<dyn AudioSink + Send + Sync + 'static>,
) -> ActorRef<AudioPlayerActor> {
    let audio_player_actor_params = CreateAudioPlayerActorParams {
        sink: sink,
        track_payload: Arc::clone(&buffer),
        meta: meta,
        cursor: 0,
    };
    let audio_player_actor =
        AudioPlayerActor::spawn(AudioPlayerActor::new(audio_player_actor_params));

    let g = kameo::registry::ActorRegistry::new();

    audio_player_actor
}

pub(crate) async fn get_player_actor_state(
    actor_ref: &ActorRef<AudioPlayerActor>,
) -> Result<GetPlayerStateResult, String> {
    let state_query_result = actor_ref
        .ask(GetPlayerState {})
        .await
        .map_err(|e| e.to_string());
    state_query_result
}

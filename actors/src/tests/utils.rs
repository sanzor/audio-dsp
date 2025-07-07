use domain::{
    actors::{
        player_state_query::PlayerStateQuery, player_state_query_result::PlayerStateQueryResult,
    },
    track::Track,
};
use kameo::{actor::ActorRef, spawn};
use player::audio_sink::AudioSink;

use crate::audio_player_actor::{
    audio_player_actor::AudioPlayerActor,
    create_audio_player_actor_params::CreateAudioPlayerActorParams,
};

pub(crate) fn create_user_actor(
    track: Track,
    sink: Box<dyn AudioSink + Send + Sync + 'static>,
) -> ActorRef<AudioPlayerActor> {
    let audio_player_actor_params = CreateAudioPlayerActorParams {
        sink: sink,
        track: track,
        cursor: 0,
    };
    let audio_player_actor = spawn(AudioPlayerActor::new(audio_player_actor_params));

    let g = kameo::registry::ActorRegistry::new();

    audio_player_actor
}

pub(crate) async fn get_player_actor_state(
    actor_ref: &ActorRef<AudioPlayerActor>,
) -> Result<PlayerStateQueryResult, String> {
    let state_query_result = actor_ref
        .ask(PlayerStateQuery {})
        .await
        .map_err(|e| e.to_string());
    state_query_result
}

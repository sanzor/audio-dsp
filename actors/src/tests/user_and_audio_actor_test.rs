use std::collections::HashMap;

use audiolib::{audio_buffer::AudioBuffer, Channels};
use domain::{
    actors::{
        player_state::AudioPlayerState, user_player_command::UserPlayerCommand,
        user_player_state_query::UserPlayerStateQuery,
        user_player_state_query_result::UserPlayerStateQueryResult,
    },
    dsp_message::DspMessage,
    track::{Track, TrackInfo},
};
use dsp_core::{command_processor::CommandProcessor, state::TracksState};
use kameo::{actor::ActorRef, spawn};

use crate::user_actor::user_actor::UserActor;

fn create_user_actor() -> ActorRef<UserActor> {
    let processor = CommandProcessor::create_processor();
    let tracks = TracksState::new();
    let players = HashMap::new();
    let actor = spawn(UserActor::new(processor, tracks, players));
    let g = kameo::registry::ActorRegistry::new();

    actor
}

#[tokio::test]
async fn can_create_player_and_play() -> Result<(), String> {
    let track_name = "some_track";
    let track = sample_track(track_name);
    let user_name = "my_user".to_string();
    let user_actor = create_user_actor();
    let insert_result = user_actor
        .ask(DspMessage::Insert {
            user_name: Some(user_name),
            track_payload: Some(to_string(&track)),
        })
        .await
        .map_err(|e| e.to_string())?;

    assert!(insert_result.output.contains("Inserted"));

    let play = user_actor
        .tell(UserPlayerCommand::Play {
            track_id: Some(track_name.to_string()),
        })
        .await;
    assert!(play.is_ok());
    let user_actor_state_result = get_state(&user_actor, track_name).await;
    assert!(user_actor_state_result.is_ok());
    assert!(matches!(
        user_actor_state_result?.state,
        AudioPlayerState::Playing
    ));

    Ok(())
}

fn sample_track(track_name: &str) -> Track {
    let samples = vec![1.1_f32; 500];
    let sample_rate = 1_f32;
    let track = Track {
        info: TrackInfo {
            name: track_name.to_string(),
        },
        data: AudioBuffer {
            channels: Channels::Mono,
            samples: samples,
            sample_rate: sample_rate,
        },
    };
    track
}
fn to_string(track: &Track) -> String {
    serde_json::to_string(track).unwrap()
}

async fn get_state(
    user_actor: &ActorRef<UserActor>,
    track_id: &str,
) -> Result<UserPlayerStateQueryResult, String> {
    let rez = user_actor
        .ask(UserPlayerStateQuery {
            track_id: Some(track_id.to_string()),
        })
        .await
        .map_err(|e| e.to_string())?;

    Ok(rez)
}

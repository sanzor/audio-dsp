use std::collections::HashMap;

use audiolib::{audio_buffer::AudioBuffer, Channels};
use domain::{
    actors::{
        player_state::AudioPlayerState, user_player_command::UserPlayerCommand,
        user_player_state_query::UserPlayerStateQuery,
        user_player_state_query_result::UserPlayerStateQueryResult,
        user_state_query::UserStateQuery, user_state_query_result::UserStateQueryResult,
    },
    dsp_message::DspMessage,
    track::{Track, TrackInfo},
};
use dsp_core::{command_processor::CommandProcessor, state::TracksState};
use kameo::{actor::ActorRef, spawn};
use ulid::Ulid;

use crate::user_actor::user_actor::{UserActor, UserActorParams};

fn create_user_actor(id:Ulid) -> ActorRef<UserActor> {
    let processor = CommandProcessor::create_processor();
    let tracks = TracksState::new();
    let players = HashMap::new();
    let actor_params=UserActorParams{
        id:id.to_string(),
        players:players,
        track_state:tracks,
        processor:processor
    };
    let actor = spawn(UserActor::new(actor_params));
    let g = kameo::registry::ActorRegistry::new();

    actor
}

#[tokio::test]
async fn can_create_player_and_play() -> Result<(), String> {
    let track_name = "some_track";
    let track = sample_track(track_name);
    let user_name = "my_user".to_string();
    let id=Ulid::new();
    let user_actor = create_user_actor(id);
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
    let user_actor_state_result = get_player_state(&user_actor, track_name).await;
    assert!(user_actor_state_result.is_ok());
    assert!(matches!(
        user_actor_state_result?.state,
        AudioPlayerState::Playing
    ));
    Ok(())
}

#[tokio::test]
async fn can_play_on_existing_player() -> Result<(), String> {
    let track_name = "some_track";
    let track = sample_track(track_name);
    let user_name = "my_user".to_string();
    let id=Ulid::new();
    let user_actor = create_user_actor(id);
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
    let user_actor_state_result = get_user_state(&user_actor).await?;
    assert!(matches!(user_actor_state_result.players.len(), 1));
    assert!(matches!(
        user_actor_state_result
            .players
            .get(track_name)
            .unwrap()
            .state,
        AudioPlayerState::Playing
    ));
    let pause = user_actor
        .tell(UserPlayerCommand::Pause {
            track_id: Some(track_name.to_string()),
        })
        .await;
    let play_again = user_actor
        .tell(UserPlayerCommand::Play {
            track_id: Some(track_name.to_string()),
        })
        .await;
    let user_actor_state_result = get_user_state(&user_actor).await?;
    assert_eq!(user_actor_state_result.players.len(), 1);
    Ok(())
}

#[tokio::test]
async fn can_create_player_and_stop() -> Result<(), String> {
    let track_name = "some_track";
    let track = sample_track(track_name);
    let user_name = "my_user".to_string();
    let id=Ulid::new();
    let user_actor = create_user_actor(id);
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
    let user_actor_state_result = get_player_state(&user_actor, track_name).await;
    assert!(user_actor_state_result.is_ok());
    assert!(matches!(
        user_actor_state_result?.state,
        AudioPlayerState::Playing
    ));
    let play = user_actor
        .tell(UserPlayerCommand::Stop {
            track_id: Some(track_name.to_string()),
        })
        .await;

    let state = get_player_state(&user_actor, track_name).await;
    assert!(state.is_err());
    assert!(state.unwrap_err().contains("Player does not exist"));
    let deleted = get_user_state(&user_actor).await?;
    assert!(deleted.players.len() == 0);
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

async fn get_player_state(
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
async fn get_user_state(user_actor: &ActorRef<UserActor>) -> Result<UserStateQueryResult, String> {
    let rez = user_actor
        .ask(UserStateQuery {})
        .await
        .map_err(|e| e.to_string())?;

    Ok(rez)
}

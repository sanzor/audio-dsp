use std::sync::Arc;

use audiolib::{audio_buffer::AudioBuffer, Channels};
use domain::{
    actors::{
        messages::{crud::insert_track::InsertTrack, user_to_player::{user_pause::UserPause, user_play::UserPlay}}, player_state::AudioPlayerState, user_player_command::UserPlayerCommand, user_player_state_query::UserPlayerStateQuery, user_player_state_query_result::UserPlayerStateQueryResult, user_state_query::UserStateQuery
    },
    track::{Track, TrackInfo},
};

use dsp_core::tracks_provider::LocalTrackStoreProvider;
use kameo::{actor::ActorRef, Actor};
use ulid::Ulid;

use crate::user_actor::{
    create_user_actor_params::CreateUserActorParams, create_user_data::CreateUserData,
    local_players_provider::LocalPlayerProvider, player_factory::PlayerFactory,
    user_actor::UserActor,
};

fn create_user_actor(id: Ulid) -> ActorRef<UserActor> {
    let tracks_provider = Box::new(LocalTrackStoreProvider::new());
    let players_provider = Box::new(LocalPlayerProvider::new());
    let actor_params = CreateUserActorParams {
        user_data: CreateUserData {
            email: id.to_string(),
            name: id.to_string(),
            id: id.to_string(),
        },
        players_provider: players_provider,
        tracks_provider,
        player_factory: Arc::new(PlayerFactory {}),
    };
    let actor = UserActor::spawn(UserActor::new(actor_params));
    let g = kameo::registry::ActorRegistry::new();

    actor
}

#[tokio::test]
async fn can_create_player_and_play() -> Result<(), String> {
    let track_id = "some_track";
    let track = sample_track(track_id);
    let user_name = "my_user".to_string();
    let id = Ulid::new();
    let user_actor = create_user_actor(id);
    let insert_result = user_actor
        .ask(InsertTrack{track:track })
        .await
        .map_err(|e| e.to_string())?;

    

    let play = user_actor
        .tell(UserPlay {player_id:track_id.into()})
        .await;
    assert!(play.is_ok());
    let user_actor_state_result = get_player_state(&user_actor, track_id).await;
    assert!(user_actor_state_result.is_ok());
    assert!(matches!(
        user_actor_state_result?.state,
        AudioPlayerState::Playing
    ));
    Ok(())
}

#[tokio::test]
async fn can_play_on_existing_player() -> Result<(), String> {
    let track_id = "some_track";
    let track = sample_track(track_id);
    let user_name = "my_user".to_string();
    let id = Ulid::new();
    let user_actor = create_user_actor(id);
    let insert_result = user_actor
        .ask(InsertTrack {track:track
        })
        .await
        .map_err(|e| e.to_string())?;

   

    let play = user_actor
        .tell(UserPlay {
            player_id:track_id.into()
        })
        .await;
    let user_actor_state_result = get_user_state(&user_actor).await?;
    assert!(matches!(user_actor_state_result.players.len(), 1));
    assert!(matches!(
        user_actor_state_result
            .players
            .get(track_id)
            .unwrap()
            .state,
        AudioPlayerState::Playing
    ));
    let pause = user_actor
        .tell(UserPause{
            track_id: track_id.into(),
        })
        .await;
    let play_again = user_actor
        .tell(UserPlay {
            player_id:track_id.into()
        })
        .await;
    let user_actor_state_result = get_user_state(&user_actor).await?;
    assert_eq!(user_actor_state_result.players.len(), 1);
    Ok(())
}

#[tokio::test]
async fn can_create_player_and_stop() -> Result<(), String> {
    let track_id = "some_track";
    let track = sample_track(track_id);
    let user_name = "my_user".to_string();
    let id = Ulid::new();
    let user_actor = create_user_actor(id);
    let insert_result = user_actor
        .ask(InsertTrack {
            track:track
        })
        .await
        .map_err(|e| e.to_string())?;

  

    let play = user_actor
        .tell(UserPlay {
            player_id: track_id.into(),
        })
        .await;
    assert!(play.is_ok());
    let user_actor_state_result = get_player_state(&user_actor, track_id).await;
    assert!(user_actor_state_result.is_ok());
    assert!(matches!(
        user_actor_state_result?.state,
        AudioPlayerState::Playing
    ));
    let play = user_actor
        .tell(UserPause {
            track_id:track_id.into()
        })
        .await;

    let state = get_player_state(&user_actor, track_id).await;
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
            track_id: track_id.into(),
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

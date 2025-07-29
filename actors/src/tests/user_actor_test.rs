use std::sync::Arc;

use crate::user_actor::create_user_actor_params::CreateUserActorParams;
use crate::user_actor::create_user_data::CreateUserData;
use crate::user_actor::local_players_provider::LocalPlayerProvider;
use crate::user_actor::player_factory::PlayerFactory;
use crate::user_actor::user_actor::UserActor;
use audiolib::{self, audio_buffer::AudioBuffer, Channels};
use domain::actors::messages::crud::copy_track::CopyTrack;
use domain::actors::messages::crud::get_tracks::GetTrackMetas;
use domain::actors::messages::crud::insert_track::{InsertTrack, InsertTrackResult};
use domain::raw_track::{RawTrack, TrackInfo};
use domain::track_meta::TrackMeta;
use dsp_core::tracks_provider::LocalTrackStoreProvider;
use kameo::actor::ActorRef;
use kameo::{self, Actor};
use ulid::Ulid;

fn create_actor(id: Ulid) -> ActorRef<UserActor> {
    let tracks_provider = Box::new(LocalTrackStoreProvider::new());
    let players_provder = LocalPlayerProvider::new();
    let actor_params = CreateUserActorParams {
        user_data: CreateUserData {
            email: id.to_string(),
            id: id.to_string(),
            name: id.to_string(),
        },
        user_actor_deps:UserActorDeps{
            
        }
    };
    let actor = UserActor::spawn(UserActor::new(actor_params));
    let g = kameo::registry::ActorRegistry::new();

    actor
}
#[tokio::test]
async fn can_run_insert() -> Result<(), String> {
    let user_name = "some_user".to_string();
    let track_name = "some_track".to_string();
    let id = Ulid::new();
    let samples = vec![1.1_f32; 500];
    let sample_rate = 1_f32;
    let track = RawTrack {
        info: TrackInfo { name: track_name },
        data: AudioBuffer {
            channels: Channels::Mono,
            samples: samples,
            sample_rate: sample_rate,
        },
    };
    let command = InsertTrack { track: track };
    let addr = create_actor(id);
    let rez = addr.ask(command).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tokio::test]
async fn can_run_copy() -> Result<(), String> {
    let user_name = "some_user".to_string();
    let track_name = "some_track".to_string();
    let copy_track_name = "some_other_track".to_string();
    let samples = vec![1.1_f32; 500];
    let sample_rate = 1_f32;
    let track = RawTrack {
        info: TrackInfo {
            name: track_name.clone(),
        },
        data: AudioBuffer {
            channels: Channels::Mono,
            samples: samples,
            sample_rate: sample_rate,
        },
    };
    let id = Ulid::new();
    let addr = create_actor(id);
    let track_result = insert_track_command(&addr, &user_name, track).await?;
    let after_insert_list = list_command(&addr, &user_name.clone()).await?;
    assert_eq!(after_insert_list.len(), 1);

    let copy_command = CopyTrack {
        track_id: track_result.track_id,
        track_copy_name: copy_track_name,
    };
    let _ = addr.ask(copy_command).await.map_err(|e| e.to_string())?;
    let after_copy_list = list_command(&addr, &user_name).await?;
    assert_eq!(after_copy_list.len(), 2);
    Ok(())
}

#[tokio::test]
async fn can_run_list() -> Result<(), String> {
    let user_name = "some_user".to_string();
    let track_name = "some_track".to_string();
    let samples = vec![1.1_f32; 500];
    let sample_rate = 1_f32;
    let track = RawTrack {
        info: TrackInfo { name: track_name },
        data: AudioBuffer {
            channels: Channels::Mono,
            samples: samples,
            sample_rate: sample_rate,
        },
    };
    let id = Ulid::new();
    let addr = create_actor(id);
    let initial_list = list_command(&addr, &user_name.clone()).await?;
    assert_eq!(initial_list.len(), 0);
    let insert_result = insert_track_command(&addr, &user_name, track).await?;
    let after_list = list_command(&addr, &user_name.clone()).await?;
    assert_eq!(after_list.len(), 1);
    Ok(())
}

async fn insert_track_command(
    addr: &ActorRef<UserActor>,
    user_name: &str,
    track: RawTrack,
) -> Result<InsertTrackResult, String> {
    let command = InsertTrack { track };
    let rez = addr.ask(command).await.map_err(|e| e.to_string());
    rez
}

async fn list_command(
    addr: &ActorRef<UserActor>,
    user_name: &str,
) -> Result<Vec<TrackMeta>, String> {
    let rez = addr
        .ask(GetTrackMetas {})
        .await
        .map_err(|e| e.to_string())?;

    Ok(rez.tracks)
}

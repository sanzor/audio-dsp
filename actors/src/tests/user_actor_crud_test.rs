use std::sync::Arc;

use crate::user_actor::create_user_actor_params::CreateUserActorParams;

use crate::user_actor::actor::UserActor;
use crate::user_actor::player_factory::PlayerFactory;
use crate::user_actor::user_actor_deps::UserActorDeps;
use audiolib::{self, audio_buffer::AudioBuffer, Channels};
use data_provider::in_memory_region_set_provider::InMemoryRegionSetProvider;
use data_provider::tracks_provider::LocalTrackStoreProvider;
use domain::actors::messages::tracks::copy_track::CopyTrack;
use domain::actors::messages::tracks::get_tracks::GetTrackMetas;
use domain::actors::messages::tracks::insert_track::{InsertTrack, InsertTrackResult};
use domain::domain_user::DomainUser;
use domain::raw_track::{RawTrack, TrackInfo};
use domain::track_meta::TrackMeta;
use kameo::actor::{ActorRef, Spawn};
use ulid::Ulid;

fn create_actor(id: Ulid) -> ActorRef<UserActor> {
    let actor_params = CreateUserActorParams {
        user_data: DomainUser {
            email: id.to_string(),
            id: id.to_string(),
            name: id.to_string(),
            google_sub_id: None,
            picture: "some".into(),
        },
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
            region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
        }),
    };
    UserActor::spawn(UserActor::new(actor_params))
}
#[tokio::test]
async fn can_run_insert() -> Result<(), String> {
    let track_name = "some_track".to_string();
    let id = Ulid::new();
    let samples = vec![1.1_f32; 500];
    let sample_rate = 1_f32;
    let track = RawTrack {
        info: TrackInfo {
            name: track_name,
            extension: "wav".to_string(),
            length: samples.len() as f32 / sample_rate,
        },
        data: AudioBuffer {
            channels: Channels::Mono,
            samples,
            sample_rate,
        },
    };
    let command = InsertTrack { track };
    let addr = create_actor(id);
    let _: InsertTrackResult = addr.ask(command).await.map_err(|e| e.to_string())?;

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
            extension: "wav".to_string(),
            length: samples.len() as f32 / sample_rate,
        },
        data: AudioBuffer {
            channels: Channels::Mono,
            samples,
            sample_rate,
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
        info: TrackInfo {
            name: track_name,
            extension: "wav".to_string(),
            length: samples.len() as f32 / sample_rate,
        },
        data: AudioBuffer {
            channels: Channels::Mono,
            samples,
            sample_rate,
        },
    };
    let id = Ulid::new();
    let addr = create_actor(id);
    let initial_list = list_command(&addr, &user_name.clone()).await?;
    assert_eq!(initial_list.len(), 0);
    let _ = insert_track_command(&addr, &user_name, track).await?;
    let after_list = list_command(&addr, &user_name.clone()).await?;
    assert_eq!(after_list.len(), 1);
    Ok(())
}

async fn insert_track_command(
    addr: &ActorRef<UserActor>,
    _user_name: &str,
    track: RawTrack,
) -> Result<InsertTrackResult, String> {
    let command = InsertTrack { track };
    let rez = addr.ask(command).await.map_err(|e| e.to_string());
    rez
}

async fn list_command(
    addr: &ActorRef<UserActor>,
    _user_name: &str,
) -> Result<Vec<TrackMeta>, String> {
    let rez = addr
        .ask(GetTrackMetas {})
        .await
        .map_err(|e| e.to_string())?;

    Ok(rez.tracks)
}

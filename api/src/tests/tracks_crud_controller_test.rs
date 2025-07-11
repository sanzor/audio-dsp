use std::{collections::HashMap, sync::Arc};

use actix_http::{Request, StatusCode};
use actix_web::{dev::Service, test, web, App};
use actors::user_actor::{
    create_user_actor_params::CreateUserActorParams, create_user_data::CreateUserData,
    local_players_provider::LocalPlayerProvider, player_factory::PlayerFactory,
    user_actor::UserActor,
};
use audiolib::{audio_buffer::AudioBuffer, Channels};
use domain::{
    actors::messages::crud::{get_track::GetRawTrackResult, get_track_info::GetTrackMetaResult, get_tracks::GetTracksResult},
    raw_track::{RawTrack, TrackInfo},
};
use dsp_core::tracks_provider::LocalTrackStoreProvider;
use kameo::{actor::ActorRef, Actor};
use rstest::rstest;
use tokio::sync::Mutex;
use ulid::Ulid;

use crate::{
    app_data::AppData,
    controllers::tracks_crud_controller::{self, AddTrackParams, AddTrackResult},
};

#[rstest]
#[actix_web::test]
async fn can_insert_track() -> Result<(), String> {
    let track = make_track_from_samples(vec![1_f32; 500], Channels::Mono);
    let user_id = Ulid::new();
    let user_actor = create_actor(user_id.clone());
    let mut user_map = HashMap::new();
    user_map.insert(user_id.to_string(), user_actor);
    let app_data = AppData {
        player_factory: Arc::new(PlayerFactory {}),
        user_map: Arc::new(Mutex::new(user_map)),
    };
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/tracks/add-track")
        .set_json(&AddTrackParams {
            track,
            user_id: user_id.to_string(),
        })
        .to_request();
    let resp: AddTrackResult = test::call_and_read_body_json(&app, req).await;
    assert!(resp.user_id == user_id.to_string());
    Ok(())
}

#[rstest]
#[actix_web::test]
async fn can_get_track_metas() -> Result<(), String> {
    let track = make_track_from_samples(vec![1_f32; 500], Channels::Mono);

    let user_id = Ulid::new();
    let user_actor = create_actor(user_id.clone());
    let mut user_map = HashMap::new();
    user_map.insert(user_id.to_string(), user_actor);
    let app_data = AppData {
        player_factory: Arc::new(PlayerFactory {}),
        user_map: Arc::new(Mutex::new(user_map)),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;
    let insert1 = AddTrackParams {
        track: track.clone(),
        user_id: user_id.to_string(),
    };
    let insert2 = AddTrackParams {
        track: track,
        user_id: user_id.to_string(),
    };
    let insert1_result = insert_track(&mut app, insert1).await?;
    let insert2_result = insert_track(&mut app, insert2).await?;
    let req = test::TestRequest::get()
        .uri(&format!("/tracks/get-all?user_id={}", user_id))
        .to_request();
    let resp: GetTracksResult = test::call_and_read_body_json(&app, req).await;
    assert!(resp.tracks.len() == 2);
    let ids: Vec<_> = resp.tracks.iter().map(|t| t.track_id.to_string()).collect();
    assert!(ids.contains(&insert1_result.track_id) && ids.contains(&insert2_result.track_id));
    Ok(())
}


#[rstest]
#[actix_web::test]
async fn can_get_track_meta() -> Result<(), String> {
    let track = make_track_from_samples(vec![1_f32; 500], Channels::Mono);

    let user_id = Ulid::new();
    let user_actor = create_actor(user_id.clone());
    let mut user_map = HashMap::new();
    user_map.insert(user_id.to_string(), user_actor);
    let app_data = AppData {
        player_factory: Arc::new(PlayerFactory {}),
        user_map: Arc::new(Mutex::new(user_map)),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;
    let insert = AddTrackParams {
        track: track.clone(),
        user_id: user_id.to_string(),
    };
   
    let insert_result = insert_track(&mut app, insert).await?;
  
    let req = test::TestRequest::get()
        .uri(&format!("/tracks/get-meta?user_id={}&track_id={}", user_id,insert_result.track_id))
        .to_request();
    let resp: GetTrackMetaResult = test::call_and_read_body_json(&app, req).await;
    assert!(resp.track_meta.track_id==insert_result.track_id);
    assert!(resp.track_meta.track_info.name==track.info.name);
    Ok(())
}


#[rstest]
#[actix_web::test]
async fn can_get_track_raw() -> Result<(), String> {
    let track = make_track_from_samples(vec![1_f32; 500], Channels::Mono);

    let user_id = Ulid::new();
    let user_actor = create_actor(user_id.clone());
    let mut user_map = HashMap::new();
    user_map.insert(user_id.to_string(), user_actor);
    let app_data = AppData {
        player_factory: Arc::new(PlayerFactory {}),
        user_map: Arc::new(Mutex::new(user_map)),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;
    let insert = AddTrackParams {
        track: track.clone(),
        user_id: user_id.to_string(),
    };
   
    let insert_result = insert_track(&mut app, insert).await?;
  
    let req = test::TestRequest::get()
        .uri(&format!("/tracks/get-raw?user_id={}&track_id={}", user_id,insert_result.track_id))
        .to_request();
    let resp: GetRawTrackResult = test::call_and_read_body_json(&app, req).await;
    assert!(resp.track.info.name==track.info.name);
    assert!(resp.track.data.samples.len()==track.data.samples.len());
    Ok(())
}

#[rstest]
#[actix_web::test]
async fn can_remove_track() -> Result<(), String> {
    let track = make_track_from_samples(vec![1_f32; 500], Channels::Mono);
    let user_id = Ulid::new();
    let user_actor = create_actor(user_id.clone());
    let mut user_map = HashMap::new();
    user_map.insert(user_id.to_string(), user_actor);
    let app_data = AppData {
        player_factory: Arc::new(PlayerFactory {}),
        user_map: Arc::new(Mutex::new(user_map)),
    };
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;

    let insert_request = test::TestRequest::post()
        .uri("/tracks/add-track")
        .set_json(&AddTrackParams {
            track,
            user_id: user_id.to_string(),
        })
        .to_request();
    let resp: AddTrackResult = test::call_and_read_body_json(&app, insert_request).await;
    assert!(resp.user_id == user_id.to_string());

    let remove_request = test::TestRequest::delete()
        .uri(&format!("/tracks/remove?user_id={}&track_id={}",user_id, resp.track_id))
        .to_request();
    let resp = test::call_service(&app, remove_request).await;
    let status = resp.status();
    assert!(matches!(resp.status(), StatusCode::OK));
    Ok(())
}

fn make_track_from_samples(samples: Vec<f32>, channels: Channels) -> RawTrack {
    match channels {
        Channels::Mono => RawTrack {
            info: TrackInfo {
                name: "some_name".to_string(),
            },
            data: AudioBuffer {
                channels: Channels::Mono,
                sample_rate: 1_f32,
                samples: samples.clone(),
            },
        },
        Channels::Stereo => RawTrack {
            info: TrackInfo {
                name: "some_name".to_string(),
            },
            data: AudioBuffer {
                samples: samples.clone(),
                sample_rate: 1_f32,
                channels: Channels::Stereo,
            },
        },
    }
}

fn create_actor(id: Ulid) -> ActorRef<UserActor> {
    let tracks_provider = Box::new(LocalTrackStoreProvider::new());
    let players_provder = LocalPlayerProvider::new();
    let actor_params = CreateUserActorParams {
        user_data: CreateUserData {
            email: id.to_string(),
            id: id.to_string(),
            name: id.to_string(),
        },
        tracks_provider: tracks_provider,
        players_provider: Box::new(players_provder),
        player_factory: Arc::new(PlayerFactory {}),
    };
    let actor = UserActor::spawn(UserActor::new(actor_params));

    let g = kameo::registry::ActorRegistry::new();

    actor
}

async fn get_tracks(
    app: &mut impl Service<
        Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
) -> Result<GetTracksResult, String> {
    let req = test::TestRequest::get().uri("/tracks/get-all").to_request();
    let resp: GetTracksResult = test::call_and_read_body_json(&app, req).await;
    Ok(resp)
}

async fn insert_track(
    app: &mut impl Service<
        Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    track_params: AddTrackParams,
) -> Result<AddTrackResult, String> {
    let req = test::TestRequest::post()
        .uri("/tracks/add-track")
        .set_json(track_params)
        .to_request();
    let resp: AddTrackResult = test::call_and_read_body_json(&app, req).await;
    Ok(resp)
}

async fn remove_track(
    app: &mut impl Service<
        Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    track_id: &str,
) -> Result<(), String> {
    let remove_request = test::TestRequest::delete()
        .uri(&format!("/tracks/remove/{}", track_id))
        .to_request();
    let resp = test::call_service(&app, remove_request).await;
    let status = resp.status();
    assert!(matches!(resp.status(), StatusCode::OK));
    Ok(())
}

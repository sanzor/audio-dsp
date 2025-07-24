use std::sync::Arc;

use actix_http::{Request, StatusCode};
use actix_web::{dev::Service, test};
use actors::user_actor::{
    create_user_actor_params::CreateUserActorParams, create_user_data::CreateUserData,
    local_players_provider::LocalPlayerProvider, player_factory::PlayerFactory,
    user_actor::UserActor,
};
use audiolib::{audio_buffer::AudioBuffer, Channels};
use domain::{
    actors::messages::crud::get_tracks::GetTracksResult,
    raw_track::{RawTrack, TrackInfo},
};
use dsp_core::tracks_provider::LocalTrackStoreProvider;
use kameo::{actor::ActorRef, Actor};
use ulid::Ulid;

use crate::controllers::{
    tracks_crud_controller::{AddTrackParams, AddTrackResult},
    user_controller::GetUserDataResult,
};

pub fn create_user_actor(id: Ulid) -> ActorRef<UserActor> {
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

pub async fn get_tracks(
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

pub async fn insert_track(
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

pub async fn remove_track(
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

pub fn make_raw_track_from_samples(samples: Vec<f32>, channels: Channels) -> RawTrack {
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
pub async fn get_user_state(
    app: &mut impl Service<
        Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    user_id: String,
) -> Result<GetUserDataResult, String> {
    let get_user_state_request = actix_web::test::TestRequest::get()
        .uri(&format!("/user/get-user-state/{}", user_id.to_string()))
        .to_request();

    let user_state_result: GetUserDataResult =
        test::call_and_read_body_json(&app, get_user_state_request).await;
    Ok(user_state_result)
}

use std::sync::Arc;

use actix_http::{Request, StatusCode};
use actix_web::{cookie::Cookie, dev::Service, http::header, test};
use actors::user_actor::{
    actor::UserActor, create_user_actor_params::CreateUserActorParams,
    player_factory::PlayerFactory, user_actor_deps::UserActorDeps,
};
use audiolib::{audio_buffer::AudioBuffer, Channels};
use data_provider::tracks::tracks_provider::TracksProvider;
use domain::{
    actors::messages::tracks::get_tracks::GetTracksResult,
    domain_user::DomainUser,
    raw_track::{RawTrack, TrackInfo},
};
use kameo::actor::{ActorRef, Spawn};
use ulid::Ulid;

use crate::controllers::{
    tracks_crud_controller::{AddTrackParams, AddTrackResult},
    user_controller::GetUserDataResult,
};
use crate::tests::test_providers::{StubRegionSetsProvider, StubTracksProvider};

pub fn create_user_actor(id: Ulid) -> ActorRef<UserActor> {
    let tracks_provider: Arc<dyn TracksProvider + Send + Sync> = Arc::new(StubTracksProvider::new());
    let player_factory = PlayerFactory {};
    let actor_params = CreateUserActorParams {
        user_data: DomainUser {
            email: id.to_string(),
            id: id.to_string(),
            name: id.to_string(),
            google_sub_id: None,
            picture: "Some str".into(),
        },
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(player_factory),
            tracks_provider: Arc::clone(&tracks_provider),
            region_sets_provider: Arc::new(StubRegionSetsProvider::new()),
        }),
    };
    let actor = UserActor::spawn(UserActor::new(actor_params));

    let _g = kameo::registry::ActorRegistry::new();
    actor
}

pub async fn get_tracks(
    cookie: Cookie<'static>,
    app: &mut impl Service<
        Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
) -> Result<GetTracksResult, String> {
    let req = test::TestRequest::get()
        .uri("/tracks/get-all")
        .insert_header((
            header::COOKIE,
            format!("{}={}", cookie.name(), cookie.value()),
        ))
        .to_request();
    let resp: GetTracksResult = test::call_and_read_body_json(&app, req).await;
    Ok(resp)
}

pub async fn insert_track(
    cookie: Cookie<'static>,
    app: &mut impl Service<
        Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    track_params: AddTrackParams,
) -> Result<AddTrackResult, String> {
    let req = test::TestRequest::post()
        .uri("/tracks/add-track")
        .set_json(&AddTrackParams {
            track: track_params.track,
        })
        .cookie(cookie)
        .to_request();

    let resp: AddTrackResult = test::call_and_read_body_json(&app, req).await;
    Ok(resp)
}

pub async fn remove_track(
    cookie: Cookie<'static>,
    app: &mut impl Service<
        Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    track_id: &str,
) -> Result<(), String> {
    let remove_request = test::TestRequest::delete()
        .uri(&format!("/tracks/remove/{}", track_id))
        .insert_header((
            header::COOKIE,
            format!("{}={}", cookie.name(), cookie.value()),
        ))
        .to_request();
    let resp = test::call_service(&app, remove_request).await;
    assert!(matches!(resp.status(), StatusCode::OK));
    Ok(())
}

pub fn make_raw_track_from_samples(samples: Vec<f32>, channels: Channels) -> RawTrack {
    match channels {
        Channels::Mono => RawTrack {
            info: TrackInfo {
                name: "some_name".to_string(),
                extension: "wav".to_string(),
                length: samples.len() as f32 / 1_f32,
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
                extension: "wav".to_string(),
                length: samples.len() as f32 / 1_f32,
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
    cookie: Cookie<'static>,
    app: &mut impl Service<
        Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    user_id: String,
) -> Result<GetUserDataResult, String> {
    let get_user_state_request = actix_web::test::TestRequest::get()
        .uri(&format!("/user/get-user-state/{}", user_id))
        .insert_header((
            header::COOKIE,
            format!("{}={}", cookie.name(), cookie.value()),
        ))
        .to_request();

    let user_state_result: GetUserDataResult =
        test::call_and_read_body_json(&app, get_user_state_request).await;
    Ok(user_state_result)
}

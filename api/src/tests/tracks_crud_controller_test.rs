use std::{collections::HashMap, sync::Arc};

use actix_http::StatusCode;
use actix_web::{test, web, App};
use actors::user_actor::{
    create_user_actor_params::CreateUserActorParams, player_factory::PlayerFactory,
    user_actor::UserActor, user_actor_deps::UserActorDeps, user_actor_registry::UserActorRegistry,
};
use audiolib::Channels;
use data_provider::{
    in_memory_user_provider::InMemoryUserProvider, tracks_provider::LocalTrackStoreProvider,
    user_provider::UserProvider,
};
use domain::{
    actors::messages::crud::{
        get_track::GetRawTrackResult, get_track_info::GetTrackMetaResult,
        get_tracks::GetTracksResult,
    },
    domain_user::DomainUser,
};

use kameo::{actor::ActorRef, Actor};
use rstest::rstest;
use ulid::Ulid;

use crate::{
    app_data::AppData,
    controllers::tracks_crud_controller::{self, AddTrackParams, AddTrackResult},
    player_controller_test::utils::{insert_track, make_raw_track_from_samples},
    token::token_utils::{create_access_token, create_token},
    user_and_actor_resolver::local_user_and_actor_resolver::LocalUserAndActorResolver,
};

#[rstest]
#[actix_web::test]
async fn can_insert_track() -> Result<(), String> {
    let raw_track = make_raw_track_from_samples(vec![1_f32; 500], Channels::Mono);
    let user_id = Ulid::new();
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
    });
    let user_actor = create_user_actor(user_id.clone(), Arc::clone(&user_actor_deps));
    let mut user_map = HashMap::new();
    user_map.insert(user_id.to_string(), user_actor);
    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(
            user_provider,
            Arc::new(UserActorRegistry::new()),
        )),
    };
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/tracks/add-track")
        .set_json(&AddTrackParams { track: raw_track })
        .to_request();
    let resp: AddTrackResult = test::call_and_read_body_json(&app, req).await;
    Ok(())
}

#[rstest]
#[actix_web::test]
async fn can_get_track_metas() -> Result<(), String> {
    let track = make_raw_track_from_samples(vec![1_f32; 500], Channels::Mono);

    let user_id = Ulid::new();
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
    });
    let user_actor = create_user_actor(user_id.clone(), Arc::clone(&user_actor_deps));
    let mut user_map = HashMap::new();
    user_map.insert(user_id.to_string(), user_actor);
    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(
            user_provider,
            Arc::new(UserActorRegistry::new()),
        )),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;
    let insert1 = AddTrackParams {
        track: track.clone(),
    };
    let insert2 = AddTrackParams { track: track };
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
    let track = make_raw_track_from_samples(vec![1_f32; 500], Channels::Mono);

    let user_id = Ulid::new();
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
    });
    let user_actor = create_user_actor(user_id.clone(), Arc::clone(&user_actor_deps));
    let mut user_map = HashMap::new();
    user_map.insert(user_id.to_string(), user_actor);
    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(
            user_provider,
            Arc::new(UserActorRegistry::new()),
        )),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;
    let insert = AddTrackParams {
        track: track.clone(),
    };

    let insert_result = insert_track(&mut app, insert).await?;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/tracks/get-meta?user_id={}&track_id={}",
            user_id, insert_result.track_id
        ))
        .to_request();
    let resp: GetTrackMetaResult = test::call_and_read_body_json(&app, req).await;
    assert!(resp.track_meta.track_id == insert_result.track_id);
    assert!(resp.track_meta.track_info.name == track.info.name);
    Ok(())
}

#[rstest]
#[actix_web::test]
async fn can_get_track_raw() -> Result<(), String> {
    let track = make_raw_track_from_samples(vec![1_f32; 500], Channels::Mono);

    let user_id = Ulid::new();
    let email = "some@yahoo.com";
    let token = create_test_token(&user_id.to_string(), email);
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
    });
    let user_actor = create_user_actor(user_id.clone(), Arc::clone(&user_actor_deps));

    let mut user_map = HashMap::new();
    user_map.insert(user_id.to_string(), user_actor);
    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());

    let app_data = AppData {
        user_actor_deps: user_actor_deps,
        user_resolver: Arc::new(LocalUserAndActorResolver::new(
            user_provider,
            Arc::new(UserActorRegistry::new()),
        )),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;
    let insert = AddTrackParams {
        track: track.clone(),
    };

    let insert_result = insert_track(&mut app, insert).await?;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/tracks/get-raw?user_id={}&track_id={}",
            user_id, insert_result.track_id
        ))
        .insert_header(("Cookie", format!("auth_token={}", token)))
        .to_request();
    let resp: GetRawTrackResult = test::call_and_read_body_json(&app, req).await;
    assert!(resp.track.info.name == track.info.name);
    assert!(resp.track.data.samples.len() == track.data.samples.len());
    Ok(())
}

#[rstest]
#[actix_web::test]
async fn can_remove_track() -> Result<(), String> {
    let track = make_raw_track_from_samples(vec![1_f32; 500], Channels::Mono);
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
    });
    let user_id = Ulid::new();
    let email = "some@gmail.com";
    let token = create_test_token(&user_id.to_string(), email);
    let user_actor = create_user_actor(user_id.clone(), Arc::clone(&user_actor_deps));
    let mut user_map = HashMap::new();
    user_map.insert(user_id.to_string(), user_actor);
    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());

    let app_data = AppData {
        user_resolver: Arc::new(LocalUserAndActorResolver::new(
            user_provider,
            Arc::new(UserActorRegistry::new()),
        )),
        user_actor_deps: user_actor_deps,
    };
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;

    let insert_request = test::TestRequest::post()
        .uri("/tracks/add-track")
        .insert_header(("Cookie", format!("auth_token={}", token)))
        .set_json(&AddTrackParams { track })
        .to_request();
    let resp: AddTrackResult = test::call_and_read_body_json(&app, insert_request).await;

    let remove_request = test::TestRequest::delete()
        .uri(&format!(
            "/tracks/remove?user_id={}&track_id={}",
            user_id, resp.track_id
        ))
        .to_request();
    let resp = test::call_service(&app, remove_request).await;
    let status = resp.status();
    assert!(matches!(resp.status(), StatusCode::OK));
    Ok(())
}

fn create_user_actor(id: Ulid, user_actor_deps: Arc<UserActorDeps>) -> ActorRef<UserActor> {
    let actor_params = CreateUserActorParams {
        user_data: DomainUser {
            email: id.to_string(),
            id: id.to_string(),
            name: id.to_string(),
            google_sub_id: None,
            picture: "Some pic".into(),
        },
        user_actor_deps: Arc::clone(&user_actor_deps),
    };

    let actor = UserActor::spawn(UserActor::new(actor_params));
    actor
}

fn create_test_token(user_id: &str, email: &str) -> String {
    create_access_token(user_id, Some(email), None)
}

use std::{env, sync::Arc};

use actix_http::StatusCode;
use actix_web::{
    http::header,
    test,
    web::{self, Bytes},
    App,
};
use actors::user_actor::{
    player_factory::PlayerFactory, user_actor_deps::UserActorDeps,
    user_actor_registry::UserActorRegistry,
};
use audiolib::Channels;
use data_provider::{
    in_memory_region_set_provider::InMemoryRegionSetProvider,
    in_memory_user_provider::InMemoryUserProvider, tracks_provider::LocalTrackStoreProvider,
    user_provider::UserProvider,
};
use domain::actors::messages::tracks::{
    get_track_info::GetTrackMetaResult, get_tracks::GetTracksResult,
};

use rstest::rstest;
use ulid::Ulid;

use crate::{
    app_data::AppData,
    controllers::tracks_crud_controller::{self, AddTrackParams, AddTrackResult},
    player_controller_test::{
        controllers::utils::{create_user_and_actor, init_env, make_test_auth_cookie},
        utils::{insert_track, make_raw_track_from_samples},
    },
    user_and_actor_resolver::local_user_and_actor_resolver::LocalUserAndActorResolver,
};

#[rstest]
#[actix_web::test]
async fn can_insert_track() -> Result<(), String> {
    init_env();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let user_name = Ulid::new();
    let raw_track = make_raw_track_from_samples(vec![1_f32; 500], Channels::Mono);
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
        region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
    });

    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());

    let (user, _) = create_user_and_actor(
        user_name.to_string(),
        user_registry.clone(),
        user_provider.clone(),
        user_actor_deps.clone(),
    )
    .await?;

    let app_data = AppData {
        user_actor_deps: Arc::clone(&user_actor_deps),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(user_provider, user_registry)),
    };

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;
    let cookie = make_test_auth_cookie(secret, user.id);
    let req = test::TestRequest::post()
        .uri("/tracks/add-track")
        .set_json(&AddTrackParams { track: raw_track })
        .cookie(cookie)
        .to_request();
    let _resp: AddTrackResult = test::call_and_read_body_json(&app, req).await;

    Ok(())
}

#[rstest]
#[actix_web::test]
async fn can_get_track_metas() -> Result<(), String> {
    let track = make_raw_track_from_samples(vec![1_f32; 500], Channels::Mono);
    init_env();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let user_name = Ulid::new();
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
        region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
    });

    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());

    let (created_user, _) = create_user_and_actor(
        user_name.to_string(),
        user_registry.clone(),
        user_provider.clone(),
        user_actor_deps.clone(),
    )
    .await?;

    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
            region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(user_provider, user_registry)),
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
    let insert2 = AddTrackParams { track };
    let cookie = make_test_auth_cookie(secret, created_user.id.to_string());
    let insert1_result = insert_track(cookie.clone(), &mut app, insert1).await?;
    let insert2_result = insert_track(cookie.clone(), &mut app, insert2).await?;

    let req = test::TestRequest::get()
        .uri(&format!("/tracks/get-all?user_id={}", created_user.id))
        .insert_header((
            header::COOKIE,
            format!("{}={}", cookie.name(), cookie.value()),
        ))
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
    init_env();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let user_name = Ulid::new();
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
        region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
    });

    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());

    let (created_user, _) = create_user_and_actor(
        user_name.to_string(),
        user_registry.clone(),
        user_provider.clone(),
        user_actor_deps.clone(),
    )
    .await?;

    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
            region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(user_provider, user_registry)),
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
    let cookie = make_test_auth_cookie(secret, created_user.id.to_string());
    let insert_result = insert_track(cookie.clone(), &mut app, insert).await?;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/tracks/get-meta?user_id={}&track_id={}",
            created_user.id, insert_result.track_id
        ))
        .insert_header((
            header::COOKIE,
            format!("{}={}", cookie.name(), cookie.value()),
        ))
        .to_request();
    let resp: GetTrackMetaResult = test::call_and_read_body_json(&app, req).await;
    assert!(resp.track_meta.track_id == insert_result.track_id);
    assert!(resp.track_meta.track_info.name == track.info.name);
    Ok(())
}

#[rstest]
#[actix_web::test]
async fn can_get_stored_track() -> Result<(), String> {
    let track = make_raw_track_from_samples(vec![1_f32; 500], Channels::Mono);

    init_env();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let user_name = Ulid::new();
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
        region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
    });

    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());

    let (created_user, _) = create_user_and_actor(
        user_name.to_string(),
        user_registry.clone(),
        user_provider.clone(),
        user_actor_deps.clone(),
    )
    .await?;

    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
            region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(user_provider, user_registry)),
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;
    let cookie = make_test_auth_cookie(secret, created_user.id.to_string());
    let insert = AddTrackParams {
        track: track.clone(),
    };

    let insert_result = insert_track(cookie.clone(), &mut app, insert).await?;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/tracks/get-stored-track?user_id={}&track_id={}",
            created_user.id, insert_result.track_id
        ))
        .insert_header((
            header::COOKIE,
            format!("{}={}", cookie.name(), cookie.value()),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert success

    // ✅ Read body as raw bytes (Vec<u8>)
    let body: Bytes = test::read_body(resp).await;

    // You can turn it into Vec<u8> like this:
    let bytes: Vec<u8> = body.to_vec();
    assert!(!bytes.is_empty());
    Ok(())
}

#[rstest]
#[actix_web::test]
async fn can_remove_track() -> Result<(), String> {
    let track = make_raw_track_from_samples(vec![1_f32; 500], Channels::Mono);
    init_env();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let user_name = Ulid::new();
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
        region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
    });

    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());

    let (created_user, _) = create_user_and_actor(
        user_name.to_string(),
        user_registry.clone(),
        user_provider.clone(),
        user_actor_deps.clone(),
    )
    .await?;

    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
            region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(user_provider, user_registry)),
    };
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/tracks").configure(tracks_crud_controller::init)),
    )
    .await;
    let cookie = make_test_auth_cookie(secret, created_user.id.to_string());
    let insert_request = test::TestRequest::post()
        .uri("/tracks/add-track")
        .insert_header((
            header::COOKIE,
            format!("{}={}", cookie.name(), cookie.value()),
        ))
        .set_json(&AddTrackParams { track })
        .to_request();
    let resp: AddTrackResult = test::call_and_read_body_json(&app, insert_request).await;

    let remove_request = test::TestRequest::delete()
        .uri(&format!(
            "/tracks/remove?user_id={}&track_id={}",
            created_user.id, resp.track_id
        ))
        .insert_header((
            header::COOKIE,
            format!("{}={}", cookie.name(), cookie.value()),
        ))
        .to_request();
    let resp = test::call_service(&app, remove_request).await;
    assert!(matches!(resp.status(), StatusCode::OK));
    Ok(())
}

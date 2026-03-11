use std::sync::Arc;

use actix_http::{Request, StatusCode};
use actix_web::{test, web, App};

use ::actors::user_actor::player_factory::PlayerFactory;
use actors::user_actor::{user_actor_deps::UserActorDeps, user_actor_registry::UserActorRegistry};
use crate::users::{in_memory_user_provider::InMemoryUserProvider, user_provider::UserProvider};
use rstest::rstest;
use ulid::Ulid;

use crate::{
    app_data::AppData,
    controllers::user_controller::{self, GetUserDataResult, UpdateUserParams, UserUpdateResult},
    player_controller_test::controllers::utils::{create_user_and_actor, init_env},
    tests::test_providers::{StubRegionSetsProvider, StubTracksProvider},
    user_and_actor_resolver::local_user_and_actor_resolver::LocalUserAndActorResolver,
};

#[rstest]
#[actix_web::test]
async fn can_get_user() -> Result<(), String> {
    init_env();
    let user_name = Ulid::new();
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(StubTracksProvider::new()),
        region_sets_provider: Arc::new(StubRegionSetsProvider::new()),
    });

    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());

    let (_created_user, _) = create_user_and_actor(
        user_name.to_string(),
        user_registry.clone(),
        user_provider.clone(),
        user_actor_deps.clone(),
    )
    .await?;
    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(StubTracksProvider::new()),
            region_sets_provider: Arc::new(StubRegionSetsProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(user_provider, user_registry)),
    };
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/user").configure(user_controller::init)),
    )
    .await;

    let uri = format!("/user/get-user-state/{}", _created_user.id);
    let get_user_request: Request = test::TestRequest::get().uri(&uri).to_request();
    let _result: GetUserDataResult = test::call_and_read_body_json(&app, get_user_request).await;
    Ok(())
}

#[rstest]
#[actix_web::test]
async fn can_insert_user() -> Result<(), String> {
    init_env();
    let user_name = Ulid::new();
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(StubTracksProvider::new()),
        region_sets_provider: Arc::new(StubRegionSetsProvider::new()),
    });

    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());

    let (_created_user, _) = create_user_and_actor(
        user_name.to_string(),
        user_registry.clone(),
        user_provider.clone(),
        user_actor_deps.clone(),
    )
    .await?;
    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(StubTracksProvider::new()),
            region_sets_provider: Arc::new(StubRegionSetsProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(user_provider, user_registry)),
    };
    let _ = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/user").configure(user_controller::init)),
    )
    .await;

    assert_eq!(_created_user.name, user_name.to_string());
    Ok(())
}

#[actix_web::test]
async fn can_remove_user() -> Result<(), String> {
    init_env();
    let user_name = Ulid::new();
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(StubTracksProvider::new()),
        region_sets_provider: Arc::new(StubRegionSetsProvider::new()),
    });

    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());

    let (_created_user, _) = create_user_and_actor(
        user_name.to_string(),
        user_registry.clone(),
        user_provider.clone(),
        user_actor_deps.clone(),
    )
    .await?;
    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(StubTracksProvider::new()),
            region_sets_provider: Arc::new(StubRegionSetsProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(user_provider, user_registry)),
    };
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/user").configure(user_controller::init)),
    )
    .await;
    let req = test::TestRequest::delete()
        .uri(&format!("/user/remove/{}", _created_user.id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();

    assert!(matches!(status, StatusCode::NO_CONTENT));
    Ok(())
}

#[actix_web::test]
async fn can_update_user() -> Result<(), String> {
    init_env();
    let new_user_name = "adrian2";
    let new_email = "adrian.bercovici2@yahoo.com";
    let user_name = Ulid::new();
    let user_actor_deps = Arc::new(UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(StubTracksProvider::new()),
        region_sets_provider: Arc::new(StubRegionSetsProvider::new()),
    });

    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());

    let (_created_user, _) = create_user_and_actor(
        user_name.to_string(),
        user_registry.clone(),
        user_provider.clone(),
        user_actor_deps.clone(),
    )
    .await?;
    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(StubTracksProvider::new()),
            region_sets_provider: Arc::new(StubRegionSetsProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(user_provider, user_registry)),
    };
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/user").configure(user_controller::init)),
    )
    .await;
    let update_request = UpdateUserParams {
        email: new_email.to_string(),
        user_id: _created_user.id.clone(),
        user_name: new_user_name.to_string(),
    };
    let update_request = test::TestRequest::put()
        .uri("/user/update")
        .set_json(update_request)
        .to_request();

    let update_result: UserUpdateResult = test::call_and_read_body_json(&app, update_request).await;

    assert_eq!(update_result.user_id, _created_user.id);
    assert_eq!(update_result.new_email, new_email);
    assert_eq!(update_result.new_user_name, new_user_name);
    Ok(())
}

use std::{collections::HashMap, sync::Arc};

use actix_web::{test, web, App};
use actors::user_actor::{
    player_factory::PlayerFactory, user_actor_deps::UserActorDeps,
    user_actor_registry::UserActorRegistry,
};
use data_provider::{
    users::{in_memory_user_provider::InMemoryUserProvider, user_provider::UserProvider},
};
use httpmock::prelude::*;

use ulid::Ulid;

use crate::{
    app_data::AppData,
    controllers::google_controller::{self, exchange_code_for_user},
    player_controller_test::utils::create_user_actor,
    tests::test_providers::{StubRegionSetsProvider, StubTracksProvider},
    user_and_actor_resolver::local_user_and_actor_resolver::LocalUserAndActorResolver,
};

#[tokio::test]
async fn test_exchange_code_for_user() -> Result<(), String> {
    let sub = "some_sub";
    let email = "some@gmail.com";
    let name = "some_name";
    let picture = "http://www.google.com";
    let server = MockServer::start();
    let json_body = &serde_json::json!({
        "access_token": "fake_access_token",
        "expires_in": 3600,
        "token_type": "Bearer",
    });
    let _token_mock = server.mock(|when, then| {
        when.method(POST).path("/token");
        then.status(200).json_body_obj(json_body);
    });

    let _user_info_mock = server.mock(|when, then| {
        when.method(GET).path("/userinfo");
        then.status(200).json_body(serde_json::json!({
            "sub":sub,
            "email":email,
            "name":name,
            "picture":picture
        }));
    });
    std::env::set_var("GOOGLE_CLIENT_ID", "test-client-id");
    std::env::set_var("GOOGLE_CLIENT_SECRET", "test-client-secret");
    std::env::set_var("GOOGLE_REDIRECT_URI", "http://localhost/callback");

    let result = exchange_code_for_user(
        "dummy_code".to_string(),
        &format!("{}/token", server.base_url()),
        &format!("{}/userinfo", server.base_url()),
    )
    .await?;
    assert_eq!(result.email, email);
    assert_eq!(result.name, name);
    assert_eq!(result.sub, sub);
    std::env::remove_var("GOOGLE_CLIENT_ID");
    std::env::remove_var("GOOGLE_CLIENT_SECRET");
    std::env::remove_var("GOOGLE_REDIRECT_URI");
    Ok(())
}

pub async fn can_auth_integration_test() -> Result<(), String> {
    let user_id = Ulid::new();
    let user_actor = create_user_actor(user_id);
    let mut user_map = HashMap::new();
    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    user_map.insert(user_id.to_string(), user_actor);
    let app_data = AppData {
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(StubTracksProvider::new()),
            region_sets_provider: Arc::new(StubRegionSetsProvider::new()),
        }),
        user_resolver: Arc::new(LocalUserAndActorResolver::new(
            user_provider,
            Arc::new(UserActorRegistry::new()),
        )),
    };
    let _app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/auth/google").configure(google_controller::init)),
    )
    .await;
    Ok(())
}

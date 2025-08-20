use std::sync::Arc;

use actors::user_actor::{
    create_user_actor_params::CreateUserActorParams, player_factory::PlayerFactory,
    user_actor_deps::UserActorDeps, user_actor_registry::UserActorRegistry,
};
use data_provider::{
    in_memory_user_provider::InMemoryUserProvider, tracks_provider::LocalTrackStoreProvider,
    user_provider::UserProvider,
};
use rstest::rstest;

use crate::{
    dtos::google_user_info::GoogleUserInfo,
    user_and_actor_resolver::local_user_and_actor_resolver::LocalUserAndActorResolver,
};

#[rstest]
#[tokio::test]
pub async fn can_resolve_new_user() -> Result<(), String> {
    let google_id = "some google id";
    let google_user_info = GoogleUserInfo {
        email: "some email".into(),
        name: "some_name".into(),
        picture: "112".into(),
        sub: google_id.into(),
    };
    let user_actor_deps = Arc::new(create_user_deps());
    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());
    let resolver =
        LocalUserAndActorResolver::new(Arc::clone(&user_provider), Arc::clone(&user_registry));
    let create_result = resolver
        .resolve_google_user_and_actor(&google_user_info, |domain_user| {
            Ok(CreateUserActorParams {
                user_data: domain_user,
                user_actor_deps: user_actor_deps,
            })
        })
        .await?;
    assert_eq!(create_result.user.google_sub_id.unwrap(), google_id);
    Ok(())
}

#[rstest]
#[tokio::test]
pub async fn can_resolve_existitng_user() -> Result<(), String> {
    let google_id = "some google id";
    let google_user_info = GoogleUserInfo {
        email: "some email".into(),
        name: "some_name".into(),
        picture: "112".into(),
        sub: google_id.into(),
    };
    let user_actor_deps = Arc::new(create_user_deps());
    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());
    let resolver =
        LocalUserAndActorResolver::new(Arc::clone(&user_provider), Arc::clone(&user_registry));
    let create_result = resolver
        .resolve_google_user_and_actor(&google_user_info, |domain_user| {
            Ok(CreateUserActorParams {
                user_data: domain_user,
                user_actor_deps: Arc::clone(&user_actor_deps),
            })
        })
        .await?;
    let lookup_result = resolver
        .resolve_google_user_and_actor(&google_user_info, |domain_user| {
            Ok(CreateUserActorParams {
                user_data: domain_user,
                user_actor_deps: user_actor_deps,
            })
        })
        .await?;
    assert_eq!(lookup_result.user.id, create_result.user.id);
    Ok(())
}

#[tokio::test]
pub async fn can_get_existing_user() -> Result<(), String> {
    let google_id = "some google id";
    let google_user_info = GoogleUserInfo {
        email: "some email".into(),
        name: "some_name".into(),
        picture: "112".into(),
        sub: google_id.into(),
    };
    let user_actor_deps = Arc::new(create_user_deps());
    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());
    let resolver =
        LocalUserAndActorResolver::new(Arc::clone(&user_provider), Arc::clone(&user_registry));
    let create_result = resolver
        .resolve_google_user_and_actor(&google_user_info, |domain_user| {
            Ok(CreateUserActorParams {
                user_data: domain_user,
                user_actor_deps: Arc::clone(&user_actor_deps),
            })
        })
        .await?;
    let lookup_result = resolver
        .resolve_existing_user_and_actor(&create_result.user.id)
        .await?;
    assert_eq!(lookup_result.user.id, create_result.user.id);
    Ok(())
}

fn create_user_deps() -> UserActorDeps {
    UserActorDeps {
        player_factory: Arc::new(PlayerFactory {}),
        tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
    }
}

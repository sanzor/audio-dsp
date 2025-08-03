use std::{collections::HashMap, sync::Arc};

use actix_web::{
    web::{self},
    App, HttpServer,
};
use actors::user_actor::{player_factory::PlayerFactory, user_actor_deps::UserActorDeps, user_actor_registry::UserActorRegistry};
use data_provider::{in_memory_user_provider::InMemoryUserProvider, tracks_provider::LocalTrackStoreProvider, user_provider::UserProvider};
use dsp_api::{
    app_data::AppData,
    controllers::{self, google_controller, ws_controller}, user_and_actor_resolver::local_user_and_actor_resolver::LocalUserAndActorResolver
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_provider:Arc<dyn UserProvider>=Arc::new(InMemoryUserProvider::new());
    let user_registry=Arc::new(UserActorRegistry::new());
    let registry = AppData {
        user_resolver: Arc::new(LocalUserAndActorResolver::new(Arc::clone(&user_provider), user_registry)),
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new())
        }),
    };
    dotenv::dotenv().ok();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(registry.clone()))
            .service(web::scope("/player").configure(controllers::player_controller::init))
            .service(web::scope("/user").configure(controllers::user_controller::init))
            .service(web::scope("/tracks").configure(controllers::tracks_crud_controller::init))
            .service(web::scope("/auth/google").configure(google_controller::init))
            // .service(web::scope("/auth/facebook").configure(facebook_controller::init))
            .service(web::scope("/ws").configure(ws_controller::init))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

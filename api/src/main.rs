use std::{collections::HashMap, sync::Arc};

use actix_web::{
    web::{self},
    App, HttpServer,
};
use actors::user_actor::player_factory::PlayerFactory;
use dsp_api::{
    app_data::AppData,
    controllers::{self, facebook_controller, google_controller, ws_controller}, user_provider::in_memory_user_provider::InMemoryUserProvider,
};
use tokio::sync::Mutex;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
   
    let registry = AppData {
        user_resolver:Arc::new(UserResolver::new())
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

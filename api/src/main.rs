use std::{collections::HashMap, sync::Arc};

use actix_web::{web, App, HttpServer};
use dsp_api::{actor_registry::AppData, controllers};
use tokio::sync::Mutex;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let registry=AppData{
        user_map:Arc::new(Mutex::new(HashMap::new()))
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(registry.clone()))
            .service(web::scope("/player").configure(controllers::player_controller::init)
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

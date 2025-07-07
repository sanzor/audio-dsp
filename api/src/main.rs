use std::{collections::HashMap, sync::Arc};

use actix_web::{
    web::{self, service},
    App, HttpServer,
};
use dsp_api::{
    app_data::AppData,
    controllers::{self, facebook_controller, google_controller, user_controller},
};
use dsp_core::command_processor::CommandProcessor;
use tokio::sync::Mutex;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let registry = AppData {
        user_map: Arc::new(Mutex::new(HashMap::new())),
        processor: Arc::new(CommandProcessor::create_processor()),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(registry.clone()))
            .service(
                web::scope("/player")
                    .configure(controllers::player_controller::init)
                    .service(web::scope("/user").configure(controllers::user_controller::init))
                    .service(
                        web::scope(
                            "/tracks
                    
                    ",
                        )
                        .configure(controllers::tracks_crud_controller::init),
                    )
                    .service(web::scope("/auth/google").configure(google_controller::init))
                    .service(web::scope("/auth/facebook").configure(facebook_controller::init)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

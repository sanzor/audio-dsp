use actix_web::{
    http::header,
    web::{self},
    App, HttpServer,
};
use actors::user_actor::{
    player_factory::PlayerFactory, user_actor_deps::UserActorDeps,
    user_actor_registry::UserActorRegistry,
};
use api::{
    app_data::AppData,
    controllers::{self, google_controller, ws_controller},
    user_and_actor_resolver::local_user_and_actor_resolver::LocalUserAndActorResolver,
};
use data_provider::{
    in_memory_region_set_provider::InMemoryRegionSetProvider,
    in_memory_user_provider::InMemoryUserProvider, tracks_provider::LocalTrackStoreProvider,
    user_provider::UserProvider,
};
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    // ✅ Ensure environment variables are loaded before anything else
    dotenv::from_path("api/.env").ok();

    // Optional: debug print to confirm env is loaded
    println!("GOOGLE_CLIENT_ID = {:?}", std::env::var("GOOGLE_CLIENT_ID"));

    // ✅ Manually start Actix runtime
    actix_web::rt::System::new().block_on(async { start_server().await })
}
async fn start_server() -> std::io::Result<()> {
    println!("🔍 CWD: {:?}", std::env::current_dir());

    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());
    let registry = AppData {
        user_resolver: Arc::new(LocalUserAndActorResolver::new(
            Arc::clone(&user_provider),
            user_registry,
        )),
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(LocalTrackStoreProvider::new()),
            region_sets_provider: Arc::new(InMemoryRegionSetProvider::new()),
        }),
    };
    let url = "localhost";
    let port = 3080;
    println!("🚀 Server running at http://{url}:{port}");

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            // allow any localhost/127.0.0.1 *with* credentials
            .allowed_origin_fn(|origin, _| {
                let o = origin.as_bytes();
                o.starts_with(b"http://localhost:") || o.starts_with(b"http://127.0.0.1:")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::ORIGIN,
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(registry.clone()))
            .service(web::scope("/player").configure(controllers::player_controller::init))
            .service(web::scope("/user").configure(controllers::user_controller::init))
            .service(web::scope("/tracks").configure(controllers::tracks_crud_controller::init))
            .service(web::scope("/regions").configure(controllers::regions_controller::init))
            .service(web::scope("/region-sets").configure(controllers::region_set_controller::init))
            .service(web::scope("/auth/google").configure(google_controller::init))
            .service(web::scope("/ws").configure(ws_controller::init))
    })
    .bind((url, port))?
    .run()
    .await
}

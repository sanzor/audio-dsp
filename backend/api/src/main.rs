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
    region_sets::region_sets_provider_service::PostgresRegionSetsProvider,
    tracks::tracks_provider_service::PostgresTracksProvider,
    users::{in_memory_user_provider::InMemoryUserProvider, user_provider::UserProvider},
};




use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let _ = dotenvy::from_filename("api/.env");
    let _ = dotenvy::from_filename("api/dev.env");
    let _ = dotenvy::dotenv();

    // ✅ Manually start Actix runtime
    actix_web::rt::System::new().block_on(async { start_server().await })
}
async fn start_server() -> std::io::Result<()> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3080);
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/audio_dsp".to_string());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let user_provider: Arc<dyn UserProvider> = Arc::new(InMemoryUserProvider::new());
    let user_registry = Arc::new(UserActorRegistry::new());
    let registry = AppData {
        user_resolver: Arc::new(LocalUserAndActorResolver::new(
            Arc::clone(&user_provider),
            user_registry,
        )),
        user_actor_deps: Arc::new(UserActorDeps {
            player_factory: Arc::new(PlayerFactory {}),
            tracks_provider: Arc::new(PostgresTracksProvider::new(pool.clone())),
            region_sets_provider: Arc::new(PostgresRegionSetsProvider::new(pool.clone())),
        }),
    };
    println!("Server running at http://{host}:{port}");

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
            .wrap_fn(|req, srv| {
                controllers::metrics_controller::inc_http_requests_total();
                srv.call(req)
            })
            .app_data(web::Data::new(registry.clone()))
            .configure(controllers::openapi_controller::init)
            .service(web::scope("/metrics").configure(controllers::metrics_controller::init))
            .service(web::scope("/player").configure(controllers::player_controller::init))
            .service(web::scope("/user").configure(controllers::user_controller::init))
            .service(web::scope("/tracks").configure(controllers::tracks_crud_controller::init))
            .service(web::scope("/regions").configure(controllers::regions_controller::init))
            .service(web::scope("/region-sets").configure(controllers::region_set_controller::init))
            .service(web::scope("/auth/google").configure(google_controller::init))
            .service(web::scope("/ws").configure(ws_controller::init))
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

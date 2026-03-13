use actix_web::{
    http::header,
    web::{self},
    App, HttpServer,
};
use actors::{
    audio_player_actor::registry::AudioPlayerRegistry,
    user_actor::{
        player_factory::PlayerFactory, user_actor_deps::UserActorDeps,
        user_actor_registry::UserActorRegistry,
    },
};
use api::{
    app_data::AppData,
    auth::{
        auth_app_data::AuthAppData,
        auth_provider_service::AuthProviderService,
        jwt_provider_service::JwtProviderService,
        mock_email_sender::MockEmailSender,
    },
    controllers::{self, google_controller, ws_controller},
    me::{me_app_data::MeAppData, me_provider_service::MeProviderService},
    graphs::{
        data_provider::graphs_data_provider_service::PostgresGraphsDataProvider,
        graphs_provider_service::GraphsProviderService,
    },
    memberships::{
        data_provider::memberships_data_provider_service::PostgresMembershipsDataProvider,
        memberships_provider::MembershipsProvider,
        memberships_provider_service::MembershipsProviderService,
    },
    middlewares::{
        jwt::JwtAuthMiddleware,
        role_context::RoleContextMiddleware,
    },
    player::player_service::PlayerService,
    projects::{
        data_provider::projects_data_provider_service::PostgresProjectsDataProvider,
        projects_provider::ProjectsProvider,
        projects_provider_service::ProjectsProviderService,
    },
    region_sets::{
        data_provider::region_sets_data_provider_service::PostgresRegionSetsDataProvider,
        region_sets_provider_service::RegionSetsProviderService,
    },
    regions::{
        data_provider::regions_data_provider_service::PostgresRegionsDataProvider,
        regions_provider_service::RegionsProviderService,
    },
    tracks::{
        data_provider::tracks_data_provider_service::PostgresTracksDataProvider,
        tracks_provider_service::TracksProviderService,
    },
    user_and_actor_resolver::local_user_and_actor_resolver::LocalUserAndActorResolver,
    users::{
        postgres_user_provider::PostgresUserProvider,
        user_provider::UserProvider,
    },
};
use actors::{
    region_sets::region_sets_provider_service::PostgresRegionSetsProvider,
    tracks::tracks_provider_service::PostgresTracksProvider,
};
use crate::users::in_memory_user_provider::InMemoryUserProvider;

use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let _ = dotenvy::from_filename("api/.env");
    let _ = dotenvy::from_filename("api/dev.env");
    let _ = dotenvy::dotenv();

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

    // User provider: Postgres-backed for real persistence
    let user_provider: Arc<dyn UserProvider> =
        Arc::new(PostgresUserProvider::new(pool.clone()));

    let user_registry = Arc::new(UserActorRegistry::new());

    let tracks_service = Arc::new(TracksProviderService::new(Arc::new(
        PostgresTracksDataProvider::new(pool.clone()),
    )));
    let player_service = Arc::new(PlayerService::new(
        Arc::new(AudioPlayerRegistry::new()),
        Arc::clone(&tracks_service) as Arc<dyn api::tracks::tracks_provider::TracksProvider>,
    ));

    let projects_service: Arc<dyn ProjectsProvider> = Arc::new(ProjectsProviderService::new(
        Arc::new(PostgresProjectsDataProvider::new(pool.clone())),
    ));
    let memberships_service: Arc<dyn MembershipsProvider> =
        Arc::new(MembershipsProviderService::new(Arc::new(
            PostgresMembershipsDataProvider::new(pool.clone()),
        )));

    // Auth
    let jwt_provider = Arc::new(JwtProviderService);
    let email_sender = Arc::new(MockEmailSender);
    let auth_provider = Arc::new(AuthProviderService::new(
        Arc::clone(&user_provider),
        Arc::clone(&memberships_service),
        Arc::clone(&jwt_provider) as Arc<dyn api::auth::jwt_provider::JwtProvider>,
        Arc::clone(&email_sender) as Arc<dyn api::auth::email_sender::EmailSender>,
    ));
    let auth_app_data = AuthAppData {
        auth_provider,
        jwt_provider: Arc::clone(&jwt_provider) as Arc<dyn api::auth::jwt_provider::JwtProvider>,
    };

    let me_app_data = MeAppData {
        me_data_provider: Arc::new(MeProviderService::new(
            Arc::clone(&user_provider),
            Arc::clone(&memberships_service),
            Arc::clone(&projects_service),
            Arc::clone(&jwt_provider) as Arc<dyn api::auth::jwt_provider::JwtProvider>,
        )),
    };

    let jwt_middleware = JwtAuthMiddleware;
    let role_middleware = RoleContextMiddleware {
        memberships: Arc::clone(&memberships_service),
        user_provider: Arc::clone(&user_provider),
    };

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
        player_service,
        tracks_service,
        region_sets_service: Arc::new(RegionSetsProviderService::new(Arc::new(
            PostgresRegionSetsDataProvider::new(pool.clone()),
        ))),
        regions_service: Arc::new(RegionsProviderService::new(Arc::new(
            PostgresRegionsDataProvider::new(pool.clone()),
        ))),
        graphs_service: Arc::new(GraphsProviderService::new(Arc::new(
            PostgresGraphsDataProvider::new(pool.clone()),
        ))),
        projects_service,
        memberships_service,
        user_provider,
    };
    println!("Server running at http://{host}:{port}");

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
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
            .app_data(web::Data::new(auth_app_data.clone()))
            .app_data(web::Data::new(me_app_data.clone()))
            .configure(controllers::openapi_controller::init)
            .service(web::scope("/metrics").configure(controllers::metrics_controller::init))
            .service(web::scope("/auth").configure(controllers::auth_controller::init))
            .service(
                web::scope("/v1/me")
                    .wrap(jwt_middleware.clone())
                    .configure(controllers::me_controller::init),
            )
            .service(
                web::scope("/player")
                    .wrap(role_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(controllers::player_controller::init),
            )
            .service(web::scope("/user").configure(controllers::user_controller::init))
            .service(
                web::scope("/tracks")
                    .wrap(role_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(controllers::tracks_crud_controller::init),
            )
            .service(
                web::scope("/regions")
                    .wrap(role_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(controllers::regions_controller::init),
            )
            .service(
                web::scope("/region-sets")
                    .wrap(role_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(controllers::region_set_controller::init),
            )
            .service(web::scope("/auth/google").configure(google_controller::init))
            .service(web::scope("/ws").configure(ws_controller::init))
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

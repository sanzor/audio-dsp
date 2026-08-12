use actix_web::{
    web::{self},
    App, HttpServer,
};
use rand::Rng;
use actors::{
    audio_player_actor::registry::AudioPlayerRegistry,
};
use api::{
    app_data::AppData, auth::{
        auth_app_data::AuthAppData,
        auth_provider_service::AuthProviderService,
        jwt_provider_service::JwtProviderService,
        mock_email_sender::MockEmailSender,
    }, controllers::{self, ws_controller}, graphs::{
        data_provider::graphs_data_provider_service::PostgresGraphsDataProvider,
        graphs_app_data::GraphsAppData,
        graphs_provider_service::GraphsProviderService,
    }, infra::producer::channel_producer::ChannelProducer, invoices::{
        data_provider::invoices_data_provider_service::InvoicesDataProviderService,
        invoices_app_data::InvoicesAppData,
        invoices_provider_service::InvoicesProviderService,
    }, me::{me_app_data::MeAppData, me_provider_service::MeProviderService}, memberships::{
        data_provider::memberships_data_provider_service::PostgresMembershipsDataProvider,
        memberships_app_data::MembershipsAppData,
        memberships_provider::MembershipsProvider,
        memberships_provider_service::MembershipsProviderService,
    }, middlewares::{
        jwt::JwtAuthMiddleware,
        membership::MembershipMiddleware,
        permissions_context::permissions_context_middleware::PermissionsContextMiddleware,
    }, player::{player_app_data::PlayerAppData, player_service::PlayerService}, products::{
        data_provider::products_data_provider_service::ProductsDataProviderService,
        products_app_data::ProductsAppData,
        products_provider_service::ProductsProviderService,
    }, purchased_products::{
        data_provider::purchased_products_data_provider_service::PurchasedProductsDataProviderService,
        purchased_products_app_data::PurchasedProductsAppData,
        purchased_products_provider_service::PurchasedProductsProviderService,
    }, region_sets::{
        data_provider::region_sets_data_provider_service::PostgresRegionSetsDataProvider,
        region_sets_app_data::RegionSetsAppData,
        region_sets_provider_service::RegionSetsProviderService,
    }, regions::{
        data_provider::regions_data_provider_service::PostgresRegionsDataProvider,
        regions_app_data::RegionsAppData,
        regions_provider_service::RegionsProviderService,
    }, sources::{
        data_provider::sources_data_provider_service::PostgresSourcesDataProvider,
        multipart_audio_parser::multipart_audio_parser_service::SourceMultipartAudioParserService,
        sources_app_data::SourcesAppData,
        sources_provider_service::SourcesProviderService,
        storage_provider::source_storage_provider_service::SourceStorageProviderService,
    }, stored_tracks::{
        data_provider::stored_tracks_data_provider_service::PostgresStoredTracksDataProvider,
        stored_tracks_app_data::StoredTracksAppData,
    }, subscriptions::{
        data_provider::subscriptions_data_provider_service::SubscriptionsDataProviderService,
        subscriptions_app_data::SubscriptionsAppData,
        subscriptions_provider_service::SubscriptionsProviderService,
    }, ticket_worker::{
        consumer::channel_consumer::ChannelConsumer, events::ticket_created_event::TicketCreatedEvent, processor::{build_job_config::BuildJobConfig, processor::Processor, processor_params::ProcessorParams}, worker::Worker, worker_config::WorkerConfig, worker_params::WorkerParams,
    }, tickets::{tickets_app_data::TicketsAppData, tickets_provider_service::TicketsProviderService}, transform_grants::{
        data_provider::transform_grants_data_provider_service::PostgresTransformGrantsDataProvider,
        transform_grants_app_data::TransformGrantsAppData,
        transform_grants_provider_service::TransformGrantsProviderService,
    }, tier_configs::{
        data_provider::tier_configs_data_provider_service::TierConfigsDataProviderService,
        tier_configs_app_data::TierConfigsAppData,
        tier_configs_provider_service::TierConfigsProviderService,
    }, tracks::{
        data_provider::tracks_data_provider_service::PostgresTracksDataProvider, multipart_audio_parser::multipart_audio_parser_service::MultipartAudioParserService, storage_provider::track_storage_provider_service::TrackStorageProviderService, tracks_app_data::TracksAppData, tracks_provider_service::TracksProviderService,
    }, transforms::{
        data_provider::transforms_data_provider_service::PostgresTransformsDataProvider,
        storage_provider::transform_storage_provider_service::DbTransformStorageProvider,
        transforms_app_data::TransformsAppData,
        transforms_provider_service::TransformsProviderService,
    }, usage::{
        data_provider::usage_data_provider_service::UsageDataProviderService,
        usage_app_data::UsageAppData,
        usage_provider_service::UsageProviderService,
    }, users::{
        data_provider::user_data_provider_service::UserDataProviderService,
        user_provider::UserProvider,
        user_provider_service::UserProviderService,
        users_app_data::UsersAppData,
    }, workspace::{
        data_provider::workspace_data_provider_service::PostgresWorkspaceDataProvider,
        workspace_app_data::WorkspaceAppData,
        workspace_provider_service::WorkspaceProviderService,
    }, workspaces::{
        data_provider::workspaces_data_provider_service::PostgresWorkspacesDataProvider,
        workspaces_app_data::WorkspacesAppData,
        workspaces_provider::WorkspacesProvider,
        workspaces_provider_service::WorkspacesProviderService,
    },
};
use tokio_util::sync::CancellationToken;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use std::sync::Arc;

fn main() -> std::io::Result<()> {
    // Match the "root .env + .env.<env>" workflow (like `temp/backend`), but keep
    // explicit ordering and allow OS env vars to win over dotenv.
    load_env_files();
    init_tracing("api");

    let app_config = api::config::load_app_config()
        .expect("Failed to load app config");

    actix_web::rt::System::new().block_on(async { start_server(app_config).await })
}

fn init_tracing(service_name: &str) {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let sampling_ratio = std::env::var("LOG_SAMPLING_RATIO")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .map(|v| v.clamp(0.0, 1.0))
        .unwrap_or(1.0);

    let filter_layer = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&log_level))
        .add_directive(format!("{service_name}=debug").parse().unwrap());

    let sampling_layer = tracing_subscriber::filter::filter_fn(move |metadata| {
        if metadata.level() <= &tracing::Level::WARN {
            return true;
        }
        rand::rng().random_bool(sampling_ratio)
    });

    let fmt_layer = fmt::layer()
        .json()
        .flatten_event(true)
        .with_current_span(true)
        .with_span_list(true)
        .with_target(true)
        .with_timer(fmt::time::UtcTime::rfc_3339());

    let _ = tracing_subscriber::registry()
        .with(filter_layer)
        .with(sampling_layer)
        .with(fmt_layer)
        .try_init();
}

fn load_env_files() {
    use std::collections::HashSet;
    use std::ffi::OsString;
    use std::path::{Path, PathBuf};

    fn load_layered_env_file(
        path: &Path,
        preexisting_keys: &HashSet<OsString>,
        loaded_keys: &mut HashSet<OsString>,
    ) {
        if !path.exists() {
            return;
        }

        let iter = match dotenvy::from_path_iter(path) {
            Ok(iter) => iter,
            Err(_) => return,
        };

        for item in iter {
            let Ok((key, val)) = item else { continue };
            let key_os: OsString = key.clone().into();

            // Preserve real environment variables (shell/VS Code env injection), but allow
            // later dotenv layers to override earlier dotenv layers.
            let can_set = !preexisting_keys.contains(&key_os) || loaded_keys.contains(&key_os);
            if can_set {
                std::env::set_var(&key, &val);
                loaded_keys.insert(key_os);
            }
        }
    }

    let preexisting_keys: HashSet<OsString> = std::env::vars_os().map(|(k, _)| k).collect();
    let mut loaded_keys: HashSet<OsString> = HashSet::new();

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir.join("../..");

    // Defaults (committed)
    load_layered_env_file(&manifest_dir.join("dev.env"), &preexisting_keys, &mut loaded_keys);

    // Root env (gitignored) + env-specific root override (gitignored)
    let root_env = repo_root.join(".env");
    load_layered_env_file(&root_env, &preexisting_keys, &mut loaded_keys);

    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
    load_layered_env_file(
        &repo_root.join(format!(".env.{app_env}")),
        &preexisting_keys,
        &mut loaded_keys,
    );

    // Service overrides (gitignored)
    load_layered_env_file(&manifest_dir.join(".env"), &preexisting_keys, &mut loaded_keys);
}

async fn start_server(app_config: api::config::AppConfig) -> std::io::Result<()> {
    let upload_limit_bytes = app_config.upload_limit_bytes();
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3080);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&app_config.database.url)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let user_provider: Arc<dyn UserProvider> = Arc::new(UserProviderService::new(Arc::new(
        UserDataProviderService::new(pool.clone()),
    )));

    let track_storage_service = Arc::new(TrackStorageProviderService::new(Arc::new(
        PostgresStoredTracksDataProvider::new(pool.clone()),
    )));
    let tracks_service = Arc::new(TracksProviderService::new(
        Arc::new(PostgresTracksDataProvider::new(pool.clone())),
        Arc::clone(&track_storage_service)
            as Arc<dyn api::tracks::storage_provider::track_storage_provider::TrackStorageProvider>,
    ));
    let sources_service = Arc::new(SourcesProviderService::new(
        Arc::new(PostgresSourcesDataProvider::new(pool.clone())),
        Arc::new(SourceStorageProviderService::new(pool.clone()))
            as Arc<dyn api::sources::storage_provider::source_storage_provider::SourceStorageProvider>,
    ));
    let player_service = Arc::new(PlayerService::new(
        Arc::new(AudioPlayerRegistry::new()),
        Arc::clone(&tracks_service) as Arc<dyn api::tracks::tracks_provider::TracksProvider>,
    ));
    let region_sets_service = Arc::new(RegionSetsProviderService::new(Arc::new(
        PostgresRegionSetsDataProvider::new(pool.clone()),
    )));
    let regions_service = Arc::new(RegionsProviderService::new(Arc::new(
        PostgresRegionsDataProvider::new(pool.clone()),
    )));
    let graphs_service = Arc::new(GraphsProviderService::new(Arc::new(
        PostgresGraphsDataProvider::new(pool.clone()),
    )));
    let workspaces_service: Arc<dyn WorkspacesProvider> = Arc::new(WorkspacesProviderService::new(
        Arc::new(PostgresWorkspacesDataProvider::new(pool.clone())),
    ));
    let memberships_service: Arc<dyn MembershipsProvider> =
        Arc::new(MembershipsProviderService::new(Arc::new(
            PostgresMembershipsDataProvider::new(pool.clone()),
        )));

    let jwt_provider = Arc::new(JwtProviderService);
    let email_sender = Arc::new(MockEmailSender);
    let auth_provider = Arc::new(AuthProviderService::new(
        Arc::clone(&user_provider),
        Arc::clone(&memberships_service),
        Arc::clone(&jwt_provider) as Arc<dyn api::auth::jwt_provider::JwtProvider>,
        Arc::clone(&email_sender) as Arc<dyn api::auth::email_sender::EmailSender>,
    ));

    // Focused app_data instances
    let auth_app_data = AuthAppData {
        auth_provider,
        jwt_provider: Arc::clone(&jwt_provider) as Arc<dyn api::auth::jwt_provider::JwtProvider>,
    };

    let me_app_data = MeAppData {
        me_data_provider: Arc::new(MeProviderService::new(
            Arc::clone(&user_provider),
            Arc::clone(&memberships_service),
            Arc::clone(&workspaces_service),
        )),
    };

    let app_data = AppData {
        workspaces_service: Arc::clone(&workspaces_service),
        memberships_service: Arc::clone(&memberships_service),
    };
    let tracks_app_data = TracksAppData {
        tracks_service: Arc::clone(&tracks_service)
            as Arc<dyn api::tracks::tracks_provider::TracksProvider>,
        multipart_parser: Arc::new(MultipartAudioParserService {}),
    };
    let sources_app_data = SourcesAppData {
        sources_service: Arc::clone(&sources_service)
            as Arc<dyn api::sources::sources_provider::SourcesProvider>,
        multipart_parser: Arc::new(SourceMultipartAudioParserService {}),
    };
    let regions_app_data = RegionsAppData {
        regions_service: Arc::clone(&regions_service)
            as Arc<dyn api::regions::regions_provider::RegionsProvider>,
    };
    let region_sets_app_data = RegionSetsAppData {
        region_sets_service: Arc::clone(&region_sets_service)
            as Arc<dyn api::region_sets::region_sets_provider::RegionSetsProvider>,
    };
    let player_app_data = PlayerAppData {
        player_service: Arc::clone(&player_service)
            as Arc<dyn api::player::player_provider::PlayerProvider>,
    };
    let workspaces_app_data = WorkspacesAppData {
        workspaces_service: Arc::clone(&workspaces_service),
        memberships_service: Arc::clone(&memberships_service),
    };
    let users_app_data = UsersAppData {
        user_provider: Arc::clone(&user_provider),
    };
    let graphs_app_data = GraphsAppData {
        graphs_service: Arc::clone(&graphs_service)
            as Arc<dyn api::graphs::graphs_provider::GraphsProvider>,
    };
    let memberships_app_data = MembershipsAppData {
        memberships_service: Arc::clone(&memberships_service),
    };

    let invoices_app_data = InvoicesAppData {
        invoices_provider: Arc::new(InvoicesProviderService::new(Arc::new(
            InvoicesDataProviderService::new(pool.clone()),
        ))),
    };
    let products_app_data = ProductsAppData {
        products_provider: Arc::new(ProductsProviderService::new(Arc::new(
            ProductsDataProviderService::new(pool.clone()),
        ))),
    };
    let tier_configs_app_data = TierConfigsAppData {
        tier_configs_provider: Arc::new(TierConfigsProviderService::new(Arc::new(
            TierConfigsDataProviderService::new(pool.clone()),
        ))),
    };
    let subscriptions_app_data = SubscriptionsAppData {
        subscriptions_provider: Arc::new(SubscriptionsProviderService::new(Arc::new(
            SubscriptionsDataProviderService::new(pool.clone()),
        ))),
    };
    let purchased_products_app_data = PurchasedProductsAppData {
        purchased_products_provider: Arc::new(PurchasedProductsProviderService::new(Arc::new(
            PurchasedProductsDataProviderService::new(pool.clone()),
        ))),
    };
    let usage_app_data = UsageAppData {
        usage_provider: Arc::new(UsageProviderService::new(Arc::new(
            UsageDataProviderService::new(pool.clone()),
        ))),
    };
    let (tx, rx) = tokio::sync::mpsc::channel::<TicketCreatedEvent>(256);
    let producer: Arc<dyn api::infra::producer::producer::Producer<TicketCreatedEvent>> =
        Arc::new(ChannelProducer::new(tx));

    let transforms_data_provider: Arc<dyn api::transforms::data_provider::transforms_data_provider::TransformsDataProvider> =
        Arc::new(PostgresTransformsDataProvider::new(pool.clone()));
    let transforms_app_data = TransformsAppData {
        transforms_service: Arc::new(TransformsProviderService::new(
            Arc::clone(&transforms_data_provider),
            Arc::new(DbTransformStorageProvider::new(pool.clone())),
        )),
    };
    let tickets_app_data = TicketsAppData {
        tickets_service: Arc::new(TicketsProviderService::new(
            Arc::clone(&transforms_data_provider),
            Arc::clone(&producer),
        )),
    };
    let transform_grants_app_data = TransformGrantsAppData {
        transform_grants_service: Arc::new(TransformGrantsProviderService::new(Arc::new(
            PostgresTransformGrantsDataProvider::new(pool.clone()),
        ))),
    };

    // Path to the transform-sdk crate the generated per-compile Cargo.toml
    // depends on. Defaults to the workspace-relative path for local dev;
    // overridden to /opt/transform-sdk in the container image (see Dockerfile).
    let transform_sdk_path = std::env::var("TRANSFORM_SDK_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../transform-sdk"));
    let transform_build_workdir = std::env::var("TRANSFORM_BUILD_WORKDIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("audio-dsp-builds"));
    let transform_cargo_target_dir = std::env::var("TRANSFORM_CARGO_TARGET_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("audio-dsp-cargo-target"));
    let transform_cargo_home = std::env::var("CARGO_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/root".to_string())).join(".cargo"));
    let transform_compile_timeout_secs: u64 = std::env::var("TRANSFORM_COMPILE_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(60);
    let transform_max_wasm_bytes: u64 = std::env::var("TRANSFORM_MAX_WASM_BYTES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5 * 1024 * 1024);
    let transform_metadata_fuel_limit: u64 = std::env::var("TRANSFORM_METADATA_FUEL_LIMIT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10_000_000);

    let token = CancellationToken::new();
    let worker_handle = Worker::spawn(
        WorkerParams {
            consumer: Box::new(ChannelConsumer::new(rx)),
            processor: Processor::new(ProcessorParams {
                data_provider: Arc::clone(&transforms_data_provider),
                build_job_config: BuildJobConfig {
                    sdk_path: transform_sdk_path,
                    build_workdir: transform_build_workdir,
                    cargo_target_dir: transform_cargo_target_dir,
                    cargo_home: transform_cargo_home,
                    compile_timeout: std::time::Duration::from_secs(transform_compile_timeout_secs),
                    max_wasm_bytes: transform_max_wasm_bytes,
                },
                metadata_fuel_limit: transform_metadata_fuel_limit,
            }),
        },
        WorkerConfig::default(),
        token.clone(),
    );
    let stored_tracks_app_data = StoredTracksAppData {
        tracks_service: Arc::clone(&tracks_service)
            as Arc<dyn api::tracks::tracks_provider::TracksProvider>,
    };
    let workspace_app_data = WorkspaceAppData {
        workspace_service: Arc::new(WorkspaceProviderService::new(Arc::new(
            PostgresWorkspaceDataProvider::new(pool.clone()),
        ))),
    };

    let jwt_middleware = JwtAuthMiddleware;
    let membership_middleware = MembershipMiddleware {
        memberships: Arc::clone(&memberships_service),
    };
    let users_permissions_middleware = PermissionsContextMiddleware::allow_all();

    println!("Server running at http://{host}:{port}");

    let server = HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allowed_origin_fn(|origin, _| {
                let o = origin.as_bytes();
                o.starts_with(b"http://localhost:")
                    || o.starts_with(b"http://127.0.0.1:")
                    || o.starts_with(b"http://0.0.0.0:")
                    || o.starts_with(b"https://localhost:")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .app_data(web::PayloadConfig::new(upload_limit_bytes))
            .app_data(web::Data::new(app_data.clone()))
            .app_data(web::Data::new(auth_app_data.clone()))
            .app_data(web::Data::new(me_app_data.clone()))
            .app_data(web::Data::new(tracks_app_data.clone()))
            .app_data(web::Data::new(sources_app_data.clone()))
            .app_data(web::Data::new(regions_app_data.clone()))
            .app_data(web::Data::new(region_sets_app_data.clone()))
            .app_data(web::Data::new(player_app_data.clone()))
            .app_data(web::Data::new(workspaces_app_data.clone()))
            .app_data(web::Data::new(users_app_data.clone()))
            .app_data(web::Data::new(graphs_app_data.clone()))
            .app_data(web::Data::new(memberships_app_data.clone()))
            .app_data(web::Data::new(invoices_app_data.clone()))
            .app_data(web::Data::new(products_app_data.clone()))
            .app_data(web::Data::new(tier_configs_app_data.clone()))
            .app_data(web::Data::new(subscriptions_app_data.clone()))
            .app_data(web::Data::new(purchased_products_app_data.clone()))
            .app_data(web::Data::new(usage_app_data.clone()))
            .app_data(web::Data::new(transforms_app_data.clone()))
            .app_data(web::Data::new(tickets_app_data.clone()))
            .app_data(web::Data::new(transform_grants_app_data.clone()))
            .app_data(web::Data::new(stored_tracks_app_data.clone()))
            .app_data(web::Data::new(workspace_app_data.clone()))
            .configure(controllers::openapi_controller::init)
            .service(web::scope("/auth").configure(controllers::auth_controller::init))
            .service(
                web::scope("/v1/me")
                    .wrap(jwt_middleware.clone())
                    .configure(controllers::me_controller::init),
            )
            .service(
                web::scope("/users")
                    .wrap(users_permissions_middleware.clone())
                    .configure(controllers::user_controller::init),
            )
            .service(
                web::scope("/sources")
                    .wrap(membership_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(controllers::sources_controller::init),
            )
            .service(
                web::scope("/v1/workspaces")
                    .wrap(membership_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(controllers::workspace_controller::init)
                    .service(
                        web::scope("/{workspace_id}")
                            .configure(controllers::workspace_tracks_controller::init)
                            .service(
                                web::scope("/tracks")
                                    .configure(controllers::tracks_crud_controller::init),
                            )
                            .service(
                                web::scope("/regions")
                                    .configure(controllers::regions_controller::init),
                            )
                            .service(
                                web::scope("/region-sets")
                                    .configure(controllers::region_set_controller::init),
                            )
                            .service(
                                web::scope("/graphs")
                                    .configure(controllers::graph_controller::init),
                            )
                            .service(
                                web::scope("/stored-tracks")
                                    .configure(controllers::track_payload_controller::init),
                            )
                            .service(
                                web::scope("/player")
                                    .configure(controllers::player_controller::init),
                            )
                            .service(web::scope("/ws").configure(ws_controller::init)),
                    ),
            )
            .service(
                web::scope("/v1/invoices")
                    .wrap(jwt_middleware.clone())
                    .configure(controllers::invoices_controller::init),
            )
            .service(
                web::scope("/v1/products")
                    .configure(controllers::products_controller::init),
            )
            .service(
                web::scope("/v1/tier-configs")
                    .configure(controllers::tier_configs_controller::init),
            )
            .service(
                web::scope("/v1/subscriptions")
                    .wrap(jwt_middleware.clone())
                    .configure(controllers::subscriptions_controller::init),
            )
            .service(
                web::scope("/v1/purchased-products")
                    .wrap(jwt_middleware.clone())
                    .configure(controllers::purchased_products_controller::init),
            )
            .service(
                web::scope("/v1/usage")
                    .wrap(jwt_middleware.clone())
                    .configure(controllers::usage_controller::init),
            )
            .service(
                web::scope("/transforms")
                    .wrap(jwt_middleware.clone())
                    .configure(controllers::transforms_controller::init)
                    .configure(controllers::ticket_controller::init)
                    .configure(controllers::transform_grants_controller::init),
            )
    })
    .bind((host.as_str(), port))?
    .run();

    let server_handle = server.handle();
    tokio::spawn(server);

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("received ctrl+c, shutting down");
            token.cancel();
            server_handle.stop(true).await;
        }
        result = worker_handle => {
            tracing::error!("worker exited unexpectedly: {:?}", result);
            server_handle.stop(true).await;
        }
    }

    Ok(())
}

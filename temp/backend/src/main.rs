use actix_web::{web, App, HttpServer};
use actix_web_prometheus::PrometheusMetricsBuilder;
use backend::stripe::stripe_webhook_app_data::StripeWebhookAppData;
use backend::stripe::rate_limit_service::rate_limit_service_impl::RateLimitServiceImpl;
use backend::stripe::stripe_webhook_handler::StripeWebhookHandler;
use backend::stripe::stripe_webhook_service::StripeWebhookServiceImpl;
use tokio_util::sync::CancellationToken;
use tracing::info;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use common::{common_config::load_app_config, telemetry::init_telemetry};
use infra::redis::build_multiplexed_connection;
use sqlx::PgPool;

use backend::access_control::{
    access_control_provider::AccessControlProvider,
    access_control_provider_service::AccessControlProviderService,
    data_provider::access_control_data_provider_service::AccessControlDataProviderService,
};
use backend::auth::auth_provider_service::AuthProviderService;
use backend::auth::mock_email_sender::MockEmailSender;
use backend::infra::smtp::{SmtpEmailSender, SmtpEmailSenderConfig};
use backend::me::me_provider_service::MeProviderService;
use backend::middlewares::jwt::{
    jwt_middleware::JwtAuthMiddleware, jwt_provider_service::Hs256JwtProvider,
};
use backend::middlewares::organization::organization_context_middleware::OrganizationContextMiddleware;
use backend::middlewares::permissions_context::permissions_context_middleware::PermissionsContextMiddleware;
use backend::{
    api_keys::{
        api_keys_app_data::ApiKeysAppData, api_keys_provider::ApiKeysProvider,
        api_keys_service::ApiKeysService,
        data_provider::api_keys_data_provider_service::ApiKeysDataProviderService,
    },
    app_config::AppConfig,
    auth::auth_app_data::AuthAppData,
    billing_mode::{
        billing_mode_app_data::BillingModeAppData, billing_mode_provider::BillingModeProvider,
        billing_mode_provider_service::BillingModeProviderService,
        data_provider::billing_mode_data_provider_service::BillingModeDataProviderService,
    },
    checkout::{
        checkout_app_data::CheckoutAppData, checkout_provider::CheckoutProvider,
        checkout_provider_service::CheckoutProviderService,
    },
    controllers::{
        api_keys_controller, auth_controller, billing_mode_controller, invoices_controller,
        me_controller, members_controller, membership_roles_controller, memberships_controller,
        organization_controller, permissions_controller, products_controller,
        purchased_products_controller, rate_limits_config_controller, role_permissions_controller,
        roles_controller, stripe_webhook_controller, subscriptions_controller,
        tier_configs_controller, usage_controller, user_controller,
    },
    invoices::{
        data_provider::invoices_data_provider_service::InvoicesDataProviderService,
        invoices_app_data::InvoicesAppData, invoices_provider::InvoicesProvider,
        invoices_provider_service::InvoicesProviderService,
    },
    usage::{
        data_provider::{
            usage_data_provider::UsageDataProvider,
            usage_data_provider_service::UsageDataProviderService,
        },
        usage_app_data::UsageAppData,
        usage_provider::UsageProvider,
        usage_provider_service::UsageProviderService,
    },
    jobs::{
        self,
        sync_worker::{sync_worker_config::SyncWorkerConfig, sync_worker_params::SyncWorkerParams},
        usage_worker::{
            usage_worker_config::UsageWorkerConfig, usage_worker_params::UsageWorkerParams,
        },
    },
    me::me_app_data::MeAppData,
    members::{
        data_provider::members_data_provider_service::MembersDataProviderService,
        members_app_data::MembersAppData, members_provider::MembersProvider,
        members_provider_service::MembersProviderService,
    },
    membership_roles::{
        data_provider::membership_roles_data_provider_service::MembershipRolesDataProviderService,
        membership_roles_app_data::MembershipRolesAppData,
        membership_roles_provider::MembershipRolesProvider,
        membership_roles_provider_service::MembershipRolesProviderService,
    },
    memberships::{
        data_provider::memberships_data_provider_service::MembershipsDataProviderService,
        memberships_app_data::MembershipsAppData, memberships_provider::MembershipsProvider,
        memberships_provider_service::MembershipsProviderService,
    },
    openapi::ApiDoc,
    organizations::{
        data_provider::organization_data_provider_service::OrganizationDataProviderService,
        organization_provider::OrganizationProvider,
        organization_provider_service::OrganizationProviderService,
        organizations_app_data::OrganizationsAppData,
    },
    permissions::{
        data_provider::permissions_data_provider_service::PermissionsDataProviderService,
        permissions_app_data::PermissionsAppData, permissions_provider::PermissionsProvider,
        permissions_provider_service::PermissionsProviderService,
    },
    products::{
        data_provider::products_data_provider_service::ProductsDataProviderService,
        products_app_data::ProductsAppData, products_provider::ProductsProvider,
        products_provider_service::ProductsProviderService,
    },
    purchased_products::{
        data_provider::{
            purchased_products_data_provider::PurchasedProductsDataProvider,
            purchased_products_data_provider_service::PurchasedProductsDataProviderService,
        },
        purchased_products_app_data::PurchasedProductsAppData,
        purchased_products_provider::PurchasedProductsProvider,
        purchased_products_provider_service::PurchasedProductsProviderService,
    },
    rate_limits_config::{
        data_provider::{
            rate_limits_data_provider::RateLimitsDataProvider,
            rate_limits_data_provider_service::RateLimitsDataProviderService,
        },
        rate_limits_app_data::RateLimitsAppData,
        rate_limits_config_provider::RateLimitsConfigProvider,
        rate_limits_config_provider_service::RateLimitsProviderService,
    },
    role_permissions::{
        data_provider::role_permissions_data_provider_service::RolePermissionsDataProviderService,
        role_permissions_app_data::RolePermissionsAppData,
        role_permissions_provider::RolePermissionsProvider,
        role_permissions_provider_service::RolePermissionsProviderService,
    },
    roles::{
        data_provider::roles_data_provider_service::RolesDataProviderService,
        roles_app_data::RolesAppData, roles_provider::RolesProvider,
        roles_provider_service::RolesProviderService,
    },
    stripe::stripe_service::StripeService,
    subscriptions::{
        data_provider::subscriptions_data_provider_service::SubscriptionsDataProviderService,
        subscriptions_app_data::SubscriptionsAppData,
        subscriptions_provider::SubscriptionsProvider,
        subscriptions_provider_service::SubscriptionsProviderService,
    },
    tier_configs::{
        data_provider::tier_configs_data_provider_service::TierConfigsDataProviderService,
        tier_configs_app_data::TierConfigsAppData, tier_configs_provider::TierConfigsProvider,
        tier_configs_provider_service::TierConfigsProviderService,
    },
    users::{
        data_provider::user_data_provider_service::UserDataProviderService,
        user_provider::UserProvider, user_provider_service::UserProviderService,
        users_app_data::UsersAppData,
    },
};

use std::{sync::Arc, time::Duration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing::debug!("User portal starting");
    let config_data: AppConfig = load_app_config().expect("Could not load config");

    let telemetry_task = init_telemetry("backend", &config_data.telemetry_config);
    if let Some(task) = telemetry_task {
        tokio::spawn(task);
    }

    start_server(config_data).await
}

async fn start_server(config_data: AppConfig) -> std::io::Result<()> {
    let mux_connection = build_multiplexed_connection(&config_data.redis.url)
        .await
        .expect("Could not connect to redis");

    let pg_pool = build_pg_pool(&config_data).await;
    let usage_data_provider: Arc<dyn UsageDataProvider> =
        Arc::new(UsageDataProviderService::new(pg_pool.clone()));
    let token = CancellationToken::new();
    let usage_worker = jobs::usage_worker::worker::UsageWorker::spawn_worker(
        UsageWorkerParams {
            conn: mux_connection.clone(),
            usage_data_provider: usage_data_provider.clone(),
        },
        UsageWorkerConfig {
            update_every: Duration::from_secs(config_data.usage_worker.update_every_seconds),
        },
        token.clone(),
    );
    let sync_worker = jobs::sync_worker::sync_worker::SyncWorker::spawn_worker(
        SyncWorkerParams {
            redis: mux_connection.clone(),
            db_conn: pg_pool.clone(),
        },
        SyncWorkerConfig {
            update_every: Duration::from_secs(config_data.sync_worker.update_every_seconds),
        },
        token.clone(),
    );
    let user_provider = build_user_provider_service(pg_pool.clone());
    let memberships_provider = build_memberships_provider_service(pg_pool.clone());
    let members_provider = build_members_provider_service(pg_pool.clone());
    let roles_provider = build_roles_provider_service(pg_pool.clone());
    let permissions_provider = build_permissions_provider_service(pg_pool.clone());
    let role_permissions_provider = build_role_permissions_provider_service(pg_pool.clone());
    let membership_roles_provider = build_membership_roles_provider_service(pg_pool.clone());
    let access_control_provider = build_access_control_provider_service(pg_pool.clone());
    let api_keys_provider = build_api_keys_provider_service(pg_pool.clone());
    let tier_configs_provider = build_tier_configs_provider_service(pg_pool.clone());
    let usage_provider = build_usage_provider_service(usage_data_provider.clone());
    let stripe_service = Arc::new(StripeService::new(
        &config_data.stripe.secret_key,
        config_data.stripe.webhook_secret.clone(),
        config_data.stripe.portal_url.clone(),
    ));
    let subscriptions_provider = build_subscriptions_provider_service(
        pg_pool.clone(),
        stripe_service.clone(),
        tier_configs_provider.clone(),
    );
    let products_provider =
        build_products_provider_service(pg_pool.clone(), stripe_service.clone());
    let purchased_products_data_provider: Arc<dyn PurchasedProductsDataProvider> =
        Arc::new(PurchasedProductsDataProviderService::new(pg_pool.clone()));
    let purchased_products_provider: Arc<dyn PurchasedProductsProvider> = Arc::new(
        PurchasedProductsProviderService::new(purchased_products_data_provider.clone()),
    );

    let rate_limits_data_provider: Arc<dyn RateLimitsDataProvider> =
        Arc::new(RateLimitsDataProviderService::new(pg_pool.clone()));
    let rate_limits_provider: Arc<dyn RateLimitsConfigProvider> = Arc::new(
        RateLimitsProviderService::new(rate_limits_data_provider.clone(), mux_connection.clone()),
    );
    let billing_mode_provider = build_billing_mode_provider_service(
        pg_pool.clone(),
        mux_connection.clone(),
        subscriptions_provider.clone(),
        tier_configs_provider.clone(),
        rate_limits_provider.clone(),
    );

    let org_provider = build_org_provider_service(
        pg_pool.clone(),
        billing_mode_provider.clone(),
        rate_limits_provider.clone(),
        Arc::new(RateLimitServiceImpl::new(rate_limits_data_provider.clone(), usage_data_provider.clone(), mux_connection.clone())),
        config_data.organization.default_token_bucket_size,
    );
    let invoices_provider = build_invoices_provider_service(pg_pool.clone());
    let jwt_provider = Arc::new(
        Hs256JwtProvider::new(
            &config_data.auth.jwt_secret,
            config_data.auth.access_token_ttl_seconds,
            config_data.auth.verification_token_ttl_seconds,
            config_data.auth.issuer.clone(),
            config_data.auth.audience.clone(),
        )
        .expect("failed to initialize jwt provider"),
    );

    let email_sender: Arc<dyn backend::auth::email_sender::EmailSender> =
        if config_data.smtp.enabled {
            let smtp_config = SmtpEmailSenderConfig {
                enabled: config_data.smtp.enabled,
                host: config_data.smtp.host.clone(),
                port: config_data.smtp.port,
                username: config_data.smtp.username.clone(),
                password: config_data.smtp.password.clone(),
                from_address: config_data.smtp.from_address.clone(),
                verification_base_url: config_data.smtp.verification_base_url.clone(),
                use_starttls: config_data.smtp.use_starttls,
            };

            let sender = SmtpEmailSender::new(smtp_config).map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::Other, format!("smtp init failed: {e}"))
            })?;
            Arc::new(sender)
        } else {
            Arc::new(MockEmailSender::new(
                config_data.smtp.from_address.clone(),
                config_data.smtp.verification_base_url.clone(),
            ))
        };

    let prometheus_metrics = PrometheusMetricsBuilder::new("user_portal_service")
        .endpoint("/metrics/system")
        .build()
        .expect("Failed to create Prometheus metrics handler");

    let swagger_config = utoipa_swagger_ui::Config::from("/api-doc/openapi.json").use_base_layout();

    let server_address = config_data.server.address;
    let worker_count = config_data.server.worker_count;

    let server = HttpServer::new(move || {
        let jwt_middleware = JwtAuthMiddleware::new(jwt_provider.clone());
        let permissions_middleware = PermissionsContextMiddleware::new(
            access_control_provider.clone(),
            user_provider.clone(),
        );
        let org_context_middleware = OrganizationContextMiddleware::new(org_provider.clone());
        let org_data = OrganizationsAppData {
            organization_provider: org_provider.clone(),
        };
        let me_data = MeAppData {
            me_data_provider: Arc::new(MeProviderService::new(
                user_provider.clone(),
                memberships_provider.clone(),
                membership_roles_provider.clone(),
                roles_provider.clone(),
                org_provider.clone(),
                access_control_provider.clone(),
                jwt_provider.clone(),
            )),
        };
        let user_data = UsersAppData {
            user_provider: user_provider.clone(),
        };
        let memberships_data = MembershipsAppData {
            memberships_provider: memberships_provider.clone(),
        };
        let members_data = MembersAppData {
            members_provider: members_provider.clone(),
        };
        let roles_data = RolesAppData {
            roles_provider: roles_provider.clone(),
        };
        let permissions_data = PermissionsAppData {
            permissions_provider: permissions_provider.clone(),
        };
        let role_permissions_data = RolePermissionsAppData {
            role_permissions_provider: role_permissions_provider.clone(),
        };
        let membership_roles_data = MembershipRolesAppData {
            membership_roles_provider: membership_roles_provider.clone(),
        };
        let api_keys_data = ApiKeysAppData {
            api_keys_provider: api_keys_provider.clone(),
        };
        let subscriptions_data = SubscriptionsAppData {
            subscriptions_provider: subscriptions_provider.clone(),
        };
        let products_data = ProductsAppData {
            products_provider: products_provider.clone(),
        };
        let purchased_products_data = PurchasedProductsAppData {
            purchased_products_provider: purchased_products_provider.clone(),
            stripe: stripe_service.clone(),
        };
        let rate_limits_data = RateLimitsAppData {
            rate_limits_provider: rate_limits_provider.clone(),
        };
        let billing_mode_data = BillingModeAppData {
            billing_mode_provider: billing_mode_provider.clone(),
        };
        let tier_configs_data = TierConfigsAppData {
            tier_configs_provider: tier_configs_provider.clone(),
        };
        let checkout_provider: Arc<dyn CheckoutProvider> = Arc::new(CheckoutProviderService::new(
            org_provider.clone(),
            products_provider.clone(),
            tier_configs_provider.clone(),
            stripe_service.clone(),
        ));
        let checkout_data = web::Data::new(CheckoutAppData { checkout_provider });
        let invoices_data = web::Data::new(InvoicesAppData {
            invoices_provider: invoices_provider.clone(),
        });
        let usage_data = web::Data::new(UsageAppData {
            usage_provider: usage_provider.clone(),
        });
        let rate_limit_service = Arc::new(RateLimitServiceImpl::new(
            rate_limits_data_provider.clone(),
            usage_data_provider.clone(),
            mux_connection.clone(),
        ));
        let webhook_service: Arc<dyn StripeWebhookHandler> =
            Arc::new(StripeWebhookServiceImpl::new(
                purchased_products_data_provider.clone(),
                subscriptions_provider.clone(),
                invoices_provider.clone(),
                rate_limit_service,
            ));
        let stripe_webhook_data = StripeWebhookAppData {
            stripe: stripe_service.clone(),
            webhook_service,
        };
        let auth_data = AuthAppData {
            auth_provider: Arc::new(AuthProviderService::new(
                user_provider.clone(),
                memberships_provider.clone(),
                roles_provider.clone(),
                membership_roles_provider.clone(),
                jwt_provider.clone(),
                email_sender.clone(),
            )),
            jwt_provider: jwt_provider.clone(),
        };
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
            .wrap(prometheus_metrics.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{tail:.*}")
                    .config(swagger_config.clone())
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .service(
                web::scope("/auth")
                    .app_data(web::Data::new(auth_data))
                    .configure(auth_controller::init),
            )
            .service(
                web::scope("/organizations")
                    .app_data(web::Data::new(org_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(organization_controller::init),
            )
            .service(
                web::scope("/v1/me")
                    .app_data(web::Data::new(me_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(me_controller::init),
            )
            .service(
                web::scope("/api-keys")
                    .app_data(web::Data::new(api_keys_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(org_context_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(api_keys_controller::init),
            )
            .service(
                web::scope("/memberships")
                    .app_data(web::Data::new(memberships_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(memberships_controller::init),
            )
            .service(
                web::scope("/members")
                    .app_data(web::Data::new(members_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(org_context_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(members_controller::init),
            )
            .service(
                web::scope("/roles")
                    .app_data(web::Data::new(roles_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(roles_controller::init),
            )
            .service(
                web::scope("/permissions")
                    .app_data(web::Data::new(permissions_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(permissions_controller::init),
            )
            .service(
                web::scope("/role-permissions")
                    .app_data(web::Data::new(role_permissions_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(role_permissions_controller::init),
            )
            .service(
                web::scope("/membership-roles")
                    .app_data(web::Data::new(membership_roles_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(membership_roles_controller::init),
            )
            .service(
                web::scope("/subscriptions")
                    .app_data(web::Data::new(subscriptions_data))
                    .app_data(checkout_data.clone())
                    .wrap(permissions_middleware.clone())
                    .wrap(org_context_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(subscriptions_controller::init),
            )
            .service(
                web::scope("/products")
                    .app_data(web::Data::new(products_data))
                    .app_data(checkout_data.clone())
                    .wrap(permissions_middleware.clone())
                    .wrap(org_context_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(products_controller::init),
            )
            .service(
                web::scope("/purchased-products")
                    .app_data(web::Data::new(purchased_products_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(org_context_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(purchased_products_controller::init),
            )
            .service(
                web::scope("/rate-limits")
                    .app_data(web::Data::new(rate_limits_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(rate_limits_config_controller::init),
            )
            .service(
                web::scope("/billing-mode")
                    .app_data(web::Data::new(billing_mode_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(org_context_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(billing_mode_controller::init),
            )
            .service(
                web::scope("/tier-configs")
                    .app_data(web::Data::new(tier_configs_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(tier_configs_controller::init),
            )
            .service(
                web::scope("/invoices")
                    .app_data(invoices_data.clone())
                    .wrap(permissions_middleware.clone())
                    .wrap(org_context_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(invoices_controller::init),
            )
            .service(
                web::scope("/usage")
                    .app_data(usage_data.clone())
                    .wrap(permissions_middleware.clone())
                    .wrap(org_context_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(usage_controller::init),
            )
            .service(
                // No JWT — Stripe signature validates authenticity
                web::scope("/webhooks/stripe")
                    .app_data(web::Data::new(stripe_webhook_data))
                    .configure(stripe_webhook_controller::init),
            )
            .service(
                web::scope("/users")
                    .app_data(web::Data::new(user_data))
                    .wrap(permissions_middleware.clone())
                    .wrap(jwt_middleware.clone())
                    .configure(user_controller::init),
            )
    })
    .workers(worker_count)
    .bind(server_address)?;

    info!("User portal running at http://{server_address}");
    let ctrl_c_token = token.clone();
    tokio::spawn(async move {
        if let Ok(_) = tokio::signal::ctrl_c().await {
            info!("Shutdown signal received (Ctrl+C)");
            ctrl_c_token.cancel();
        }
    });
    tokio::select! {
        server_result=server.run()=>{
            tracing::info!("Actix server stopped: {:?}", server_result);
        }
        usage_worker_result=usage_worker=>{
            tracing::info!("Usage worker stopped: {:?}", usage_worker_result);
        }
        sync_worker_result=sync_worker=>{
            tracing::info!("Sync worker stopped: {:?}", sync_worker_result);
        }
    }
    tracing::info!("👋 Shutdown complete.");
    Ok(())
}

async fn build_pg_pool(config_data: &AppConfig) -> PgPool {
    PgPool::connect(&config_data.database.url)
        .await
        .expect("Could not connect to postgres")
}

fn build_org_provider_service(
    pool: PgPool,
    billing_mode_provider: Arc<dyn BillingModeProvider>,
    rate_limits_config_provider: Arc<dyn RateLimitsConfigProvider>,
    rate_limit_service: Arc<dyn backend::stripe::rate_limit_service::rate_limit_service::RateLimitService>,
    default_token_bucket_size: u64,
) -> Arc<dyn OrganizationProvider> {
    let data_provider = Arc::new(OrganizationDataProviderService::new(pool));
    Arc::new(OrganizationProviderService::new(
        data_provider,
        billing_mode_provider,
        rate_limits_config_provider,
        rate_limit_service,
        default_token_bucket_size,
    ))
}

fn build_user_provider_service(pool: PgPool) -> Arc<dyn UserProvider> {
    let data_provider = Arc::new(UserDataProviderService::new(pool));
    Arc::new(UserProviderService::new(data_provider))
}

fn build_api_keys_provider_service(pool: PgPool) -> Arc<dyn ApiKeysProvider> {
    let data_provider = Arc::new(ApiKeysDataProviderService::new(pool));
    Arc::new(ApiKeysService::new(data_provider))
}

fn build_memberships_provider_service(pool: PgPool) -> Arc<dyn MembershipsProvider> {
    let data_provider = Arc::new(MembershipsDataProviderService::new(pool));
    Arc::new(MembershipsProviderService::new(data_provider))
}

fn build_members_provider_service(pool: PgPool) -> Arc<dyn MembersProvider> {
    let data_provider = Arc::new(MembersDataProviderService::new(pool));
    Arc::new(MembersProviderService::new(data_provider))
}

fn build_roles_provider_service(pool: PgPool) -> Arc<dyn RolesProvider> {
    let data_provider = Arc::new(RolesDataProviderService::new(pool));
    Arc::new(RolesProviderService::new(data_provider))
}

fn build_permissions_provider_service(pool: PgPool) -> Arc<dyn PermissionsProvider> {
    let data_provider = Arc::new(PermissionsDataProviderService::new(pool));
    Arc::new(PermissionsProviderService::new(data_provider))
}

fn build_role_permissions_provider_service(pool: PgPool) -> Arc<dyn RolePermissionsProvider> {
    let data_provider = Arc::new(RolePermissionsDataProviderService::new(pool));
    Arc::new(RolePermissionsProviderService::new(data_provider))
}

fn build_membership_roles_provider_service(pool: PgPool) -> Arc<dyn MembershipRolesProvider> {
    let data_provider = Arc::new(MembershipRolesDataProviderService::new(pool));
    Arc::new(MembershipRolesProviderService::new(data_provider))
}

fn build_access_control_provider_service(pool: PgPool) -> Arc<dyn AccessControlProvider> {
    let data_provider = Arc::new(AccessControlDataProviderService::new(pool));
    Arc::new(AccessControlProviderService::new(data_provider))
}

fn build_subscriptions_provider_service(
    pool: PgPool,
    stripe_service: Arc<StripeService>,
    tier_configs_provider: Arc<dyn TierConfigsProvider>,
) -> Arc<dyn SubscriptionsProvider> {
    let data_provider = Arc::new(SubscriptionsDataProviderService::new(pool));
    Arc::new(SubscriptionsProviderService::new(
        data_provider,
        stripe_service,
        tier_configs_provider,
    ))
}

fn build_products_provider_service(
    pool: PgPool,
    stripe_service: Arc<StripeService>,
) -> Arc<dyn ProductsProvider> {
    let data_provider = Arc::new(ProductsDataProviderService::new(pool));
    Arc::new(ProductsProviderService::new(data_provider, stripe_service))
}

fn build_billing_mode_provider_service(
    pool: PgPool,
    redis: deadpool_redis::redis::aio::MultiplexedConnection,
    subscriptions_provider: Arc<dyn SubscriptionsProvider>,
    tier_configs_provider: Arc<dyn TierConfigsProvider>,
    rate_limits_config_provider: Arc<dyn RateLimitsConfigProvider>,
) -> Arc<dyn BillingModeProvider> {
    let data_provider = Arc::new(BillingModeDataProviderService::new(pool));
    Arc::new(BillingModeProviderService::new(
        data_provider,
        redis,
        subscriptions_provider,
        tier_configs_provider,
        rate_limits_config_provider,
    ))
}

fn build_tier_configs_provider_service(pool: PgPool) -> Arc<dyn TierConfigsProvider> {
    let data_provider = Arc::new(TierConfigsDataProviderService::new(pool));
    Arc::new(TierConfigsProviderService::new(data_provider))
}

fn build_invoices_provider_service(pool: PgPool) -> Arc<dyn InvoicesProvider> {
    let data_provider = Arc::new(InvoicesDataProviderService::new(pool));
    Arc::new(InvoicesProviderService::new(data_provider))
}

fn build_usage_provider_service(data_provider: Arc<dyn UsageDataProvider>) -> Arc<dyn UsageProvider> {
    Arc::new(UsageProviderService::new(data_provider))
}

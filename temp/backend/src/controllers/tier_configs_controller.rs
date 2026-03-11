use crate::domain::Tier;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;
use crate::tier_configs::tier_configs_app_data::TierConfigsAppData;
use crate::tier_configs::tier_configs_provider::UpdateTierConfigParams;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Response ──────────────────────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
pub struct TierConfigResult {
    pub tier: Tier,
    pub token_limit: i64,
    pub window_size_secs: i64,
    pub stripe_price_id: Option<String>,
    pub price_cents: i64,
    pub currency: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ListTierConfigsResponse {
    pub tier_configs: Vec<TierConfigResult>,
}

#[derive(Serialize, ToSchema)]
pub struct GetTierConfigResponse {
    pub tier_config: TierConfigResult,
}

// ── Input ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct UpdateTierConfigInput {
    pub token_limit: Option<i64>,
    pub window_size_secs: Option<i64>,
    pub stripe_price_id: Option<String>,
    pub price_cents: Option<i64>,
    pub currency: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

// ── Routing ───────────────────────────────────────────────────────────────────

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_tier_configs)
        .service(get_tier_config)
        .service(update_tier_config);
}

// ── Handlers ──────────────────────────────────────────────────────────────────

#[get("")]
pub async fn list_tier_configs(
    perms: PermissionsContext,
    app_state: web::Data<TierConfigsAppData>,
) -> HttpResponse {
    if !perms.has("tier_configs:read") {
        return HttpResponse::Forbidden().finish();
    }

    match app_state.tier_configs_provider.list().await {
        Ok(rows) => {
            let tier_configs = rows
                .into_iter()
                .map(|r| TierConfigResult {
                    tier: r.tier,
                    token_limit: r.token_limit,
                    window_size_secs: r.window_size_secs,
                    stripe_price_id: r.stripe_price_id,
                    price_cents: r.price_cents,
                    currency: r.currency,
                    display_name: r.display_name,
                    description: r.description,
                })
                .collect();
            HttpResponse::Ok().json(ListTierConfigsResponse { tier_configs })
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/{tier}")]
pub async fn get_tier_config(
    perms: PermissionsContext,
    tier: web::Path<String>,
    app_state: web::Data<TierConfigsAppData>,
) -> HttpResponse {
    if !perms.has("tier_configs:read") {
        return HttpResponse::Forbidden().finish();
    }

    match app_state.tier_configs_provider.get(&tier).await {
        Ok(Some(r)) => HttpResponse::Ok().json(GetTierConfigResponse {
            tier_config: TierConfigResult {
                tier: r.tier,
                token_limit: r.token_limit,
                window_size_secs: r.window_size_secs,
                stripe_price_id: r.stripe_price_id,
                price_cents: r.price_cents,
                currency: r.currency,
                display_name: r.display_name,
                description: r.description,
            },
        }),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/{tier}")]
pub async fn update_tier_config(
    perms: PermissionsContext,
    tier: web::Path<String>,
    payload: web::Json<UpdateTierConfigInput>,
    app_state: web::Data<TierConfigsAppData>,
) -> HttpResponse {
    if !perms.has("tier_configs:update") {
        return HttpResponse::Forbidden().finish();
    }

    let params = UpdateTierConfigParams {
        token_limit: payload.token_limit,
        window_size_secs: payload.window_size_secs,
        stripe_price_id: payload.stripe_price_id.clone(),
        price_cents: payload.price_cents,
        currency: payload.currency.clone(),
        display_name: payload.display_name.clone(),
        description: payload.description.clone(),
    };

    match app_state.tier_configs_provider.update(&tier, params).await {
        Ok(Some(r)) => HttpResponse::Ok().json(GetTierConfigResponse {
            tier_config: TierConfigResult {
                tier: r.tier,
                token_limit: r.token_limit,
                window_size_secs: r.window_size_secs,
                stripe_price_id: r.stripe_price_id,
                price_cents: r.price_cents,
                currency: r.currency,
                display_name: r.display_name,
                description: r.description,
            },
        }),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

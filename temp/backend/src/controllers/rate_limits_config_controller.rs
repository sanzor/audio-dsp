use actix_web::{get, post, put, web, HttpResponse};
use serde::Deserialize;
use tracing::{error, info, warn};
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;
use crate::rate_limits_config::rate_limit_config_input::RateLimitConfigInput;
use crate::rate_limits_config::rate_limit_config_response::RateLimitConfigResponse;
use crate::rate_limits_config::rate_limits_app_data::RateLimitsAppData;
#[derive(Deserialize, ToSchema, Debug)]
pub struct RateLimitPath {
    pub org_id: OrganizationId,
}

#[utoipa::path(
    get,
    path = "/rate-limits/{org_id}/config",
    tag = "RateLimits",
    params(
        ("org_id" = OrganizationId, Path, description = "Organization id"),
    ),
    responses(
        (status = 200, description = "Rate limit config retrieved", body = RateLimitConfigResponse),
        (status = 404, description = "Config not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/{org_id}/config")]
pub async fn get_rate_limit_config(
    perms: PermissionsContext,
    path: web::Path<RateLimitPath>,
    app_state: web::Data<RateLimitsAppData>,
) -> HttpResponse {
    if !perms.has("rate_limits:read") {
        return HttpResponse::Forbidden().body("missing required permission: rate_limits:read");
    }
    let org_id = path.into_inner().org_id;
    info!(org_id, "get rate limit config request received");

    match app_state.rate_limits_provider.get_config(org_id).await {
        Ok(Some(result)) => HttpResponse::Ok().json(result),
        Ok(None) => HttpResponse::NotFound().body("rate limit config not found"),
        Err(e) => {
            error!(org_id, error = %e, "get rate limit config failed");
            HttpResponse::InternalServerError().body("failed to fetch rate limit config")
        }
    }
}

#[utoipa::path(
    put,
    path = "/rate-limits/{org_id}/config",
    tag = "RateLimits",
    request_body = RateLimitConfigInput,
    params(
        ("org_id" = OrganizationId, Path, description = "Organization id"),
    ),
    responses(
        (status = 200, description = "Rate limit config updated", body = RateLimitConfigResponse),
        (status = 400, description = "Invalid input parameters"),
        (status = 500, description = "Internal server error"),
    )
)]
#[put("/{org_id}/config")]
pub async fn update_rate_limit_config(
    perms: PermissionsContext,
    path: web::Path<RateLimitPath>,
    payload: web::Json<RateLimitConfigInput>,
    app_state: web::Data<RateLimitsAppData>,
) -> HttpResponse {
    if !perms.has("rate_limits:update") {
        return HttpResponse::Forbidden().body("missing required permission: rate_limits:update");
    }
    let org_id = path.into_inner().org_id;
    let input = payload.into_inner();
    info!(org_id, "update rate limit config request received");

    match input {
        RateLimitConfigInput::TokenBucket { limit } if limit == 0 => {
            warn!(org_id, "token bucket limit must be greater than zero");
            return HttpResponse::BadRequest().body("limit must be greater than zero");
        }
        RateLimitConfigInput::TokenWindow {
            limit,
            window_size_secs,
        } if limit == 0 || window_size_secs == 0 => {
            warn!(org_id, "token window values must be greater than zero");
            return HttpResponse::BadRequest().body("limit and window_size_secs must be > 0");
        }
        _ => {}
    }

    match app_state
        .rate_limits_provider
        .upsert_config(org_id, input)
        .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            error!(org_id, error = %e, "update rate limit config failed");
            HttpResponse::InternalServerError().body("failed to update rate limit config")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_rate_limit_config)
        .service(update_rate_limit_config);
}

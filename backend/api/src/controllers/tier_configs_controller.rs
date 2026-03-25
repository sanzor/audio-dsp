use actix_web::{get, put, web, HttpResponse};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use crate::domain::tier::Tier;
use crate::middlewares::jwt::jwt_context::JwtContext;
use crate::tier_configs::tier_configs_app_data::TierConfigsAppData;
use crate::tier_configs::update_tier_config_params::UpdateTierConfigParams;

#[derive(Deserialize, IntoParams)]
pub struct TierPath {
    pub tier: Tier,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateTierConfigInput {
    pub max_projects: Option<i32>,
    pub max_tracks_per_project: Option<i32>,
    pub max_storage_bytes: Option<i64>,
}

#[utoipa::path(get, path = "/v1/tier-configs", tag = "TierConfigs",
    responses((status = 200, body = Vec<crate::domain::db::db_tier_config::DbTierConfig>)))]
#[get("")]
pub async fn list_tier_configs(app: web::Data<TierConfigsAppData>) -> HttpResponse {
    match app.tier_configs_provider.list_tier_configs().await {
        Ok(configs) => HttpResponse::Ok().json(configs),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(get, path = "/v1/tier-configs/{tier}", tag = "TierConfigs",
    params(TierPath),
    responses((status = 200, body = crate::domain::db::db_tier_config::DbTierConfig), (status = 404)))]
#[get("/{tier}")]
pub async fn get_tier_config(
    path: web::Path<TierPath>,
    app: web::Data<TierConfigsAppData>,
) -> HttpResponse {
    let tier = path.into_inner().tier;
    match app.tier_configs_provider.get_tier_config(&tier).await {
        Ok(Some(cfg)) => HttpResponse::Ok().json(cfg),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(put, path = "/v1/tier-configs/{tier}", tag = "TierConfigs",
    params(TierPath),
    request_body = UpdateTierConfigInput,
    responses((status = 200, body = crate::domain::db::db_tier_config::DbTierConfig), (status = 401), (status = 403), (status = 404)))]
#[put("/{tier}")]
pub async fn update_tier_config(
    auth: JwtContext,
    path: web::Path<TierPath>,
    payload: web::Json<UpdateTierConfigInput>,
    app: web::Data<TierConfigsAppData>,
) -> HttpResponse {
    if !auth.is_admin {
        return HttpResponse::Forbidden().finish();
    }
    let tier = path.into_inner().tier;
    let input = payload.into_inner();
    let params = UpdateTierConfigParams {
        max_projects: input.max_projects,
        max_tracks_per_project: input.max_tracks_per_project,
        max_storage_bytes: input.max_storage_bytes,
    };
    match app.tier_configs_provider.update_tier_config(&tier, params).await {
        Ok(Some(cfg)) => HttpResponse::Ok().json(cfg),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_tier_configs)
        .service(get_tier_config)
        .service(update_tier_config);
}

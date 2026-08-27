use actix_web::{get, post, web, HttpResponse};
use utoipa::IntoParams;
use serde::Deserialize;
use crate::middlewares::jwt::jwt_context::JwtContext;
use crate::usage::usage_app_data::UsageAppData;

#[derive(Deserialize, IntoParams)]
pub struct UserIdQuery {
    pub user_id: Option<domain::domain_user::UserId>,
}

#[utoipa::path(get, path = "/v1/usage/mine", tag = "Usage",
    responses((status = 200, body = crate::domain::db::db_usage::DbUsage), (status = 401), (status = 404)))]
#[get("/mine")]
pub async fn get_my_usage(
    auth: JwtContext,
    app: web::Data<UsageAppData>,
) -> HttpResponse {
    match app.usage_provider.get_usage(auth.user_id).await {
        Ok(Some(u)) => HttpResponse::Ok().json(u),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(post, path = "/v1/usage/mine/refresh", tag = "Usage",
    responses((status = 200, body = crate::domain::db::db_usage::DbUsage), (status = 401)))]
#[post("/mine/refresh")]
pub async fn refresh_my_usage(
    auth: JwtContext,
    app: web::Data<UsageAppData>,
) -> HttpResponse {
    match app.usage_provider.refresh_usage(auth.user_id).await {
        Ok(u) => HttpResponse::Ok().json(u),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(post, path = "/v1/usage/refresh", tag = "Usage",
    params(UserIdQuery),
    responses((status = 200, body = crate::domain::db::db_usage::DbUsage), (status = 401), (status = 403)))]
#[post("/refresh")]
pub async fn admin_refresh_usage(
    auth: JwtContext,
    query: web::Query<UserIdQuery>,
    app: web::Data<UsageAppData>,
) -> HttpResponse {
    if !auth.is_admin {
        return HttpResponse::Forbidden().finish();
    }
    let user_id = match query.user_id {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("user_id required"),
    };
    match app.usage_provider.refresh_usage(user_id).await {
        Ok(u) => HttpResponse::Ok().json(u),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_my_usage)
        .service(refresh_my_usage)
        .service(admin_refresh_usage);
}

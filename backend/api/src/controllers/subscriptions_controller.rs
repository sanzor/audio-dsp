use crate::domain::db::db_subscription::SubscriptionId;
use crate::domain::tier::Tier;
use crate::middlewares::jwt::jwt_context::JwtContext;
use crate::subscriptions::create_subscription_params::CreateSubscriptionParams;
use crate::subscriptions::subscriptions_app_data::SubscriptionsAppData;
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, ToSchema)]
pub struct CreateSubscriptionInput {
    pub tier: Tier,
    pub expires_at: Option<String>,
}

#[derive(Deserialize, IntoParams)]
pub struct SubscriptionIdPath {
    pub id: SubscriptionId,
}

#[utoipa::path(post, path = "/v1/subscriptions", tag = "Subscriptions",
    request_body = CreateSubscriptionInput,
    responses((status = 201, body = crate::domain::db::db_subscription::DbSubscription), (status = 401)))]
#[post("")]
pub async fn create_subscription(
    auth: JwtContext,
    payload: web::Json<CreateSubscriptionInput>,
    app: web::Data<SubscriptionsAppData>,
) -> HttpResponse {
    let input = payload.into_inner();
    let params = CreateSubscriptionParams {
        user_id: auth.user_id,
        tier: input.tier,
        expires_at: input.expires_at,
    };
    match app.subscriptions_provider.create_subscription(params).await {
        Ok(sub) => HttpResponse::Created().json(sub),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(get, path = "/v1/subscriptions/mine", tag = "Subscriptions",
    responses((status = 200, body = Vec<crate::domain::db::db_subscription::DbSubscription>), (status = 401)))]
#[get("/mine")]
pub async fn list_my_subscriptions(
    auth: JwtContext,
    app: web::Data<SubscriptionsAppData>,
) -> HttpResponse {
    match app.subscriptions_provider.list_by_user(auth.user_id).await {
        Ok(subs) => HttpResponse::Ok().json(subs),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(get, path = "/v1/subscriptions/mine/active", tag = "Subscriptions",
    responses((status = 200, body = crate::domain::db::db_subscription::DbSubscription), (status = 401), (status = 404)))]
#[get("/mine/active")]
pub async fn get_my_active_subscription(
    auth: JwtContext,
    app: web::Data<SubscriptionsAppData>,
) -> HttpResponse {
    match app
        .subscriptions_provider
        .get_active_subscription_for_user(auth.user_id)
        .await
    {
        Ok(Some(sub)) => HttpResponse::Ok().json(sub),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(post, path = "/v1/subscriptions/{id}/deactivate", tag = "Subscriptions",
    params(SubscriptionIdPath),
    responses((status = 204), (status = 401), (status = 403), (status = 404)))]
#[post("/{id}/deactivate")]
pub async fn deactivate_subscription(
    auth: JwtContext,
    path: web::Path<SubscriptionIdPath>,
    app: web::Data<SubscriptionsAppData>,
) -> HttpResponse {
    if !auth.is_admin {
        return HttpResponse::Forbidden().finish();
    }
    let id = path.into_inner().id;
    match app.subscriptions_provider.deactivate_subscription(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_subscription)
        .service(list_my_subscriptions)
        .service(get_my_active_subscription)
        .service(deactivate_subscription);
}

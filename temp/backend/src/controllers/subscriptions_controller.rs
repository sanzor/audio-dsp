use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

use crate::checkout::checkout_app_data::CheckoutAppData;
use crate::checkout::checkout_error::CheckoutError;
use crate::checkout::create_subscription_checkout_params::CreateSubscriptionCheckoutParams;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_subscription::SubscriptionId;
use crate::domain::Tier;
use crate::middlewares::organization::organization_context::OrganizationContext;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;

use crate::subscriptions::subscriptions_app_data::SubscriptionsAppData;
use crate::subscriptions::update_subscription_params::UpdateSubscriptionParams;
use crate::subscriptions::DbSubscription;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SubscriptionResult {
    pub id: SubscriptionId,
    pub org_id: OrganizationId,
    pub tier: Tier,
    pub stripe_subscription_id: Option<String>,
    pub status: String,
    pub current_period_end: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SubscriptionsResult {
    pub subscriptions: Vec<SubscriptionResult>,
    pub total: i64,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SubscriptionListQuery {
    pub index: Option<i64>,
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SubscriptionPath {
    pub org_id: OrganizationId,
}

fn map_subscription(subscription: DbSubscription) -> SubscriptionResult {
    SubscriptionResult {
        id: subscription.id,
        org_id: subscription.org_id,
        tier: subscription.tier,
        stripe_subscription_id: subscription.stripe_subscription_id,
        status: subscription.status,
        current_period_end: subscription.current_period_end,
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateSubscriptionInput {
    pub tier: Option<Tier>,
    pub stripe_subscription_id: Option<String>,
    pub status: Option<String>,
    pub current_period_end: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateSubscriptionResult {
    pub subscription: SubscriptionResult,
}

#[utoipa::path(
    put,
    path = "/subscriptions/{org_id}",
    tag = "Subscriptions",
    request_body = UpdateSubscriptionInput,
    params(
        ("org_id" = u64, Path, description = "Organization id"),
    ),
    responses(
        (status = 200, description = "Subscription updated", body = UpdateSubscriptionResult),
        (status = 404, description = "Subscription not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[put("/{org_id}")]
pub async fn update_subscription(
    perms: PermissionsContext,
    path: web::Path<SubscriptionPath>,
    payload: web::Json<UpdateSubscriptionInput>,
    app_state: web::Data<SubscriptionsAppData>,
) -> HttpResponse {
    if !perms.has("subscriptions:update") {
        return HttpResponse::Forbidden().body("missing required permission: subscriptions:update");
    }
    let org_id = path.into_inner().org_id;
    info!(org_id, "update subscription request received");
    let input = payload.into_inner();

    if input.tier.is_none()
        && input.stripe_subscription_id.is_none()
        && input.status.is_none()
        && input.current_period_end.is_none()
    {
        warn!(org_id, "update subscription rejected: no fields provided");
        return HttpResponse::BadRequest().body("at least one field is required");
    }

    let params = UpdateSubscriptionParams {
        tier: input.tier,
        stripe_subscription_id: input.stripe_subscription_id,
        status: input.status,
        current_period_end: input.current_period_end,
    };

    match app_state
        .subscriptions_provider
        .update_subscription(org_id, params)
        .await
    {
        Ok(Some(result)) => HttpResponse::Ok().json(UpdateSubscriptionResult {
            subscription: map_subscription(result),
        }),
        Ok(None) => HttpResponse::NotFound().body("subscription not found"),
        Err(e) => {
            error!(org_id, error = %e, "update subscription failed");
            HttpResponse::InternalServerError().body("failed to update subscription")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/subscriptions/{org_id}",
    tag = "Subscriptions",
    params(
        ("org_id" = u64, Path, description = "Organization id"),
    ),
    responses(
        (status = 204, description = "Subscription deleted"),
        (status = 404, description = "Subscription not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[delete("/{org_id}")]
pub async fn delete_subscription(
    perms: PermissionsContext,
    path: web::Path<SubscriptionPath>,
    app_state: web::Data<SubscriptionsAppData>,
) -> HttpResponse {
    if !perms.has("subscriptions:delete") {
        return HttpResponse::Forbidden().body("missing required permission: subscriptions:delete");
    }
    let org_id = path.into_inner().org_id;
    info!(org_id, "delete subscription request received");

    match app_state
        .subscriptions_provider
        .delete_subscription(org_id)
        .await
    {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("subscription not found"),
        Err(e) => {
            error!(org_id, error = %e, "delete subscription failed");
            HttpResponse::InternalServerError().body("failed to delete subscription")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GetSubscriptionResult {
    pub subscription: SubscriptionResult,
}

#[utoipa::path(
    get,
    path = "/subscriptions/{org_id}",
    tag = "Subscriptions",
    params(
        ("org_id" = u64, Path, description = "Organization id"),
    ),
    responses(
        (status = 200, description = "Subscription retrieved", body = GetSubscriptionResult),
        (status = 404, description = "Subscription not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/{org_id}")]
pub async fn get_subscription(
    perms: PermissionsContext,
    path: web::Path<SubscriptionPath>,
    app_state: web::Data<SubscriptionsAppData>,
) -> HttpResponse {
    if !perms.has("subscriptions:read") {
        return HttpResponse::Forbidden().body("missing required permission: subscriptions:read");
    }
    let org_id = path.into_inner().org_id;
    info!(org_id, "get subscription request received");

    match app_state
        .subscriptions_provider
        .get_subscription(org_id)
        .await
    {
        Ok(Some(result)) => HttpResponse::Ok().json(GetSubscriptionResult {
            subscription: map_subscription(result),
        }),
        Ok(None) => HttpResponse::NotFound().body("subscription not found"),
        Err(e) => {
            error!(org_id, error = %e, "get subscription failed");
            HttpResponse::InternalServerError().body("failed to get subscription")
        }
    }
}

#[utoipa::path(
    get,
    path = "/subscriptions",
    tag = "Subscriptions",
    responses(
        (status = 200, description = "Subscriptions retrieved", body = SubscriptionsResult),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
pub async fn list_subscriptions(
    perms: PermissionsContext,
    query: web::Query<SubscriptionListQuery>,
    app_state: web::Data<SubscriptionsAppData>,
) -> HttpResponse {
    if !perms.has("subscriptions:read") {
        return HttpResponse::Forbidden().body("missing required permission: subscriptions:read");
    }
    let offset = query.index.unwrap_or(0).max(0) as usize;
    let limit = query.count.unwrap_or(20).clamp(1, 100) as usize;
    info!(offset, limit, "list subscriptions request received");

    match app_state.subscriptions_provider.list_subscriptions().await {
        Ok(result) => {
            let total = result.len() as i64;
            let page: Vec<_> = result
                .into_iter()
                .skip(offset)
                .take(limit)
                .map(map_subscription)
                .collect();
            HttpResponse::Ok().json(SubscriptionsResult {
                subscriptions: page,
                total,
            })
        }
        Err(e) => {
            error!(error = %e, "list subscriptions failed");
            HttpResponse::InternalServerError().body("failed to fetch subscriptions")
        }
    }
}

// ── Checkout ──────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct SubscriptionCheckoutInput {
    pub tier: Tier,
    /// Absolute URL Stripe should redirect to after successful payment.
    #[schema(example = "http://localhost:5173/org/dev-org/checkout/success")]
    pub success_url: Option<String>,
    /// Absolute URL Stripe should redirect to if the user cancels checkout.
    #[schema(example = "http://localhost:5173/org/dev-org/checkout/cancel")]
    pub cancel_url: Option<String>,
}

fn normalize_redirect_target(
    origin: Option<&str>,
    value: Option<String>,
    fallback_path: &str,
) -> Result<Option<String>, String> {
    let origin = origin
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.trim_end_matches('/'))
        .filter(|v| v.starts_with("http://") || v.starts_with("https://"));

    let Some(mut v) = value else {
        return Ok(origin.map(|o| format!("{o}{fallback_path}")));
    };

    v = v.trim().to_string();
    if v.is_empty() {
        return Err("redirect URL/path is empty".to_string());
    }

    let is_abs = v.starts_with("http://") || v.starts_with("https://");
    if is_abs {
        if let Some(o) = origin {
            let rest = v.strip_prefix(o).unwrap_or("");
            let boundary_ok = rest.is_empty()
                || rest.starts_with('/')
                || rest.starts_with('?')
                || rest.starts_with('#');
            if !boundary_ok {
                return Err("redirect URL must match request Origin".to_string());
            }
        }
        return Ok(Some(v));
    }

    if v.starts_with('/') {
        return Ok(origin.map(|o| format!("{o}{v}")).or(Some(v)));
    }

    Err("redirect URL must be absolute (http/https) or start with '/'".to_string())
}

#[derive(Serialize, ToSchema)]
pub struct CheckoutSessionResponse {
    pub checkout_url: String,
}

/// POST /subscriptions/checkout
/// Initiates a Stripe Checkout Session for a subscription tier upgrade.
/// Requires `subscriptions:create` permission and an org-scoped JWT.
#[post("/checkout")]
pub async fn subscription_checkout(
    perms: PermissionsContext,
    org_ctx: OrganizationContext,
    req: HttpRequest,
    payload: web::Json<SubscriptionCheckoutInput>,
    checkout: web::Data<CheckoutAppData>,
) -> HttpResponse {
    if !perms.has("subscriptions:create") {
        return HttpResponse::Forbidden().finish();
    }

    let org_id = OrganizationId::from(org_ctx.id);
    let payload = payload.into_inner();

    let origin = req
        .headers()
        .get("Origin")
        .and_then(|v| v.to_str().ok());
    let default_success_path = format!("/org/{}/checkout/success", org_ctx.slug);
    let default_cancel_path = format!("/org/{}/checkout/cancel", org_ctx.slug);

    let success_url =
        match normalize_redirect_target(origin, payload.success_url, &default_success_path) {
            Ok(v) => v,
            Err(msg) => return HttpResponse::UnprocessableEntity().body(msg),
        };
    let cancel_url =
        match normalize_redirect_target(origin, payload.cancel_url, &default_cancel_path) {
            Ok(v) => v,
            Err(msg) => return HttpResponse::UnprocessableEntity().body(msg),
        };

    let params = CreateSubscriptionCheckoutParams {
        org_id,
        org_slug: org_ctx.slug,
        tier: payload.tier,
        success_url,
        cancel_url,
    };

    match checkout
        .checkout_provider
        .create_subscription_checkout(params)
        .await
    {
        Ok(r) => HttpResponse::Ok().json(CheckoutSessionResponse {
            checkout_url: r.checkout_url,
        }),
        Err(CheckoutError::NotFound(msg)) => HttpResponse::NotFound().body(msg),
        Err(CheckoutError::InvalidInput(msg)) => HttpResponse::UnprocessableEntity().body(msg),
        Err(CheckoutError::Internal(msg)) => HttpResponse::InternalServerError().body(msg),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(update_subscription)
        .service(delete_subscription)
        .service(get_subscription)
        .service(list_subscriptions)
        .service(subscription_checkout);
}

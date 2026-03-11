use actix_web::{get, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use utoipa::ToSchema;

use crate::billing_mode::billing_mode::BillingMode;
use crate::billing_mode::billing_mode_app_data::BillingModeAppData;
use crate::domain::db::db_organization::OrganizationId;
use crate::middlewares::organization::organization_context::OrganizationContext;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;

// ── Shared output DTO ─────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct BillingModeResult {
    pub org_id: OrganizationId,
    pub mode: BillingMode,
}

fn map_mode(org_id: OrganizationId, mode: BillingMode) -> BillingModeResult {
    BillingModeResult { org_id, mode }
}

// ── GET /config ───────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GetBillingModeResponse {
    pub billing_mode: BillingModeResult,
}

#[utoipa::path(
    get,
    path = "/billing-mode/config",
    tag = "BillingMode",
    responses(
        (status = 200, description = "Billing mode retrieved", body = GetBillingModeResponse),
        (status = 404, description = "Config not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/config")]
pub async fn get_billing_mode(
    perms: PermissionsContext,
    org_ctx: OrganizationContext,
    app_state: web::Data<BillingModeAppData>,
) -> HttpResponse {
    if !perms.has("billing_mode:read") {
        return HttpResponse::Forbidden().body("missing required permission: billing_mode:read");
    }
    let org_id = org_ctx.id;
    info!(org_id, "get billing mode request received");

    match app_state.billing_mode_provider.get_mode(org_id).await {
        Ok(Some(mode)) => HttpResponse::Ok().json(GetBillingModeResponse {
            billing_mode: map_mode(org_id, mode),
        }),
        Ok(None) => HttpResponse::NotFound().body("billing mode config not found"),
        Err(e) => {
            error!(org_id, error = %e, "get billing mode failed");
            HttpResponse::InternalServerError().body("failed to fetch billing mode")
        }
    }
}

// ── PUT /config ───────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UpsertBillingModeInput {
    pub mode: BillingMode,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpsertBillingModeResponse {
    pub billing_mode: BillingModeResult,
}

#[utoipa::path(
    put,
    path = "/billing-mode/config",
    tag = "BillingMode",
    request_body = UpsertBillingModeInput,
    responses(
        (status = 200, description = "Billing mode updated", body = UpsertBillingModeResponse),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error"),
    )
)]
#[put("/config")]
pub async fn update_billing_mode(
    perms: PermissionsContext,
    org_ctx: OrganizationContext,
    payload: web::Json<UpsertBillingModeInput>,
    app_state: web::Data<BillingModeAppData>,
) -> HttpResponse {
    if !perms.has("billing_mode:update") {
        return HttpResponse::Forbidden().body("missing required permission: billing_mode:update");
    }
    let org_id = org_ctx.id;
    let input = payload.into_inner();
    info!(org_id, "update billing mode request received");

    match app_state
        .billing_mode_provider
        .upsert_mode(org_id, input.mode)
        .await
    {
        Ok(mode) => HttpResponse::Ok().json(UpsertBillingModeResponse {
            billing_mode: map_mode(org_id, mode),
        }),
        Err(e) => {
            error!(org_id, error = %e, "update billing mode failed");
            HttpResponse::InternalServerError().body("failed to update billing mode")
        }
    }
}

// ── Router ────────────────────────────────────────────────────────────────────

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_billing_mode).service(update_billing_mode);
}

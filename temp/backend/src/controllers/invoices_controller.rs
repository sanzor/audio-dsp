use actix_web::{delete, get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use utoipa::ToSchema;

use crate::domain::db::db_invoice::{DbInvoice, InvoiceId};
use crate::domain::db::db_organization::OrganizationId;
use crate::invoices::invoices_app_data::InvoicesAppData;
use crate::middlewares::organization::organization_context::OrganizationContext;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;

// ── DTOs ──────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct InvoiceResult {
    pub id: InvoiceId,
    pub org_id: OrganizationId,
    pub stripe_invoice_id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub hosted_url: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct InvoicesResult {
    pub invoices: Vec<InvoiceResult>,
    pub total: i64,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct InvoicePath {
    pub id: InvoiceId,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct InvoicePaginationQuery {
    pub index: Option<i64>,
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct AdminInvoicePaginationQuery {
    pub index: Option<i64>,
    pub count: Option<i64>,
    pub org_id: Option<OrganizationId>,
}

fn map_invoice(i: DbInvoice) -> InvoiceResult {
    InvoiceResult {
        id: i.id,
        org_id: i.org_id,
        stripe_invoice_id: i.stripe_invoice_id,
        amount: i.amount,
        currency: i.currency,
        status: i.status,
        hosted_url: i.hosted_url,
        created_at: i.created_at,
    }
}

// ── Tenant endpoints (org context required) ───────────────────────────────────

/// GET /invoices
/// Lists the invoices for the current org. Supports `index` (offset) and
/// `count` (page size) query params. Requires `invoices:read`.
#[get("")]
pub async fn list_my_invoices(
    perms: PermissionsContext,
    org_ctx: OrganizationContext,
    query: web::Query<InvoicePaginationQuery>,
    app: web::Data<InvoicesAppData>,
) -> HttpResponse {
    if !perms.has("invoices:read") {
        return HttpResponse::Forbidden().body("missing required permission: invoices:read");
    }

    let offset = query.index.unwrap_or(0).max(0);
    let limit = query.count.unwrap_or(20).clamp(1, 100);
    let org_id: OrganizationId = org_ctx.id;
    info!(org_id, offset, limit, "list my invoices request received");

    match app
        .invoices_provider
        .list_by_org_paginated(org_id, offset, limit)
        .await
    {
        Ok((invoices, total)) => HttpResponse::Ok().json(InvoicesResult {
            invoices: invoices.into_iter().map(map_invoice).collect(),
            total,
        }),
        Err(e) => {
            error!(org_id, error = %e, "list my invoices failed");
            HttpResponse::InternalServerError().body("failed to list invoices")
        }
    }
}

/// GET /invoices/{id}
/// Returns a single invoice, verifying it belongs to the current org.
/// Requires `invoices:read`.
#[get("/{id}")]
pub async fn get_my_invoice(
    perms: PermissionsContext,
    org_ctx: OrganizationContext,
    path: web::Path<InvoicePath>,
    app: web::Data<InvoicesAppData>,
) -> HttpResponse {
    if !perms.has("invoices:read") {
        return HttpResponse::Forbidden().body("missing required permission: invoices:read");
    }

    let id = path.into_inner().id;
    let org_id: OrganizationId = org_ctx.id;
    info!(id, org_id, "get my invoice request received");

    match app.invoices_provider.get_invoice(id).await {
        Ok(Some(invoice)) if invoice.org_id == org_id => {
            HttpResponse::Ok().json(map_invoice(invoice))
        }
        Ok(Some(_)) => HttpResponse::Forbidden().body("invoice does not belong to this org"),
        Ok(None) => HttpResponse::NotFound().body("invoice not found"),
        Err(e) => {
            error!(id, error = %e, "get my invoice failed");
            HttpResponse::InternalServerError().body("failed to get invoice")
        }
    }
}

// ── Admin endpoints (no org context required) ─────────────────────────────────

/// GET /invoices/admin
/// Admin list of all invoices. Supports optional `org_id` filter and
/// `index` / `count` pagination. Requires `invoices:read`.
#[get("/admin")]
pub async fn list_all_invoices(
    perms: PermissionsContext,
    query: web::Query<AdminInvoicePaginationQuery>,
    app: web::Data<InvoicesAppData>,
) -> HttpResponse {
    if !perms.has("invoices:read") {
        return HttpResponse::Forbidden().body("missing required permission: invoices:read");
    }

    let offset = query.index.unwrap_or(0).max(0);
    let limit = query.count.unwrap_or(20).clamp(1, 100);
    let org_id = query.org_id;
    info!(
        offset,
        limit, org_id, "admin list invoices request received"
    );

    match app
        .invoices_provider
        .list_all_paginated(org_id, offset, limit)
        .await
    {
        Ok((invoices, total)) => HttpResponse::Ok().json(InvoicesResult {
            invoices: invoices.into_iter().map(map_invoice).collect(),
            total,
        }),
        Err(e) => {
            error!(error = %e, "admin list invoices failed");
            HttpResponse::InternalServerError().body("failed to list invoices")
        }
    }
}

/// DELETE /invoices/{id}
/// Admin-only hard delete of an invoice record.
/// Requires `invoices:delete`.
#[delete("/{id}")]
pub async fn delete_invoice(
    perms: PermissionsContext,
    path: web::Path<InvoicePath>,
    app: web::Data<InvoicesAppData>,
) -> HttpResponse {
    if !perms.has("invoices:delete") {
        return HttpResponse::Forbidden().body("missing required permission: invoices:delete");
    }

    let id = path.into_inner().id;
    info!(id, "delete invoice request received");

    match app.invoices_provider.delete_invoice(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("invoice not found"),
        Err(e) => {
            error!(id, error = %e, "delete invoice failed");
            HttpResponse::InternalServerError().body("failed to delete invoice")
        }
    }
}

// ── Routing ───────────────────────────────────────────────────────────────────

pub fn init(cfg: &mut web::ServiceConfig) {
    // /admin must be registered before /{id} so actix-web doesn't interpret
    // the literal "admin" segment as an invoice id.
    cfg.service(list_all_invoices)
        .service(list_my_invoices)
        .service(get_my_invoice)
        .service(delete_invoice);
}

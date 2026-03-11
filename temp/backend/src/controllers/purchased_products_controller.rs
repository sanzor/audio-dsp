use actix_web::{delete, get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_product::ProductId;
use crate::domain::db::db_purchased_product::PurchasedProductId;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;
use crate::purchased_products::purchased_products_app_data::PurchasedProductsAppData;
use crate::purchased_products::DbPurchasedProduct;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct PurchasedProductResult {
    pub id: PurchasedProductId,
    pub org_id: OrganizationId,
    pub product_id: ProductId,
    pub tokens_granted: i64,
    pub stripe_payment_intent_id: Option<String>,
    pub purchased_at: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct PurchasedProductsResult {
    pub purchased_products: Vec<PurchasedProductResult>,
    pub total: i64,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct PaginationQuery {
    pub index: Option<i64>,
    pub count: Option<i64>,
    pub org_id: Option<OrganizationId>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct PurchasedProductPath {
    pub id: PurchasedProductId,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct OrgIdPath {
    pub org_id: OrganizationId,
}

fn map_purchased_product(p: DbPurchasedProduct) -> PurchasedProductResult {
    PurchasedProductResult {
        id: p.id,
        org_id: p.org_id,
        product_id: p.product_id,
        tokens_granted: p.tokens_granted,
        stripe_payment_intent_id: p.stripe_payment_intent_id,
        purchased_at: p.purchased_at,
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GetPurchasedProductResponse {
    pub purchased_product: PurchasedProductResult,
}

#[utoipa::path(
    get,
    path = "/purchased-products/{id}",
    tag = "PurchasedProducts",
    params(("id" = i64, Path, description = "Purchased product id")),
    responses(
        (status = 200, description = "Purchase retrieved", body = GetPurchasedProductResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/{id}")]
pub async fn get_purchased_product(
    perms: PermissionsContext,
    path: web::Path<PurchasedProductPath>,
    app_state: web::Data<PurchasedProductsAppData>,
) -> HttpResponse {
    if !perms.has("purchased_products:read") {
        return HttpResponse::Forbidden()
            .body("missing required permission: purchased_products:read");
    }
    let id = path.into_inner().id;
    info!(id, "get purchased product request received");

    match app_state
        .purchased_products_provider
        .get_purchased_product(id)
        .await
    {
        Ok(Some(record)) => HttpResponse::Ok().json(GetPurchasedProductResponse {
            purchased_product: map_purchased_product(record),
        }),
        Ok(None) => HttpResponse::NotFound().body("purchased product not found"),
        Err(e) => {
            error!(id, error = %e, "get purchased product failed");
            HttpResponse::InternalServerError().body("failed to get purchased product")
        }
    }
}

#[utoipa::path(
    get,
    path = "/purchased-products",
    tag = "PurchasedProducts",
    params(
        ("index" = Option<i64>, Query, description = "Offset (number of records to skip)"),
        ("count" = Option<i64>, Query, description = "Page size (max records to return)"),
        ("org_id" = Option<i64>, Query, description = "Filter by organization id"),
    ),
    responses(
        (status = 200, description = "Purchases retrieved", body = PurchasedProductsResult),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
pub async fn list_all_purchased_products(
    perms: PermissionsContext,
    query: web::Query<PaginationQuery>,
    app_state: web::Data<PurchasedProductsAppData>,
) -> HttpResponse {
    if !perms.has("purchased_products:read") {
        return HttpResponse::Forbidden()
            .body("missing required permission: purchased_products:read");
    }
    let offset = query.index.unwrap_or(0).max(0);
    let limit = query.count.unwrap_or(20).clamp(1, 100);
    let org_id = query.org_id;
    info!(
        offset,
        limit, org_id, "list purchased products request received"
    );

    match app_state
        .purchased_products_provider
        .list_paginated(org_id, offset, limit)
        .await
    {
        Ok((records, total)) => HttpResponse::Ok().json(PurchasedProductsResult {
            purchased_products: records.into_iter().map(map_purchased_product).collect(),
            total,
        }),
        Err(e) => {
            error!(error = %e, "list purchased products failed");
            HttpResponse::InternalServerError().body("failed to list purchased products")
        }
    }
}

#[utoipa::path(
    get,
    path = "/purchased-products/by-org/{org_id}",
    tag = "PurchasedProducts",
    params(("org_id" = i64, Path, description = "Organization id")),
    responses(
        (status = 200, description = "Org purchases retrieved", body = PurchasedProductsResult),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/by-org/{org_id}")]
pub async fn list_purchased_products_by_org(
    perms: PermissionsContext,
    path: web::Path<OrgIdPath>,
    app_state: web::Data<PurchasedProductsAppData>,
) -> HttpResponse {
    if !perms.has("purchased_products:read") {
        return HttpResponse::Forbidden()
            .body("missing required permission: purchased_products:read");
    }
    let org_id = path.into_inner().org_id;
    info!(org_id, "list purchased products by org request received");

    match app_state
        .purchased_products_provider
        .list_by_org(org_id)
        .await
    {
        Ok(records) => {
            let total = records.len() as i64;
            HttpResponse::Ok().json(PurchasedProductsResult {
                purchased_products: records.into_iter().map(map_purchased_product).collect(),
                total,
            })
        }
        Err(e) => {
            error!(org_id, error = %e, "list purchased products by org failed");
            HttpResponse::InternalServerError().body("failed to list purchased products")
        }
    }
}


#[derive(Serialize, ToSchema)]
pub struct ReceiptResponse {
    pub receipt_url: String,
}

#[utoipa::path(
    delete,
    path = "/purchased-products/{id}",
    tag = "PurchasedProducts",
    params(("id" = i64, Path, description = "Purchased product id")),
    responses(
        (status = 204, description = "Purchase deleted"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal server error"),
    )
)]


/// GET /purchased-products/{id}/receipt
/// Fetches the Stripe-hosted receipt URL for a one-time token pack purchase.
#[get("/{id}/receipt")]
pub async fn get_purchase_receipt(
    perms: PermissionsContext,
    path: web::Path<PurchasedProductPath>,
    app_state: web::Data<PurchasedProductsAppData>,
) -> HttpResponse {
    if !perms.has("purchased_products:read") {
        return HttpResponse::Forbidden()
            .body("missing required permission: purchased_products:read");
    }
    let id = path.into_inner().id;
    info!(id, "get purchase receipt request received");

    let record = match app_state.purchased_products_provider.get_purchased_product(id).await {
        Ok(Some(r)) => r,
        Ok(None) => return HttpResponse::NotFound().body("purchased product not found"),
        Err(e) => {
            error!(id, error = %e, "get purchase receipt: failed to fetch purchase");
            return HttpResponse::InternalServerError().body("failed to fetch purchase");
        }
    };

    let payment_intent_id = match record.stripe_payment_intent_id {
        Some(ref pi) => pi.clone(),
        None => return HttpResponse::UnprocessableEntity().body("purchase has no payment intent"),
    };

    match app_state.stripe.get_receipt_url(&payment_intent_id).await {
        Ok(Some(url)) => HttpResponse::Ok().json(ReceiptResponse { receipt_url: url }),
        Ok(None) => HttpResponse::NotFound().body("receipt not yet available for this payment"),
        Err(e) => {
            error!(id, error = %e, "get purchase receipt: stripe error");
            HttpResponse::InternalServerError().body("failed to fetch receipt from Stripe")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_purchased_products_by_org)
        .service(get_purchase_receipt)
        .service(get_purchased_product)
        .service(list_all_purchased_products);
}

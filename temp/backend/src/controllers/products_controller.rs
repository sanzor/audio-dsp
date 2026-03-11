use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

use crate::checkout::checkout_app_data::CheckoutAppData;
use crate::checkout::checkout_error::CheckoutError;
use crate::checkout::create_token_pack_checkout_params::CreateTokenPackCheckoutParams;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_product::ProductId;
use crate::domain::service_error::ServiceError;
use crate::middlewares::organization::organization_context::OrganizationContext;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;
use crate::products::create_product_params::CreateProductParams;
use crate::products::products_app_data::ProductsAppData;
use crate::products::update_product_params::UpdateProductParams;
use crate::products::DbProduct;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ProductResult {
    pub id: ProductId,
    pub name: String,
    pub description: Option<String>,
    pub token_amount: i64,
    pub price_cents: i64,
    pub currency: String,
    pub stripe_price_id: Option<String>,
    pub is_active: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ProductsResult {
    pub products: Vec<ProductResult>,
    pub total: i64,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ProductListQuery {
    pub index: Option<i64>,
    pub count: Option<i64>,
    pub active_only: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct ProductPath {
    pub id: ProductId,
}

fn map_product(p: DbProduct) -> ProductResult {
    ProductResult {
        id: p.id,
        name: p.name,
        description: p.description,
        token_amount: p.token_amount,
        price_cents: p.price_cents,
        currency: p.currency,
        stripe_price_id: p.stripe_price_id,
        is_active: p.is_active,
        created_at: p.created_at,
        updated_at: p.updated_at,
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct CreateProductInput {
    pub name: String,
    pub description: Option<String>,
    pub token_amount: i64,
    pub price_cents: i64,
    pub currency: Option<String>,
    pub stripe_price_id: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct CreateProductResponse {
    pub product: ProductResult,
}

#[utoipa::path(
    post,
    path = "/products",
    tag = "Products",
    request_body = CreateProductInput,
    responses(
        (status = 201, description = "Product created", body = CreateProductResponse),
        (status = 400, description = "Invalid input"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("")]
pub async fn create_product(
    perms: PermissionsContext,
    payload: web::Json<CreateProductInput>,
    app_state: web::Data<ProductsAppData>,
) -> HttpResponse {
    if !perms.has("products:create") {
        return HttpResponse::Forbidden().body("missing required permission: products:create");
    }
    info!("create product request received");
    let input = payload.into_inner();

    if input.name.trim().is_empty() {
        warn!("create product rejected: missing name");
        return HttpResponse::BadRequest().body("name is required");
    }
    if input.token_amount <= 0 {
        warn!("create product rejected: invalid token_amount");
        return HttpResponse::BadRequest().body("token_amount must be greater than zero");
    }
    if input.price_cents < 0 {
        warn!("create product rejected: invalid price_cents");
        return HttpResponse::BadRequest().body("price_cents must be non-negative");
    }

    let params = CreateProductParams {
        name: input.name,
        description: input.description,
        token_amount: input.token_amount,
        price_cents: input.price_cents,
        currency: input.currency.unwrap_or_else(|| "usd".to_string()),
        stripe_price_id: input.stripe_price_id,
    };

    match app_state.products_provider.create_product(params).await {
        Ok(product) => HttpResponse::Created().json(CreateProductResponse {
            product: map_product(product),
        }),
        Err(e) => {
            error!(error = %e, "create product failed");
            HttpResponse::InternalServerError().body("failed to create product")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateProductInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub token_amount: Option<i64>,
    pub price_cents: Option<i64>,
    pub stripe_price_id: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateProductResponse {
    pub product: ProductResult,
}

#[utoipa::path(
    put,
    path = "/products/{id}",
    tag = "Products",
    request_body = UpdateProductInput,
    params(("id" = i64, Path, description = "Product id")),
    responses(
        (status = 200, description = "Product updated", body = UpdateProductResponse),
        (status = 400, description = "No fields provided"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Product not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[put("/{id}")]
pub async fn update_product(
    perms: PermissionsContext,
    path: web::Path<ProductPath>,
    payload: web::Json<UpdateProductInput>,
    app_state: web::Data<ProductsAppData>,
) -> HttpResponse {
    if !perms.has("products:update") {
        return HttpResponse::Forbidden().body("missing required permission: products:update");
    }
    let id = path.into_inner().id;
    info!(id, "update product request received");
    let input = payload.into_inner();

    if input.name.is_none()
        && input.description.is_none()
        && input.token_amount.is_none()
        && input.price_cents.is_none()
        && input.stripe_price_id.is_none()
        && input.is_active.is_none()
    {
        warn!(id, "update product rejected: no fields provided");
        return HttpResponse::BadRequest().body("at least one field is required");
    }

    let params = UpdateProductParams {
        name: input.name,
        description: input.description,
        token_amount: input.token_amount,
        price_cents: input.price_cents,
        stripe_price_id: input.stripe_price_id,
        is_active: input.is_active,
    };

    match app_state.products_provider.update_product(id, params).await {
        Ok(Some(product)) => HttpResponse::Ok().json(UpdateProductResponse {
            product: map_product(product),
        }),
        Ok(None) => HttpResponse::NotFound().body("product not found"),
        Err(e) => {
            error!(id, error = %e, "update product failed");
            HttpResponse::InternalServerError().body("failed to update product")
        }
    }
}

#[post("/activate/{id}")]
pub async fn activate_product(
    perms: PermissionsContext,
    path: web::Path<ProductPath>,
    app_state: web::Data<ProductsAppData>,
) -> HttpResponse {
    if !perms.has("products:update") {
        return HttpResponse::Forbidden().body("missing required permission: products:update");
    }
    let id = path.into_inner().id;
    info!(id, "activate product request received");
    match app_state.products_provider.activate_product(id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(ServiceError::NotFound) => HttpResponse::NotFound().body("could not find product"),
        Err(ServiceError::Validation(msg)) => HttpResponse::UnprocessableEntity().body(msg),
        Err(e) => {
            error!(id, error = %e, "activate product failed");
            HttpResponse::InternalServerError().body("failed to update product")
        }
    }
}

#[post("/deactivate/{id}")]
pub async fn deactivate_product(
    perms: PermissionsContext,
    path: web::Path<ProductPath>,
    app_state: web::Data<ProductsAppData>,
) -> HttpResponse {
    if !perms.has("products:update") {
        return HttpResponse::Forbidden().body("missing required permission: products:update");
    }
    let id = path.into_inner().id;
    info!(id, "deactivate product request received");

    match app_state.products_provider.deactivate_product(id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(ServiceError::NotFound) => HttpResponse::NotFound().body("could not find product"),

        Err(e) => {
            error!(id, error = %e, "activate product failed");
            HttpResponse::InternalServerError().body("failed to update product")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/products/{id}",
    tag = "Products",
    params(("id" = i64, Path, description = "Product id")),
    responses(
        (status = 204, description = "Product deleted"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Product not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[delete("/{id}")]
pub async fn delete_product(
    perms: PermissionsContext,
    path: web::Path<ProductPath>,
    app_state: web::Data<ProductsAppData>,
) -> HttpResponse {
    if !perms.has("products:delete") {
        return HttpResponse::Forbidden().body("missing required permission: products:delete");
    }
    let id = path.into_inner().id;
    info!(id, "delete product request received");

    match app_state.products_provider.delete_product(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("product not found"),
        Err(e) => {
            error!(id, error = %e, "delete product failed");
            HttpResponse::InternalServerError().body("failed to delete product")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GetProductResponse {
    pub product: ProductResult,
}

#[utoipa::path(
    get,
    path = "/products/{id}",
    tag = "Products",
    params(("id" = i64, Path, description = "Product id")),
    responses(
        (status = 200, description = "Product retrieved", body = GetProductResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Product not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/{id}")]
pub async fn get_product(
    perms: PermissionsContext,
    path: web::Path<ProductPath>,
    app_state: web::Data<ProductsAppData>,
) -> HttpResponse {
    if !perms.has("products:read") {
        return HttpResponse::Forbidden().body("missing required permission: products:read");
    }
    let id = path.into_inner().id;
    info!(id, "get product request received");

    match app_state.products_provider.get_product(id).await {
        Ok(Some(product)) => HttpResponse::Ok().json(GetProductResponse {
            product: map_product(product),
        }),
        Ok(None) => HttpResponse::NotFound().body("product not found"),
        Err(e) => {
            error!(id, error = %e, "get product failed");
            HttpResponse::InternalServerError().body("failed to get product")
        }
    }
}

#[utoipa::path(
    get,
    path = "/products",
    tag = "Products",
    responses(
        (status = 200, description = "Products retrieved", body = ProductsResult),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
pub async fn list_products(
    perms: PermissionsContext,
    query: web::Query<ProductListQuery>,
    app_state: web::Data<ProductsAppData>,
) -> HttpResponse {
    if !perms.has("products:read") {
        return HttpResponse::Forbidden().body("missing required permission: products:read");
    }
    let offset = query.index.unwrap_or(0).max(0) as usize;
    let limit = query.count.unwrap_or(100).clamp(1, 200) as usize;
    let active_only = query.active_only.unwrap_or(false);
    info!(offset, limit, active_only, "list products request received");

    match app_state.products_provider.list_products().await {
        Ok(products) => {
            let filtered: Vec<_> = products
                .into_iter()
                .filter(|p| !active_only || p.is_active)
                .collect();
            let total = filtered.len() as i64;
            let page: Vec<_> = filtered
                .into_iter()
                .skip(offset)
                .take(limit)
                .map(map_product)
                .collect();
            HttpResponse::Ok().json(ProductsResult {
                products: page,
                total,
            })
        }
        Err(e) => {
            error!(error = %e, "list products failed");
            HttpResponse::InternalServerError().body("failed to list products")
        }
    }
}

// ── Checkout ──────────────────────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
pub struct CheckoutSessionResponse {
    pub checkout_url: String,
}

#[derive(Deserialize, ToSchema)]
pub struct TokenPackCheckoutInput {
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

/// POST /products/{id}/checkout
/// Initiates a Stripe Checkout Session for a one-time token pack purchase.
/// Requires `checkout:create` permission and an org-scoped JWT.
#[post("/{id}/checkout")]
pub async fn token_pack_checkout(
    perms: PermissionsContext,
    org_ctx: OrganizationContext,
    req: HttpRequest,
    path: web::Path<ProductPath>,
    payload: web::Json<TokenPackCheckoutInput>,
    checkout: web::Data<CheckoutAppData>,
) -> HttpResponse {
    if !perms.has("checkout:create") {
        return HttpResponse::Forbidden().finish();
    }

    let org_id = OrganizationId::from(org_ctx.id);
    let product_id = i64::from(path.into_inner().id);

    let origin = req
        .headers()
        .get("Origin")
        .and_then(|v| v.to_str().ok());
    let default_success_path = format!("/org/{}/checkout/success", org_ctx.slug);
    let default_cancel_path = format!("/org/{}/checkout/cancel", org_ctx.slug);

    let success_url =
        match normalize_redirect_target(origin, payload.success_url.clone(), &default_success_path) {
            Ok(v) => v,
            Err(msg) => return HttpResponse::UnprocessableEntity().body(msg),
        };
    let cancel_url =
        match normalize_redirect_target(origin, payload.cancel_url.clone(), &default_cancel_path) {
            Ok(v) => v,
            Err(msg) => return HttpResponse::UnprocessableEntity().body(msg),
        };

    let params = CreateTokenPackCheckoutParams {
        org_id,
        org_slug: org_ctx.slug,
        product_id,
        success_url,
        cancel_url,
    };

    match checkout
        .checkout_provider
        .create_token_pack_checkout(params)
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
    cfg.service(create_product)
        .service(update_product)
        .service(activate_product)
        .service(deactivate_product)
        .service(delete_product)
        .service(get_product)
        .service(list_products)
        .service(token_pack_checkout);
}

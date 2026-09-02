use crate::domain::db::db_product::ProductId;
use crate::domain::tier::Tier;
use crate::middlewares::jwt::jwt_context::JwtContext;
use crate::products::create_product_params::CreateProductParams;
use crate::products::products_app_data::ProductsAppData;
use crate::products::update_product_params::UpdateProductParams;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, ToSchema)]
pub struct CreateProductInput {
    pub name: String,
    pub description: Option<String>,
    pub tier: Tier,
    pub price_cents: i64,
    pub currency: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateProductInput {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub tier: Option<Tier>,
    pub price_cents: Option<i64>,
    pub currency: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize, IntoParams)]
pub struct ProductIdPath {
    pub id: ProductId,
}

#[derive(Deserialize, IntoParams)]
pub struct ActiveOnlyQuery {
    pub active_only: Option<bool>,
}

#[utoipa::path(get, path = "/v1/products", tag = "Products",
    params(ActiveOnlyQuery),
    responses((status = 200, body = Vec<crate::domain::db::db_product::DbProduct>)))]
#[get("")]
pub async fn list_products(
    query: web::Query<ActiveOnlyQuery>,
    app: web::Data<ProductsAppData>,
) -> HttpResponse {
    let active_only = query.active_only.unwrap_or(true);
    match app.products_provider.list_products(active_only).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(get, path = "/v1/products/{id}", tag = "Products",
    params(ProductIdPath),
    responses((status = 200, body = crate::domain::db::db_product::DbProduct), (status = 404)))]
#[get("/{id}")]
pub async fn get_product(
    path: web::Path<ProductIdPath>,
    app: web::Data<ProductsAppData>,
) -> HttpResponse {
    let id = path.into_inner().id;
    match app.products_provider.get_product(id).await {
        Ok(Some(p)) => HttpResponse::Ok().json(p),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(post, path = "/v1/products", tag = "Products",
    request_body = CreateProductInput,
    responses((status = 201, body = crate::domain::db::db_product::DbProduct), (status = 401), (status = 403)))]
#[post("")]
pub async fn create_product(
    auth: JwtContext,
    payload: web::Json<CreateProductInput>,
    app: web::Data<ProductsAppData>,
) -> HttpResponse {
    if !auth.is_admin {
        return HttpResponse::Forbidden().finish();
    }
    let input = payload.into_inner();
    let params = CreateProductParams {
        name: input.name,
        description: input.description,
        tier: input.tier,
        price_cents: input.price_cents,
        currency: input.currency,
    };
    match app.products_provider.create_product(params).await {
        Ok(p) => HttpResponse::Created().json(p),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(put, path = "/v1/products/{id}", tag = "Products",
    params(ProductIdPath),
    request_body = UpdateProductInput,
    responses((status = 200, body = crate::domain::db::db_product::DbProduct), (status = 401), (status = 403), (status = 404)))]
#[put("/{id}")]
pub async fn update_product(
    auth: JwtContext,
    path: web::Path<ProductIdPath>,
    payload: web::Json<UpdateProductInput>,
    app: web::Data<ProductsAppData>,
) -> HttpResponse {
    if !auth.is_admin {
        return HttpResponse::Forbidden().finish();
    }
    let id = path.into_inner().id;
    let input = payload.into_inner();
    let params = UpdateProductParams {
        name: input.name,
        description: input.description,
        tier: input.tier,
        price_cents: input.price_cents,
        currency: input.currency,
        is_active: input.is_active,
    };
    match app.products_provider.update_product(id, params).await {
        Ok(Some(p)) => HttpResponse::Ok().json(p),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(delete, path = "/v1/products/{id}", tag = "Products",
    params(ProductIdPath),
    responses((status = 204), (status = 401), (status = 403), (status = 404)))]
#[delete("/{id}")]
pub async fn delete_product(
    auth: JwtContext,
    path: web::Path<ProductIdPath>,
    app: web::Data<ProductsAppData>,
) -> HttpResponse {
    if !auth.is_admin {
        return HttpResponse::Forbidden().finish();
    }
    let id = path.into_inner().id;
    match app.products_provider.delete_product(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_products)
        .service(get_product)
        .service(create_product)
        .service(update_product)
        .service(delete_product);
}

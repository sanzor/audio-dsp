use crate::domain::db::db_purchased_product::PurchasedProductId;
use crate::middlewares::jwt::jwt_context::JwtContext;
use crate::purchased_products::create_purchased_product_params::CreatePurchasedProductParams;
use crate::purchased_products::purchased_products_app_data::PurchasedProductsAppData;
use actix_web::{delete, get, post, web, HttpResponse};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, ToSchema)]
pub struct CreatePurchasedProductInput {
    pub product_id: i32,
}

#[derive(Deserialize, IntoParams)]
pub struct PurchasedProductIdPath {
    pub id: PurchasedProductId,
}

#[utoipa::path(post, path = "/v1/purchased-products", tag = "PurchasedProducts",
    request_body = CreatePurchasedProductInput,
    responses((status = 201, body = crate::domain::db::db_purchased_product::DbPurchasedProduct), (status = 401)))]
#[post("")]
pub async fn create_purchased_product(
    auth: JwtContext,
    payload: web::Json<CreatePurchasedProductInput>,
    app: web::Data<PurchasedProductsAppData>,
) -> HttpResponse {
    let input = payload.into_inner();
    let params = CreatePurchasedProductParams {
        user_id: auth.user_id,
        product_id: input.product_id,
    };
    match app
        .purchased_products_provider
        .create_purchased_product(params)
        .await
    {
        Ok(pp) => HttpResponse::Created().json(pp),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(get, path = "/v1/purchased-products/mine", tag = "PurchasedProducts",
    responses((status = 200, body = Vec<crate::domain::db::db_purchased_product::DbPurchasedProduct>), (status = 401)))]
#[get("/mine")]
pub async fn list_my_purchased_products(
    auth: JwtContext,
    app: web::Data<PurchasedProductsAppData>,
) -> HttpResponse {
    match app
        .purchased_products_provider
        .list_by_user(auth.user_id)
        .await
    {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(delete, path = "/v1/purchased-products/{id}", tag = "PurchasedProducts",
    params(PurchasedProductIdPath),
    responses((status = 204), (status = 401), (status = 403), (status = 404)))]
#[delete("/{id}")]
pub async fn delete_purchased_product(
    auth: JwtContext,
    path: web::Path<PurchasedProductIdPath>,
    app: web::Data<PurchasedProductsAppData>,
) -> HttpResponse {
    if !auth.is_admin {
        return HttpResponse::Forbidden().finish();
    }
    let id = path.into_inner().id;
    match app
        .purchased_products_provider
        .delete_purchased_product(id)
        .await
    {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_purchased_product)
        .service(list_my_purchased_products)
        .service(delete_purchased_product);
}

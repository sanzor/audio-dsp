use crate::domain::db::db_invoice::InvoiceId;
use crate::invoices::create_invoice_params::CreateInvoiceParams;
use crate::invoices::invoices_app_data::InvoicesAppData;
use crate::middlewares::jwt::jwt_context::JwtContext;
use actix_web::{delete, get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, ToSchema)]
pub struct CreateInvoiceInput {
    pub stripe_invoice_id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub hosted_url: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct InvoiceListResponse {
    pub invoices: Vec<crate::domain::db::db_invoice::DbInvoice>,
    pub total: i64,
}

#[derive(Deserialize, IntoParams)]
pub struct PaginationQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[utoipa::path(post, path = "/v1/invoices", tag = "Invoices",
    request_body = CreateInvoiceInput,
    responses((status = 201, body = crate::domain::db::db_invoice::DbInvoice), (status = 401)))]
#[post("")]
pub async fn create_invoice(
    auth: JwtContext,
    payload: web::Json<CreateInvoiceInput>,
    app: web::Data<InvoicesAppData>,
) -> HttpResponse {
    let input = payload.into_inner();
    let params = CreateInvoiceParams {
        user_id: auth.user_id,
        stripe_invoice_id: input.stripe_invoice_id,
        amount: input.amount,
        currency: input.currency,
        status: input.status,
        hosted_url: input.hosted_url,
    };
    match app.invoices_provider.create_invoice(params).await {
        Ok(inv) => HttpResponse::Created().json(inv),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize, IntoParams)]
pub struct InvoiceIdPath {
    pub id: InvoiceId,
}

#[utoipa::path(get, path = "/v1/invoices/mine", tag = "Invoices",
    params(PaginationQuery),
    responses((status = 200, body = InvoiceListResponse), (status = 401)))]
#[get("/mine")]
pub async fn list_my_invoices(
    auth: JwtContext,
    query: web::Query<PaginationQuery>,
    app: web::Data<InvoicesAppData>,
) -> HttpResponse {
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    match app
        .invoices_provider
        .list_by_user_paginated(auth.user_id, offset, limit)
        .await
    {
        Ok((invoices, total)) => HttpResponse::Ok().json(InvoiceListResponse { invoices, total }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(get, path = "/v1/invoices/{id}", tag = "Invoices",
    params(InvoiceIdPath),
    responses((status = 200, body = crate::domain::db::db_invoice::DbInvoice), (status = 401), (status = 403), (status = 404)))]
#[get("/{id}")]
pub async fn get_invoice(
    auth: JwtContext,
    path: web::Path<InvoiceIdPath>,
    app: web::Data<InvoicesAppData>,
) -> HttpResponse {
    let id = path.into_inner().id;
    match app.invoices_provider.get_invoice(id).await {
        Ok(Some(inv)) if inv.user_id == auth.user_id || auth.is_admin => {
            HttpResponse::Ok().json(inv)
        }
        Ok(Some(_)) => HttpResponse::Forbidden().finish(),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(get, path = "/v1/invoices", tag = "Invoices",
    params(PaginationQuery),
    responses((status = 200, body = InvoiceListResponse), (status = 401), (status = 403)))]
#[get("")]
pub async fn list_all_invoices(
    auth: JwtContext,
    query: web::Query<PaginationQuery>,
    app: web::Data<InvoicesAppData>,
) -> HttpResponse {
    if !auth.is_admin {
        return HttpResponse::Forbidden().finish();
    }
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    match app
        .invoices_provider
        .list_all_paginated(None, offset, limit)
        .await
    {
        Ok((invoices, total)) => HttpResponse::Ok().json(InvoiceListResponse { invoices, total }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(delete, path = "/v1/invoices/{id}", tag = "Invoices",
    params(InvoiceIdPath),
    responses((status = 204), (status = 401), (status = 403), (status = 404)))]
#[delete("/{id}")]
pub async fn delete_invoice(
    auth: JwtContext,
    path: web::Path<InvoiceIdPath>,
    app: web::Data<InvoicesAppData>,
) -> HttpResponse {
    if !auth.is_admin {
        return HttpResponse::Forbidden().finish();
    }
    let id = path.into_inner().id;
    match app.invoices_provider.delete_invoice(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_invoice)
        .service(list_my_invoices)
        .service(get_invoice)
        .service(list_all_invoices)
        .service(delete_invoice);
}

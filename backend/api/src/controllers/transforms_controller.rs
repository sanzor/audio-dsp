use crate::{
    middlewares::role_context::role_context::RoleContext,
    transforms::transforms_app_data::TransformsAppData,
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use domain::db::db_transform::TransformId;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// ─── Request / Response types ────────────────────────────────────────────────

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateTransformParams {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UpdateTransformParams {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AddPortParams {
    pub name: String,
    pub direction: String,
    pub port_order: i32,
    pub description: Option<String>,
}

#[derive(Deserialize, IntoParams)]
pub struct PaginationQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct TransformListResponse {
    pub transforms: Vec<domain::transforms::transform::Transform>,
    pub total: i64,
}

#[derive(Deserialize)]
pub struct TransformIdQuery {
    pub transform_id: TransformId,
}

#[derive(Deserialize)]
pub struct PortIdQuery {
    pub port_id: i64,
}

// ─── Handlers ────────────────────────────────────────────────────────────────

#[utoipa::path(get, path = "/transforms/get-all", tag = "transforms",
    params(PaginationQuery),
    responses((status = 200, description = "Paginated transforms", body = serde_json::Value)))]
#[get("/get-all")]
pub async fn get_all_transforms(
    _role: RoleContext,
    query: web::Query<PaginationQuery>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    match app.transforms_service.get_transforms_paginated(offset, limit).await {
        Ok((transforms, total)) => HttpResponse::Ok().json(TransformListResponse { transforms, total }),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[utoipa::path(get, path = "/transforms/get-by-id", tag = "transforms",
    params(("transform_id" = i64, Query, description = "Transform ID")),
    responses((status = 200, description = "Transform", body = serde_json::Value)))]
#[get("/get-by-id")]
pub async fn get_transform_by_id(
    _role: RoleContext,
    query: web::Query<TransformIdQuery>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    match app.transforms_service.get_transform(query.transform_id).await {
        Ok(t) => HttpResponse::Ok().json(t),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

#[utoipa::path(post, path = "/transforms/create", tag = "transforms",
    request_body = CreateTransformParams,
    responses((status = 200, description = "Created transform", body = serde_json::Value)))]
#[post("/create")]
pub async fn create_transform(
    role: RoleContext,
    body: web::Json<CreateTransformParams>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let p = body.into_inner();
    match app.transforms_service.create_transform(p.name, p.description, p.icon).await {
        Ok(t) => HttpResponse::Ok().json(t),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[utoipa::path(put, path = "/transforms/update", tag = "transforms",
    params(("transform_id" = i64, Query, description = "Transform ID")),
    request_body = UpdateTransformParams,
    responses((status = 200, description = "Updated transform", body = serde_json::Value)))]
#[put("/update")]
pub async fn update_transform(
    role: RoleContext,
    query: web::Query<TransformIdQuery>,
    body: web::Json<UpdateTransformParams>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let p = body.into_inner();
    match app.transforms_service.update_transform(query.transform_id, p.name, p.description, p.icon).await {
        Ok(t) => HttpResponse::Ok().json(t),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[utoipa::path(delete, path = "/transforms/delete", tag = "transforms",
    params(("transform_id" = i64, Query, description = "Transform ID")),
    responses((status = 200, description = "Deleted")))]
#[delete("/delete")]
pub async fn delete_transform(
    role: RoleContext,
    query: web::Query<TransformIdQuery>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    match app.transforms_service.delete_transform(query.transform_id).await {
        Ok(_) => HttpResponse::Ok().body("Deleted"),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[utoipa::path(post, path = "/transforms/add-port", tag = "transforms",
    params(("transform_id" = i64, Query, description = "Transform ID")),
    request_body = AddPortParams,
    responses((status = 200, description = "Added port", body = serde_json::Value)))]
#[post("/add-port")]
pub async fn add_port(
    role: RoleContext,
    query: web::Query<TransformIdQuery>,
    body: web::Json<AddPortParams>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let p = body.into_inner();
    match app.transforms_service.add_port(query.transform_id, p.name, p.direction, p.port_order, p.description).await {
        Ok(port) => HttpResponse::Ok().json(port),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[utoipa::path(delete, path = "/transforms/delete-port", tag = "transforms",
    params(("port_id" = i64, Query, description = "Port ID")),
    responses((status = 200, description = "Port deleted")))]
#[delete("/delete-port")]
pub async fn delete_port(
    role: RoleContext,
    query: web::Query<PortIdQuery>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    match app.transforms_service.delete_port(query.port_id).await {
        Ok(_) => HttpResponse::Ok().body("Deleted"),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

// ─── Route registration ───────────────────────────────────────────────────────

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_transforms)
        .service(get_transform_by_id)
        .service(create_transform)
        .service(update_transform)
        .service(delete_transform)
        .service(add_port)
        .service(delete_port);
}

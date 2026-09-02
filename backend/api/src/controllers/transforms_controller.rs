use crate::{
    domain::service_error::ServiceError,
    middlewares::{
        authz::{
            transform_access_context::TransformAccessContext,
            transform_authz::{require_access, require_owner},
        },
        jwt::jwt_context::JwtContext,
    },
    transforms::{
        dto::{
            requests::{PaginationQuery, TransformIdPath, TransformIdsRequest},
            responses::{
                TransformBinariesResponse, TransformBinaryDto, TransformDto, TransformSummaryDto,
                TransformSummaryListResponse, TransformsResponse,
            },
        },
        transforms_app_data::TransformsAppData,
    },
};
use actix_web::{delete, get, post, web, HttpResponse};
use tracing::error;

// ─── /transforms handlers (published, bucket 3) ───────────────────────────────

fn map_service_error(err: ServiceError) -> HttpResponse {
    match err {
        ServiceError::NotFound => HttpResponse::NotFound().body("not found"),
        ServiceError::Conflict(msg) => HttpResponse::Conflict().body(msg),
        ServiceError::Validation(msg) => HttpResponse::BadRequest().body(msg),
        ServiceError::Internal(msg) => {
            error!(error = %msg, "internal error");
            HttpResponse::InternalServerError().body("internal server error")
        }
    }
}

#[utoipa::path(get, path = "/transforms", tag = "transforms",
    params(PaginationQuery),
    responses((status = 200, description = "Paginated transform summaries", body = serde_json::Value)))]
#[get("")]
pub async fn list_transform_summaries(
    jwt: JwtContext,
    query: web::Query<PaginationQuery>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    // Unfiltered, cross-owner browsing doesn't fit the ownership model —
    // regular users get filtered results from the workspace-scoped catalog
    // endpoint (GET /v1/workspaces/{workspace_id}/transforms) instead.
    if !jwt.is_admin {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    match app
        .transforms_service
        .list_transform_summaries(offset, limit)
        .await
    {
        Ok((transforms, total)) => HttpResponse::Ok().json(TransformSummaryListResponse {
            transforms: transforms
                .into_iter()
                .map(TransformSummaryDto::from)
                .collect(),
            total,
        }),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(get, path = "/transforms/{transform_id}", tag = "transforms",
    params(TransformIdPath),
    responses((status = 200, description = "Transform", body = serde_json::Value)))]
#[get("/{transform_id}")]
pub async fn get_transform(
    jwt: JwtContext,
    path: web::Path<TransformIdPath>,
    app: web::Data<TransformsAppData>,
    access: TransformAccessContext,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_access(&app, &access, transform_id, &jwt).await {
        return resp;
    }
    match app.transforms_service.get_transform(transform_id).await {
        Ok(t) => HttpResponse::Ok().json(TransformDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/transforms/resolve", tag = "transforms",
    request_body = TransformIdsRequest,
    responses((status = 200, description = "Resolved transform definitions", body = serde_json::Value)))]
#[post("/resolve")]
pub async fn get_transforms(
    jwt: JwtContext,
    body: web::Json<TransformIdsRequest>,
    app: web::Data<TransformsAppData>,
    access: TransformAccessContext,
) -> HttpResponse {
    let request = body.into_inner();
    for id in &request.ids {
        if let Err(resp) = require_access(&app, &access, *id, &jwt).await {
            return resp;
        }
    }
    match app.transforms_service.get_transforms(&request.ids).await {
        Ok(transforms) => HttpResponse::Ok().json(TransformsResponse {
            transforms: transforms.into_iter().map(TransformDto::from).collect(),
        }),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/transforms/binaries", tag = "transforms",
    request_body = TransformIdsRequest,
    responses((status = 200, description = "Resolved transform binaries", body = serde_json::Value)))]
#[post("/binaries")]
pub async fn get_transform_binaries(
    jwt: JwtContext,
    body: web::Json<TransformIdsRequest>,
    app: web::Data<TransformsAppData>,
    access: TransformAccessContext,
) -> HttpResponse {
    let request = body.into_inner();
    for id in &request.ids {
        if let Err(resp) = require_access(&app, &access, *id, &jwt).await {
            return resp;
        }
    }
    match app.transforms_service.get_transforms(&request.ids).await {
        Ok(transforms) => HttpResponse::Ok().json(TransformBinariesResponse {
            binaries: transforms
                .into_iter()
                .map(TransformBinaryDto::from)
                .collect(),
        }),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(delete, path = "/transforms/{transform_id}", tag = "transforms",
    params(TransformIdPath),
    responses((status = 200, description = "Deleted")))]
#[delete("/{transform_id}")]
pub async fn delete_transform(
    jwt: JwtContext,
    path: web::Path<TransformIdPath>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_owner(&app, transform_id, &jwt).await {
        return resp;
    }
    match app.transforms_service.delete_transform(transform_id).await {
        Ok(_) => HttpResponse::Ok().body("Deleted"),
        Err(e) => map_service_error(e),
    }
}

// ─── Route registration ───────────────────────────────────────────────────────

/// Mounted at `/transforms` — published (bucket 3) reads and deletion.
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_transform_summaries)
        .service(get_transform)
        .service(get_transforms)
        .service(get_transform_binaries)
        .service(delete_transform);
}

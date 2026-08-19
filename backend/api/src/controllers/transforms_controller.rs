use crate::{
    domain::service_error::ServiceError,
    middlewares::jwt::jwt_context::JwtContext,
    transform_grants::transform_grants_app_data::TransformGrantsAppData,
    transforms::{authz::{require_access, require_owner}, transforms_app_data::TransformsAppData},
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use domain::{
    db::{
        db_transform::{DbTransform, TransformId},
        db_transform_draft::{DbTransformDraft, TransformDraftId},
        ticket::db_resource::ResourceId,
    },
    domain_user::UserId,
};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::{IntoParams, ToSchema};

// ─── Request / Response types ────────────────────────────────────────────────

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateTransformParams {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    /// "primitive" | "composite".
    pub kind: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SaveTransformParams {
    pub source_code: String,
    /// A resource_id from a successful compile ticket, if the frontend wants
    /// to attach that build's binary/metadata to this save. Omit to save
    /// source only, leaving any previously saved binary untouched.
    pub resource_id: Option<ResourceId>,
}

#[derive(Deserialize, IntoParams)]
pub struct PaginationQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct TransformSummaryListResponse {
    pub transforms: Vec<TransformSummaryDto>,
    pub total: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransformSummaryDto {
    pub transform_id: TransformId,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    /// "primitive" | "composite".
    pub kind: String,
}

/// A published (bucket 3) transform's definition. `wasm_bytecode`/`metadata`
/// aren't included here — fetch those via the dedicated binary endpoints
/// below, which base64-encode `wasm_bytecode` for transport.
#[derive(Debug, Serialize, ToSchema)]
pub struct TransformDto {
    pub transform_id: TransformId,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    /// "primitive" | "composite".
    pub kind: String,
    pub source_code: String,
    pub owner_user_id: UserId,
    /// RFC 3339 / ISO 8601.
    pub created_at: String,
}

/// A transform's in-progress (bucket 2) draft state.
#[derive(Debug, Serialize, ToSchema)]
pub struct TransformDraftDto {
    pub transform_id: TransformDraftId,
    pub source_code: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub kind: String,
    /// Whether a compiled binary is currently attached (via a prior save
    /// with a `resource_id`) — not whether it's still in sync with
    /// `source_code`; a source-only save can leave a stale binary attached.
    pub has_binary: bool,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TransformIdsRequest {
    pub ids: Vec<TransformId>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransformsResponse {
    pub transforms: Vec<TransformDto>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransformBinaryDto {
    pub transform_id: TransformId,
    pub wasm_base64: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransformBinariesResponse {
    pub binaries: Vec<TransformBinaryDto>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DraftPublishableDto {
    /// Whether bucket 2 currently holds a binary in sync with its source —
    /// i.e. whether `publish` would succeed right now. See
    /// `TransformsProvider::validate_composite_draft` — the name is a
    /// holdover from the retired composite-graph model.
    pub publishable: bool,
}

#[derive(Deserialize, IntoParams)]
pub struct TransformIdPath {
    pub transform_id: TransformId,
}

impl From<DbTransform> for TransformSummaryDto {
    fn from(value: DbTransform) -> Self {
        Self {
            transform_id: value.transform_id,
            name: value.name,
            description: value.description,
            icon: value.icon,
            kind: value.kind,
        }
    }
}

impl From<DbTransform> for TransformDto {
    fn from(value: DbTransform) -> Self {
        Self {
            transform_id: value.transform_id,
            name: value.name,
            description: value.description,
            icon: value.icon,
            kind: value.kind,
            source_code: value.source_code,
            owner_user_id: value.owner_user_id,
            created_at: value.created_at.to_rfc3339(),
        }
    }
}

impl From<DbTransform> for TransformBinaryDto {
    fn from(value: DbTransform) -> Self {
        Self {
            transform_id: value.transform_id,
            wasm_base64: BASE64_STANDARD.encode(value.wasm_bytecode),
        }
    }
}

impl From<DbTransformDraft> for TransformDraftDto {
    fn from(value: DbTransformDraft) -> Self {
        Self {
            transform_id: value.transform_id,
            source_code: value.source_code,
            has_binary: value.wasm_bytecode.is_some(),
            name: value.name,
            description: value.description,
            kind: value.kind,
        }
    }
}

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

// ─── Handlers ────────────────────────────────────────────────────────────────

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
    match app.transforms_service.list_transform_summaries(offset, limit).await {
        Ok((transforms, total)) => HttpResponse::Ok().json(TransformSummaryListResponse {
            transforms: transforms.into_iter().map(TransformSummaryDto::from).collect(),
            total,
        }),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(get, path = "/transforms/{transform_id}", tag = "transforms",
    params(TransformIdPath),
    responses((status = 200, description = "Transform definition", body = serde_json::Value)))]
#[get("/{transform_id}")]
pub async fn get_transform_definition(
    jwt: JwtContext,
    path: web::Path<TransformIdPath>,
    app: web::Data<TransformsAppData>,
    grants: web::Data<TransformGrantsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_access(&app, &grants, transform_id, &jwt).await {
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
pub async fn resolve_transform_definitions(
    jwt: JwtContext,
    body: web::Json<TransformIdsRequest>,
    app: web::Data<TransformsAppData>,
    grants: web::Data<TransformGrantsAppData>,
) -> HttpResponse {
    let request = body.into_inner();
    for id in &request.ids {
        if let Err(resp) = require_access(&app, &grants, *id, &jwt).await {
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

#[utoipa::path(get, path = "/transforms/{transform_id}/binary", tag = "transforms",
    params(TransformIdPath),
    responses((status = 200, description = "Transform WASM binary", content_type = "application/wasm"),
              (status = 404, description = "Transform not found")))]
#[get("/{transform_id}/binary")]
pub async fn get_transform_binary(
    jwt: JwtContext,
    path: web::Path<TransformIdPath>,
    app: web::Data<TransformsAppData>,
    grants: web::Data<TransformGrantsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_access(&app, &grants, transform_id, &jwt).await {
        return resp;
    }
    match app.transforms_service.get_transform(transform_id).await {
        Ok(t) => HttpResponse::Ok()
            .content_type("application/wasm")
            .body(t.wasm_bytecode),
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
    grants: web::Data<TransformGrantsAppData>,
) -> HttpResponse {
    let request = body.into_inner();
    for id in &request.ids {
        if let Err(resp) = require_access(&app, &grants, *id, &jwt).await {
            return resp;
        }
    }
    match app.transforms_service.get_transforms(&request.ids).await {
        Ok(transforms) => HttpResponse::Ok().json(TransformBinariesResponse {
            binaries: transforms.into_iter().map(TransformBinaryDto::from).collect(),
        }),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/transforms", tag = "transforms",
    request_body = CreateTransformParams,
    responses((status = 200, description = "Created transform draft", body = serde_json::Value)))]
#[post("")]
pub async fn create_transform(
    jwt: JwtContext,
    body: web::Json<CreateTransformParams>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    let p = body.into_inner();
    if p.kind != "primitive" && p.kind != "composite" {
        return HttpResponse::BadRequest().body("kind must be 'primitive' or 'composite'");
    }
    match app.transforms_service.create_transform_draft(p.name, p.description, p.icon, p.kind, UserId::from(jwt.user_id)).await {
        Ok(t) => HttpResponse::Ok().json(TransformDraftDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(put, path = "/transforms/{transform_id}/save", tag = "transforms",
    params(TransformIdPath),
    request_body = SaveTransformParams,
    responses((status = 200, description = "Saved transform draft state", body = serde_json::Value)))]
#[put("/{transform_id}/save")]
pub async fn save_transform(
    jwt: JwtContext,
    path: web::Path<TransformIdPath>,
    body: web::Json<SaveTransformParams>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_owner(&app, transform_id, &jwt).await {
        return resp;
    }
    let p = body.into_inner();
    match app.transforms_service.save_transform_draft(transform_id, p.source_code, p.resource_id).await {
        Ok(t) => HttpResponse::Ok().json(TransformDraftDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/transforms/{transform_id}/validate", tag = "transforms",
    params(TransformIdPath),
    responses((status = 200, description = "Whether the currently-saved draft is in a publishable state", body = serde_json::Value)))]
#[post("/{transform_id}/validate")]
pub async fn validate_transform_draft(
    jwt: JwtContext,
    path: web::Path<TransformIdPath>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_owner(&app, transform_id, &jwt).await {
        return resp;
    }
    match app.transforms_service.validate_composite_draft(transform_id).await {
        Ok(publishable) => HttpResponse::Ok().json(DraftPublishableDto { publishable }),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/transforms/{transform_id}/publish", tag = "transforms",
    params(TransformIdPath),
    responses(
        (status = 200, description = "Published transform", body = serde_json::Value),
        (status = 400, description = "Nothing saved with a successful build yet")
    ))]
#[post("/{transform_id}/publish")]
pub async fn publish_transform(
    jwt: JwtContext,
    path: web::Path<TransformIdPath>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_owner(&app, transform_id, &jwt).await {
        return resp;
    }
    match app.transforms_service.publish_transform(transform_id).await {
        Ok(t) => HttpResponse::Ok().json(TransformDto::from(t)),
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

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_transform_summaries)
        .service(get_transform_definition)
        .service(resolve_transform_definitions)
        .service(get_transform_binary)
        .service(get_transform_binaries)
        .service(create_transform)
        .service(save_transform)
        .service(validate_transform_draft)
        .service(publish_transform)
        .service(delete_transform);
}

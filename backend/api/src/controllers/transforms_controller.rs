use crate::{
    domain::service_error::ServiceError,
    middlewares::role_context::role_context::RoleContext,
    transforms::transforms_app_data::TransformsAppData,
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use domain::db::{db_transform::{DbTransform, DbTransformDefinition, DbTransformParam, DbTransformPort, TransformId}, ticket::db_resource::ResourceId};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::{IntoParams, ToSchema};

// ─── Request / Response types ────────────────────────────────────────────────

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateTransformParams {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
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
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransformPortDto {
    pub port_id: i64,
    pub name: String,
    pub direction: String,
    pub port_order: i32,
    pub description: Option<String>,
    /// "program" | "sidechain".
    pub kind: String,
    /// "single" | "many".
    pub cardinality: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransformParamDto {
    pub param_id: i64,
    pub name: String,
    pub param_order: i32,
    pub default_value: f32,
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransformDefinitionDto {
    pub transform_id: TransformId,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub source_code: Option<String>,
    pub ports: Vec<TransformPortDto>,
    pub params: Vec<TransformParamDto>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TransformIdsRequest {
    pub ids: Vec<TransformId>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransformDefinitionsResponse {
    pub transforms: Vec<TransformDefinitionDto>,
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

#[derive(Deserialize, IntoParams)]
pub struct TransformIdPath {
    pub transform_id: TransformId,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PortShapeSummaryDto {
    pub name: String,
    pub direction: String,
    pub kind: String,
    pub cardinality: String,
}

/// Advisory pre-publish check — see `TransformsProvider::diff_publish_port_shape`.
/// The creator's Publish flow polls this immediately before calling
/// `POST /transforms/{id}/publish` and shows a non-blocking confirm dialog
/// when `changed` is true. The backend never blocks on this itself.
#[derive(Debug, Serialize, ToSchema)]
pub struct PublishPortShapeDiffDto {
    pub changed: bool,
    pub current: Vec<PortShapeSummaryDto>,
    pub incoming: Vec<PortShapeSummaryDto>,
}

impl From<crate::transforms::transforms_provider::PortShapeSummary> for PortShapeSummaryDto {
    fn from(value: crate::transforms::transforms_provider::PortShapeSummary) -> Self {
        Self { name: value.name, direction: value.direction, kind: value.kind, cardinality: value.cardinality }
    }
}

impl From<crate::transforms::transforms_provider::PublishPortShapeDiff> for PublishPortShapeDiffDto {
    fn from(value: crate::transforms::transforms_provider::PublishPortShapeDiff) -> Self {
        Self {
            changed: value.changed,
            current: value.current.into_iter().map(PortShapeSummaryDto::from).collect(),
            incoming: value.incoming.into_iter().map(PortShapeSummaryDto::from).collect(),
        }
    }
}

impl From<DbTransform> for TransformSummaryDto {
    fn from(value: DbTransform) -> Self {
        Self {
            transform_id: value.transform_id,
            name: value.name,
            description: value.description,
            icon: value.icon,
        }
    }
}

impl From<DbTransformPort> for TransformPortDto {
    fn from(value: DbTransformPort) -> Self {
        Self {
            port_id: value.port_id,
            name: value.name,
            direction: value.direction,
            port_order: value.port_order,
            description: value.description,
            kind: value.kind,
            cardinality: value.cardinality,
        }
    }
}

impl From<DbTransformParam> for TransformParamDto {
    fn from(value: DbTransformParam) -> Self {
        Self {
            param_id: value.param_id,
            name: value.name,
            param_order: value.param_order,
            default_value: value.default_value,
            min_value: value.min_value,
            max_value: value.max_value,
            description: value.description,
        }
    }
}

impl From<DbTransformDefinition> for TransformDefinitionDto {
    fn from(value: DbTransformDefinition) -> Self {
        Self {
            transform_id: value.transform_id,
            name: value.name,
            description: value.description,
            icon: value.icon,
            source_code: value.source_code,
            ports: value.ports.into_iter().map(TransformPortDto::from).collect(),
            params: value.params.into_iter().map(TransformParamDto::from).collect(),
        }
    }
}

impl From<domain::db::db_transform::DbTransformBinary> for TransformBinaryDto {
    fn from(value: domain::db::db_transform::DbTransformBinary) -> Self {
        Self {
            transform_id: value.transform_id,
            wasm_base64: BASE64_STANDARD.encode(value.wasm_bytecode),
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
    _role: RoleContext,
    query: web::Query<PaginationQuery>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
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
    _role: RoleContext,
    path: web::Path<TransformIdPath>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    match app.transforms_service.get_transform_definition(path.into_inner().transform_id).await {
        Ok(t) => HttpResponse::Ok().json(TransformDefinitionDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/transforms/resolve", tag = "transforms",
    request_body = TransformIdsRequest,
    responses((status = 200, description = "Resolved transform definitions", body = serde_json::Value)))]
#[post("/resolve")]
pub async fn resolve_transform_definitions(
    _role: RoleContext,
    body: web::Json<TransformIdsRequest>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    let request = body.into_inner();
    match app.transforms_service.get_transform_definitions(&request.ids).await {
        Ok(transforms) => HttpResponse::Ok().json(TransformDefinitionsResponse {
            transforms: transforms.into_iter().map(TransformDefinitionDto::from).collect(),
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
    _role: RoleContext,
    path: web::Path<TransformIdPath>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    match app.transforms_service.get_transform_binary(path.into_inner().transform_id).await {
        Ok(bytes) => HttpResponse::Ok()
            .content_type("application/wasm")
            .body(bytes),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/transforms/binaries", tag = "transforms",
    request_body = TransformIdsRequest,
    responses((status = 200, description = "Resolved transform binaries", body = serde_json::Value)))]
#[post("/binaries")]
pub async fn get_transform_binaries(
    _role: RoleContext,
    body: web::Json<TransformIdsRequest>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    let request = body.into_inner();
    match app.transforms_service.get_transform_binaries(&request.ids).await {
        Ok(binaries) => HttpResponse::Ok().json(TransformBinariesResponse {
            binaries: binaries.into_iter().map(TransformBinaryDto::from).collect(),
        }),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/transforms", tag = "transforms",
    request_body = CreateTransformParams,
    responses((status = 200, description = "Created transform", body = serde_json::Value)))]
#[post("")]
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
        Ok(t) => HttpResponse::Ok().json(TransformDefinitionDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(put, path = "/transforms/{transform_id}/save", tag = "transforms",
    params(TransformIdPath),
    request_body = SaveTransformParams,
    responses((status = 200, description = "Saved transform state", body = serde_json::Value)))]
#[put("/{transform_id}/save")]
pub async fn save_transform(
    role: RoleContext,
    path: web::Path<TransformIdPath>,
    body: web::Json<SaveTransformParams>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let p = body.into_inner();
    match app.transforms_service.save_transform_state(path.into_inner().transform_id, p.source_code, p.resource_id).await {
        Ok(t) => HttpResponse::Ok().json(TransformDefinitionDto::from(t)),
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
    role: RoleContext,
    path: web::Path<TransformIdPath>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    match app.transforms_service.publish_transform(path.into_inner().transform_id).await {
        Ok(t) => HttpResponse::Ok().json(TransformDefinitionDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(get, path = "/transforms/{transform_id}/publish/port-diff", tag = "transforms",
    params(TransformIdPath),
    responses((status = 200, description = "Whether the about-to-be-published port shape differs from what's currently live", body = serde_json::Value)))]
#[get("/{transform_id}/publish/port-diff")]
pub async fn get_publish_port_shape_diff(
    _role: RoleContext,
    path: web::Path<TransformIdPath>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    match app.transforms_service.diff_publish_port_shape(path.into_inner().transform_id).await {
        Ok(diff) => HttpResponse::Ok().json(PublishPortShapeDiffDto::from(diff)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(delete, path = "/transforms/{transform_id}", tag = "transforms",
    params(TransformIdPath),
    responses((status = 200, description = "Deleted")))]
#[delete("/{transform_id}")]
pub async fn delete_transform(
    role: RoleContext,
    path: web::Path<TransformIdPath>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    match app.transforms_service.delete_transform(path.into_inner().transform_id).await {
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
        .service(publish_transform)
        .service(get_publish_port_shape_diff)
        .service(delete_transform);
}

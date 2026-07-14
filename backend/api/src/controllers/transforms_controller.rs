use crate::{
    domain::service_error::ServiceError,
    middlewares::jwt::jwt_context::JwtContext,
    middlewares::role_context::role_context::RoleContext,
    transforms::{compile_params::RequestCompileParams, compile_result::CompileResult},
    transforms::transforms_app_data::TransformsAppData,
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use domain::{db::{
    db_transform::{DbTransform, DbTransformDefinition, DbTransformParam, DbTransformPort, TransformId},
    ticket::{
        db_resource::ResourceId,
        db_ticket::{DbTicket, TicketId},
        ticket_status::TicketStatus,
    },
}, domain_user::UserId};
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

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCompileTicketRequest {
    pub transform_id: TransformId,
    pub source_code: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompileTicketStatusDto {
    pub state: String,
    pub resource_id: Option<ResourceId>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompileTicketDto {
    pub ticket_id: TicketId,
    pub issued_by: i64,
    pub status: CompileTicketStatusDto,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompileResourceDto {
    pub resource_id: ResourceId,
    pub ticket_id: TicketId,
}

#[derive(Deserialize, IntoParams)]
pub struct TransformIdPath {
    pub transform_id: TransformId,
}

#[derive(Deserialize, IntoParams)]
pub struct TicketIdPath {
    pub ticket_id: TicketId,
}

#[derive(Deserialize, IntoParams)]
pub struct ResourceIdPath {
    pub resource_id: ResourceId,
}

#[derive(Deserialize)]
pub struct PortIdQuery {
    pub port_id: i64,
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

impl From<TicketStatus> for CompileTicketStatusDto {
    fn from(value: TicketStatus) -> Self {
        match value {
            TicketStatus::Processing => Self {
                state: "processing".to_string(),
                resource_id: None,
                message: None,
            },
            TicketStatus::Failed { message } => Self {
                state: "failed".to_string(),
                resource_id: None,
                message: Some(message),
            },
            TicketStatus::Successful { resource_id } => Self {
                state: "successful".to_string(),
                resource_id: Some(resource_id),
                message: None,
            },
        }
    }
}

impl From<DbTicket> for CompileTicketDto {
    fn from(value: DbTicket) -> Self {
        Self {
            ticket_id: value.id,
            issued_by: value.issued_by,
            status: value.status.into(),
            timestamp: value.timestamp,
        }
    }
}

impl From<CompileResult> for CompileResourceDto {
    fn from(value: CompileResult) -> Self {
        Self {
            resource_id: value.resource_id,
            ticket_id: value.ticket_id,
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

#[utoipa::path(post, path = "/transforms/tickets", tag = "transforms",
    request_body = CreateCompileTicketRequest,
    responses((status = 201, description = "Compile ticket created", body = CompileTicketDto)))]
#[post("/tickets")]
pub async fn create_compile_ticket(
    auth: JwtContext,
    role: RoleContext,
    body: web::Json<CreateCompileTicketRequest>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let request = body.into_inner();
    if request.source_code.trim().is_empty() {
        return HttpResponse::BadRequest().body("sourceCode is required");
    }

    match app
        .transforms_service
        .request_compile_transform(RequestCompileParams {
            user_id: UserId::from(auth.user_id),
            transform_id: request.transform_id,
            payload: request.source_code,
        })
        .await
    {
        Ok(ticket) => HttpResponse::Created().json(CompileTicketDto::from(ticket)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(get, path = "/transforms/tickets/{ticket_id}", tag = "transforms",
    params(TicketIdPath),
    responses((status = 200, description = "Compile ticket status", body = CompileTicketDto)))]
#[get("/tickets/{ticket_id}")]
pub async fn get_compile_ticket_status(
    role: RoleContext,
    path: web::Path<TicketIdPath>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    match app
        .transforms_service
        .get_compile_ticket_status(path.into_inner().ticket_id)
        .await
    {
        Ok(ticket) => HttpResponse::Ok().json(CompileTicketDto::from(ticket)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(get, path = "/transforms/resources/{resource_id}", tag = "transforms",
    params(ResourceIdPath),
    responses((status = 200, description = "Compile resource", body = CompileResourceDto)))]
#[get("/resources/{resource_id}")]
pub async fn get_compile_resource(
    role: RoleContext,
    path: web::Path<ResourceIdPath>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    match app
        .transforms_service
        .get_ticket_result(path.into_inner().resource_id)
        .await
    {
        Ok(resource) => HttpResponse::Ok().json(CompileResourceDto::from(resource)),
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

#[utoipa::path(put, path = "/transforms/{transform_id}", tag = "transforms",
    params(TransformIdPath),
    request_body = UpdateTransformParams,
    responses((status = 200, description = "Updated transform", body = serde_json::Value)))]
#[put("/{transform_id}")]
pub async fn update_transform(
    role: RoleContext,
    path: web::Path<TransformIdPath>,
    body: web::Json<UpdateTransformParams>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let p = body.into_inner();
    match app.transforms_service.update_transform(path.into_inner().transform_id, p.name, p.description, p.icon).await {
        Ok(t) => HttpResponse::Ok().json(TransformDefinitionDto::from(t)),
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

#[utoipa::path(post, path = "/transforms/{transform_id}/ports", tag = "transforms",
    params(TransformIdPath),
    request_body = AddPortParams,
    responses((status = 200, description = "Added port", body = serde_json::Value)))]
#[post("/{transform_id}/ports")]
pub async fn add_port(
    role: RoleContext,
    path: web::Path<TransformIdPath>,
    body: web::Json<AddPortParams>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let p = body.into_inner();
    match app.transforms_service.add_port(path.into_inner().transform_id, p.name, p.direction, p.port_order, p.description).await {
        Ok(port) => HttpResponse::Ok().json(TransformPortDto::from(port)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(delete, path = "/transforms/ports/{port_id}", tag = "transforms",
    params(("port_id" = i64, Path, description = "Port ID")),
    responses((status = 200, description = "Port deleted")))]
#[delete("/ports/{port_id}")]
pub async fn delete_port(
    role: RoleContext,
    path: web::Path<PortIdQuery>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    match app.transforms_service.delete_port(path.into_inner().port_id).await {
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
        .service(create_compile_ticket)
        .service(get_compile_ticket_status)
        .service(get_compile_resource)
        .service(create_transform)
        .service(update_transform)
        .service(delete_transform)
        .service(add_port)
        .service(delete_port);
}

use crate::{
    domain::service_error::ServiceError,
    middlewares::membership::membership_context::RoleContext,
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

/// Mirrors `domain::db::transform_snapshot::CompositeNodePosition` exactly.
/// A local DTO (rather than reusing the domain type directly) purely so it
/// can derive `ToSchema` for the OpenAPI doc — `#[serde(default)]` on both
/// fields for the same backward-compat reason as the domain type: an old
/// client/response predating this field has no `position` key at all.
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, Copy, Default)]
pub struct CompositeNodePositionDto {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
}

impl From<CompositeNodePositionDto> for domain::db::transform_snapshot::CompositeNodePosition {
    fn from(value: CompositeNodePositionDto) -> Self {
        Self { x: value.x, y: value.y }
    }
}

impl From<domain::db::transform_snapshot::CompositeNodePosition> for CompositeNodePositionDto {
    fn from(value: domain::db::transform_snapshot::CompositeNodePosition) -> Self {
        Self { x: value.x, y: value.y }
    }
}

/// Mirrors `domain::db::transform_snapshot::CompositeNode` exactly (same
/// `node_kind` tag field, same variant tag values, same per-variant field
/// names) — see that type's doc comment for why. An Input/Output node has
/// no `transform_id` (nothing left over from `CompositeExposedPort`, which
/// this replaces); a Leaf node has no `name`.
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
#[serde(tag = "node_kind", rename_all = "lowercase")]
pub enum CompositeNodeDto {
    Leaf {
        node_id: i64,
        transform_id: TransformId,
        #[serde(default)]
        position: CompositeNodePositionDto,
    },
    Input {
        node_id: i64,
        name: String,
        #[serde(default)]
        position: CompositeNodePositionDto,
    },
    Output {
        node_id: i64,
        name: String,
        #[serde(default)]
        position: CompositeNodePositionDto,
    },
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct CompositeEdgeDto {
    pub from_node_id: i64,
    pub from_port: String,
    pub to_node_id: i64,
    pub to_port: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct CompositeGraphDefinitionDto {
    pub nodes: Vec<CompositeNodeDto>,
    pub edges: Vec<CompositeEdgeDto>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SaveCompositeGraphParams {
    pub graph_definition: CompositeGraphDefinitionDto,
}

impl From<CompositeNodeDto> for domain::db::transform_snapshot::CompositeNode {
    fn from(value: CompositeNodeDto) -> Self {
        match value {
            CompositeNodeDto::Leaf { node_id, transform_id, position } => {
                Self::Leaf { node_id, transform_id, position: position.into() }
            }
            CompositeNodeDto::Input { node_id, name, position } => {
                Self::Input { node_id, name, position: position.into() }
            }
            CompositeNodeDto::Output { node_id, name, position } => {
                Self::Output { node_id, name, position: position.into() }
            }
        }
    }
}

impl From<domain::db::transform_snapshot::CompositeNode> for CompositeNodeDto {
    fn from(value: domain::db::transform_snapshot::CompositeNode) -> Self {
        use domain::db::transform_snapshot::CompositeNode as DomainNode;
        match value {
            DomainNode::Leaf { node_id, transform_id, position } => {
                Self::Leaf { node_id, transform_id, position: position.into() }
            }
            DomainNode::Input { node_id, name, position } => {
                Self::Input { node_id, name, position: position.into() }
            }
            DomainNode::Output { node_id, name, position } => {
                Self::Output { node_id, name, position: position.into() }
            }
        }
    }
}

impl From<CompositeGraphDefinitionDto> for domain::db::transform_snapshot::CompositeTransformDefinition {
    fn from(value: CompositeGraphDefinitionDto) -> Self {
        Self {
            nodes: value.nodes.into_iter().map(domain::db::transform_snapshot::CompositeNode::from).collect(),
            edges: value
                .edges
                .into_iter()
                .map(|e| domain::db::transform_snapshot::CompositeEdge {
                    from_node_id: e.from_node_id,
                    from_port: e.from_port,
                    to_node_id: e.to_node_id,
                    to_port: e.to_port,
                })
                .collect(),
        }
    }
}

impl From<domain::db::transform_snapshot::CompositeTransformDefinition> for CompositeGraphDefinitionDto {
    fn from(value: domain::db::transform_snapshot::CompositeTransformDefinition) -> Self {
        Self {
            nodes: value.nodes.into_iter().map(CompositeNodeDto::from).collect(),
            edges: value
                .edges
                .into_iter()
                .map(|e| CompositeEdgeDto {
                    from_node_id: e.from_node_id,
                    from_port: e.from_port,
                    to_node_id: e.to_node_id,
                    to_port: e.to_port,
                })
                .collect(),
        }
    }
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
    /// Live in transform_binary (primitive) or transform_composite (composite).
    pub published: bool,
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
    /// "primitive" | "composite".
    pub kind: String,
    pub source_code: Option<String>,
    /// Present only for kind = "composite".
    pub graph_definition: Option<CompositeGraphDefinitionDto>,
    pub ports: Vec<TransformPortDto>,
    pub params: Vec<TransformParamDto>,
    /// Composite-only sub-state between Save and Publish: true once the
    /// currently-persisted graph_definition has passed the explicit validate
    /// action (`POST /transforms/{id}/validate`). Any subsequent save flips
    /// this back to false. Always false for primitives. See
    /// agents/decisions/0007-composite-draft-validation-gate.md.
    pub is_validated: bool,
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
            kind: value.kind,
            published: value.published,
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
            kind: value.kind,
            source_code: value.source_code,
            graph_definition: value.graph_definition.map(CompositeGraphDefinitionDto::from),
            ports: value.ports.into_iter().map(TransformPortDto::from).collect(),
            params: value.params.into_iter().map(TransformParamDto::from).collect(),
            is_validated: value.is_validated,
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
    if p.kind != "primitive" && p.kind != "composite" {
        return HttpResponse::BadRequest().body("kind must be 'primitive' or 'composite'");
    }
    match app.transforms_service.create_transform(p.name, p.description, p.icon, p.kind).await {
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
    match app.transforms_service.save_transform_draft(path.into_inner().transform_id, p.source_code, p.resource_id).await {
        Ok(t) => HttpResponse::Ok().json(TransformDefinitionDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(put, path = "/transforms/{transform_id}/save-composite", tag = "transforms",
    params(TransformIdPath),
    request_body = SaveCompositeGraphParams,
    responses((status = 200, description = "Saved composite transform graph", body = serde_json::Value)))]
#[put("/{transform_id}/save-composite")]
pub async fn save_composite_transform(
    role: RoleContext,
    path: web::Path<TransformIdPath>,
    body: web::Json<SaveCompositeGraphParams>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    let p = body.into_inner();
    let graph = domain::db::transform_snapshot::CompositeTransformDefinition::from(p.graph_definition);
    match app.transforms_service.save_composite_draft(path.into_inner().transform_id, graph).await {
        Ok(t) => HttpResponse::Ok().json(TransformDefinitionDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/transforms/{transform_id}/validate", tag = "transforms",
    params(TransformIdPath),
    responses(
        (status = 200, description = "Composite graph validated; ports derived and is_validated set to true", body = serde_json::Value),
        (status = 400, description = "Validation failed (invalid wiring, or nothing saved yet) — ports/is_validated left untouched")
    ))]
#[post("/{transform_id}/validate")]
pub async fn validate_composite_transform(
    role: RoleContext,
    path: web::Path<TransformIdPath>,
    app: web::Data<TransformsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    match app.transforms_service.validate_composite_draft(path.into_inner().transform_id).await {
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
        .service(save_composite_transform)
        .service(validate_composite_transform)
        .service(publish_transform)
        .service(get_publish_port_shape_diff)
        .service(delete_transform);
}

use crate::{
    domain::service_error::ServiceError,
    middlewares::jwt::jwt_context::JwtContext,
    middlewares::membership::membership_context::RoleContext,
    tickets::{compile_params::RequestCompileParams, compile_result::CompileResult},
    tickets::tickets_app_data::TicketsAppData,
};
use actix_web::{get, post, web, HttpResponse};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use domain::{db::{
    db_transform::TransformId,
    ticket::{
        db_resource::ResourceId,
        db_ticket::{DbTicket, TicketId},
        ticket_status::TicketStatus,
    },
    transform_snapshot::ParamSnapshot,
}, domain_user::UserId};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::{IntoParams, ToSchema};

// ─── Request / Response types ────────────────────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCompileTicketRequest {
    pub transform_id: TransformId,
    pub source_code: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompileParamDto {
    pub name: String,
    pub param_order: i32,
    pub default_value: f32,
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
    pub description: Option<String>,
}

impl From<ParamSnapshot> for CompileParamDto {
    fn from(value: ParamSnapshot) -> Self {
        Self {
            name: value.name,
            param_order: value.param_order,
            default_value: value.default_value,
            min_value: value.min_value,
            max_value: value.max_value,
            description: value.description,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompileTicketStatusDto {
    pub state: String,
    pub resource_id: Option<ResourceId>,
    pub message: Option<String>,
    /// Present only once state == "successful" — the compiled binary,
    /// base64-encoded (same encoding `TransformBinaryDto` uses for published
    /// binaries), so the creator surface can run a "Try it" preview of this
    /// exact build before deciding to save. Never sent back to the backend —
    /// see agents/decisions/0003-transform-preview-flow.md.
    pub wasm_base64: Option<String>,
    /// Present alongside wasm_base64 — the introspected params (with
    /// default_value), so "Try it" can seed the preview graph with this
    /// build's real declared defaults instead of zeros. Ports aren't needed
    /// here: preview always wires exactly one raw input to one sink output,
    /// so port identity/order doesn't affect execution the way param values do.
    pub params: Option<Vec<CompileParamDto>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompileTicketDto {
    pub ticket_id: TicketId,
    pub issued_by: i64,
    pub status: CompileTicketStatusDto,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompileResourceDto {
    pub resource_id: ResourceId,
    pub ticket_id: TicketId,
    /// The compiled binary, base64-encoded — see the note on
    /// `CompileTicketStatusDto::wasm_base64`.
    pub wasm_base64: String,
    pub params: Vec<CompileParamDto>,
}

#[derive(Deserialize, IntoParams)]
pub struct TicketIdPath {
    pub ticket_id: TicketId,
}

#[derive(Deserialize, IntoParams)]
pub struct ResourceIdPath {
    pub resource_id: ResourceId,
}

impl From<TicketStatus> for CompileTicketStatusDto {
    fn from(value: TicketStatus) -> Self {
        match value {
            TicketStatus::Processing => Self {
                state: "processing".to_string(),
                resource_id: None,
                message: None,
                wasm_base64: None,
                params: None,
            },
            TicketStatus::Failed { message } => Self {
                state: "failed".to_string(),
                resource_id: None,
                message: Some(message),
                wasm_base64: None,
                params: None,
            },
            TicketStatus::Successful { resource_id } => Self {
                state: "successful".to_string(),
                resource_id: Some(resource_id),
                message: None,
                // Filled in by the handler (needs a second fetch to get the
                // resource's bytecode/params) — TicketStatus alone doesn't
                // carry them.
                wasm_base64: None,
                params: None,
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
            wasm_base64: BASE64_STANDARD.encode(&value.wasm_bytecode),
            params: value.params.into_iter().map(CompileParamDto::from).collect(),
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

#[utoipa::path(post, path = "/transforms/tickets", tag = "tickets",
    request_body = CreateCompileTicketRequest,
    responses((status = 201, description = "Compile ticket created", body = CompileTicketDto)))]
#[post("/tickets")]
pub async fn create_compile_ticket(
    auth: JwtContext,
    role: RoleContext,
    body: web::Json<CreateCompileTicketRequest>,
    app: web::Data<TicketsAppData>,
) -> HttpResponse {
    if !role.can_edit() {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let request = body.into_inner();
    if request.source_code.trim().is_empty() {
        return HttpResponse::BadRequest().body("sourceCode is required");
    }

    match app
        .tickets_service
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

#[utoipa::path(get, path = "/transforms/tickets/{ticket_id}", tag = "tickets",
    params(TicketIdPath),
    responses((status = 200, description = "Compile ticket status", body = CompileTicketDto)))]
#[get("/tickets/{ticket_id}")]
pub async fn get_compile_ticket_status(
    role: RoleContext,
    path: web::Path<TicketIdPath>,
    app: web::Data<TicketsAppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    match app
        .tickets_service
        .get_compile_ticket_status(path.into_inner().ticket_id)
        .await
    {
        Ok(ticket) => {
            // TicketStatus::Successful only carries a resource_id, not the
            // bytecode itself — fetch the resource so the "Try it" preview
            // flow can decode-and-run this exact build client-side. See
            // agents/decisions/0003-transform-preview-flow.md.
            let successful_resource_id = match &ticket.status {
                TicketStatus::Successful { resource_id } => Some(*resource_id),
                _ => None,
            };

            let mut dto = CompileTicketDto::from(ticket);

            if let Some(resource_id) = successful_resource_id {
                match app.tickets_service.get_ticket_result(resource_id).await {
                    Ok(result) => {
                        dto.status.wasm_base64 = Some(BASE64_STANDARD.encode(&result.wasm_bytecode));
                        dto.status.params = Some(result.params.into_iter().map(CompileParamDto::from).collect());
                    }
                    Err(e) => {
                        // Don't fail the whole status response over this —
                        // the ticket status itself is still valid, preview
                        // just won't be available this poll.
                        error!(error = %e, resource_id, "failed to fetch compile resource for preview binary");
                    }
                }
            }

            HttpResponse::Ok().json(dto)
        }
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(get, path = "/transforms/resources/{resource_id}", tag = "tickets",
    params(ResourceIdPath),
    responses((status = 200, description = "Compile resource", body = CompileResourceDto)))]
#[get("/resources/{resource_id}")]
pub async fn get_compile_resource(
    role: RoleContext,
    path: web::Path<ResourceIdPath>,
    app: web::Data<TicketsAppData>,
) -> HttpResponse {
    if !role.can_view() {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    match app
        .tickets_service
        .get_ticket_result(path.into_inner().resource_id)
        .await
    {
        Ok(resource) => HttpResponse::Ok().json(CompileResourceDto::from(resource)),
        Err(e) => map_service_error(e),
    }
}

// ─── Route registration ───────────────────────────────────────────────────────

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_compile_ticket)
        .service(get_compile_ticket_status)
        .service(get_compile_resource);
}

use crate::{
    domain::service_error::ServiceError,
    middlewares::{
        authz::{
            transform_access_context::TransformAccessContext,
            transform_authz::{require_access, require_owner},
        },
        jwt::jwt_context::JwtContext,
    },
    tickets::{compile_params::RequestCompileParams, compile_result::CompileResult},
    tickets::tickets_app_data::TicketsAppData,
    transforms::transforms_app_data::TransformsAppData,
};
use actix_web::{get, post, web, HttpResponse};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use domain::{db::{
    db_transform::TransformId,
    ticket::{
        db_resource::ResourceId,
        db_ticket::{DbTicket, TicketId},
        ticket_status::TicketStatus,
    }
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
pub struct CompileTicketStatusDto {
    pub state: String,
    pub resource_id: Option<ResourceId>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompileTicketDto {
    pub ticket_id: TicketId,
    pub issued_by: i64,
    pub status: CompileTicketStatusDto,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompileResultDto {
    pub resource_id: ResourceId,
    pub ticket_id: TicketId,
    /// Exact source submitted to the ticket which produced this binary.
    pub source_code: String,
    /// The compiled binary, base64-encoded for the Creator's temporary
    /// source/WASM package.
    pub wasm_base64: String,
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

impl From<CompileResult> for CompileResultDto {
    fn from(value: CompileResult) -> Self {
        Self {
            resource_id: value.resource_id,
            ticket_id: value.ticket_id,
            source_code: value.source_code,
            wasm_base64: BASE64_STANDARD.encode(&value.wasm_bytecode),
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
    body: web::Json<CreateCompileTicketRequest>,
    app: web::Data<TicketsAppData>,
    transforms_app: web::Data<TransformsAppData>,
) -> HttpResponse {
    let request = body.into_inner();

    if let Err(resp) = require_owner(&transforms_app, request.transform_id, &auth).await {
        return resp;
    }

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
    jwt: JwtContext,
    path: web::Path<TicketIdPath>,
    app: web::Data<TicketsAppData>,
    transforms_app: web::Data<TransformsAppData>,
    access: TransformAccessContext,
) -> HttpResponse {
    let ticket_id = path.into_inner().ticket_id;

    let transform_id = match app.tickets_service.get_ticket_transform_id(ticket_id).await {
        Ok(id) => id,
        Err(e) => return map_service_error(e),
    };
    if let Err(resp) = require_access(&transforms_app, &access, transform_id, &jwt).await {
        return resp;
    }

    match app
        .tickets_service
        .get_compile_ticket_status(ticket_id)
        .await
    {
        Ok(ticket) => HttpResponse::Ok().json(CompileTicketDto::from(ticket)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(get, path = "/transforms/resources/{resource_id}", tag = "tickets",
    params(ResourceIdPath),
    responses((status = 200, description = "Compile resource", body = CompileResultDto)))]
#[get("/resources/{resource_id}")]
pub async fn get_compile_resource(
    jwt: JwtContext,
    path: web::Path<ResourceIdPath>,
    app: web::Data<TicketsAppData>,
    transforms_app: web::Data<TransformsAppData>,
    access: TransformAccessContext,
) -> HttpResponse {
    let resource_id = path.into_inner().resource_id;

    let transform_id = match app.tickets_service.get_compiled_transform_id(resource_id).await {
        Ok(id) => id,
        Err(e) => return map_service_error(e),
    };
    if let Err(resp) = require_access(&transforms_app, &access, transform_id, &jwt).await {
        return resp;
    }

    match app
        .tickets_service
        .get_ticket_result(resource_id)
        .await
    {
        Ok(resource) => HttpResponse::Ok().json(CompileResultDto::from(resource)),
        Err(e) => map_service_error(e),
    }
}

// ─── Route registration ───────────────────────────────────────────────────────

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_compile_ticket)
        .service(get_compile_ticket_status)
        .service(get_compile_resource);
}

#[cfg(test)]
mod tests {
    use super::CompileResultDto;
    use crate::tickets::compile_result::CompileResult;

    #[test]
    fn compile_resource_dto_keeps_ticket_source_paired_with_wasm() {
        let dto = CompileResultDto::from(CompileResult {
            resource_id: 7,
            ticket_id: 11,
            source_code: "pub struct Test;".to_string(),
            wasm_bytecode: vec![0, 97, 115, 109],
        });

        assert_eq!(dto.source_code, "pub struct Test;");
        assert_eq!(dto.wasm_base64, "AGFzbQ==");
    }
}

use crate::{
    domain::service_error::ServiceError,
    middlewares::{
        authz::{
            transform_access_context::TransformAccessContext,
            transform_draft_authz::{require_access, require_owner},
        },
        jwt::jwt_context::JwtContext,
    },
    transform_drafts::{
        dto::{
            requests::{
                CheckSourceParams, CreateTransformParams, SaveCompositeParams, SavePrimitiveParams,
                TransformDraftIdPath, TransformDraftIdsRequest, ValidateGraphParams,
            },
            responses::{TransformDraftDto, TransformDraftsResponse, ValidateGraphResponse},
        },
        transform_drafts_app_data::TransformDraftsAppData,
    },
    transforms::dto::responses::TransformDto,
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use domain::domain_user::UserId;
use tracing::error;

// ─── /draft_transforms handlers (bucket 2 + the actions that act on it) ───────

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

#[utoipa::path(post, path = "/draft_transforms", tag = "draft_transforms",
    request_body = CreateTransformParams,
    responses((status = 200, description = "Created transform draft", body = serde_json::Value)))]
#[post("")]
pub async fn create_transform_draft(
    jwt: JwtContext,
    body: web::Json<CreateTransformParams>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let p = body.into_inner();
    if p.kind != "primitive" && p.kind != "composite" {
        return HttpResponse::BadRequest().body("kind must be 'primitive' or 'composite'");
    }
    match app.transform_drafts_service.create_transform_draft(p.name, p.description, p.icon, p.kind, UserId::from(jwt.user_id)).await {
        Ok(t) => HttpResponse::Ok().json(TransformDraftDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(get, path = "/draft_transforms/{transform_id}", tag = "draft_transforms",
    params(TransformDraftIdPath),
    responses((status = 200, description = "Transform draft", body = serde_json::Value)))]
#[get("/{transform_id}")]
pub async fn get_transform_draft(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    app: web::Data<TransformDraftsAppData>,
    access: TransformAccessContext,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_access(&app, &access, transform_id, &jwt).await {
        return resp;
    }
    match app.transform_drafts_service.get_transform_draft(transform_id).await {
        Ok(t) => HttpResponse::Ok().json(TransformDraftDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/draft_transforms/resolve", tag = "draft_transforms",
    request_body = TransformDraftIdsRequest,
    responses((status = 200, description = "Resolved transform drafts", body = serde_json::Value)))]
#[post("/resolve")]
pub async fn get_transform_drafts(
    jwt: JwtContext,
    body: web::Json<TransformDraftIdsRequest>,
    app: web::Data<TransformDraftsAppData>,
    access: TransformAccessContext,
) -> HttpResponse {
    let request = body.into_inner();
    for id in &request.ids {
        if let Err(resp) = require_access(&app, &access, *id, &jwt).await {
            return resp;
        }
    }
    match app.transform_drafts_service.get_transform_drafts(&request.ids).await {
        Ok(drafts) => HttpResponse::Ok().json(TransformDraftsResponse {
            drafts: drafts.into_iter().map(TransformDraftDto::from).collect(),
        }),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(put, path = "/draft_transforms/{transform_id}/save-primitive", tag = "draft_transforms",
    params(TransformDraftIdPath),
    request_body = SavePrimitiveParams,
    responses(
        (status = 200, description = "Saved transform draft state", body = serde_json::Value),
        (status = 400, description = "transform_id is a composite draft")
    ))]
#[put("/{transform_id}/save-primitive")]
pub async fn save_primitive_draft(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    body: web::Json<SavePrimitiveParams>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_owner(&app, transform_id, &jwt).await {
        return resp;
    }
    let p = body.into_inner();
    match app.transform_drafts_service.save_primitive_draft(transform_id, p.source_code).await {
        Ok(t) => HttpResponse::Ok().json(TransformDraftDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(put, path = "/draft_transforms/{transform_id}/save-composite", tag = "draft_transforms",
    params(TransformDraftIdPath),
    request_body = SaveCompositeParams,
    responses(
        (status = 200, description = "Saved transform draft state", body = serde_json::Value),
        (status = 400, description = "transform_id is a primitive draft")
    ))]
#[put("/{transform_id}/save-composite")]
pub async fn save_composite_draft(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    body: web::Json<SaveCompositeParams>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_owner(&app, transform_id, &jwt).await {
        return resp;
    }
    let p = body.into_inner();
    match app.transform_drafts_service.save_composite_draft(transform_id, p.graph_json).await {
        Ok(t) => HttpResponse::Ok().json(TransformDraftDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/draft_transforms/{transform_id}/validate-source", tag = "draft_transforms",
    params(TransformDraftIdPath),
    request_body = CheckSourceParams,
    responses(
        (status = 200, description = "Source compiles cleanly (cargo check, no codegen)"),
        (status = 400, description = "Compiler diagnostics")
    ))]
#[post("/{transform_id}/validate-source")]
pub async fn validate_source(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    body: web::Json<CheckSourceParams>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_owner(&app, transform_id, &jwt).await {
        return resp;
    }
    let p = body.into_inner();
    match app.transform_drafts_service.check_source(p.source_code).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/draft_transforms/{transform_id}/validate-graph", tag = "draft_transforms",
    params(TransformDraftIdPath),
    request_body = ValidateGraphParams,
    responses(
        (status = 200, description = "Derived composite ports on success", body = serde_json::Value),
        (status = 400, description = "Graph is malformed, or references a transform that doesn't exist, isn't published, or is the wrong kind")
    ))]
#[post("/{transform_id}/validate-graph")]
pub async fn validate_graph_draft(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    body: web::Json<ValidateGraphParams>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_owner(&app, transform_id, &jwt).await {
        return resp;
    }
    let p = body.into_inner();
    match app.transform_drafts_service.validate_graph_draft(transform_id, p.graph_json).await {
        Ok(ports) => HttpResponse::Ok().json(ValidateGraphResponse { ports }),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/draft_transforms/{transform_id}/publish-primitive", tag = "draft_transforms",
    params(TransformDraftIdPath),
    responses(
        (status = 200, description = "Published transform", body = serde_json::Value),
        (status = 400, description = "Nothing saved with a successful build yet, or transform_id is a composite draft")
    ))]
#[post("/{transform_id}/publish-primitive")]
pub async fn publish_primitive(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_owner(&app, transform_id, &jwt).await {
        return resp;
    }
    match app.transform_drafts_service.publish_primitive(transform_id).await {
        Ok(t) => HttpResponse::Ok().json(TransformDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/draft_transforms/{transform_id}/publish-composite", tag = "draft_transforms",
    params(TransformDraftIdPath),
    responses(
        (status = 200, description = "Published transform", body = serde_json::Value),
        (status = 400, description = "Nothing saved yet, the saved graph no longer validates, or transform_id is a primitive draft")
    ))]
#[post("/{transform_id}/publish-composite")]
pub async fn publish_composite(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_owner(&app, transform_id, &jwt).await {
        return resp;
    }
    match app.transform_drafts_service.publish_composite(transform_id).await {
        Ok(t) => HttpResponse::Ok().json(TransformDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(delete, path = "/draft_transforms/{transform_id}", tag = "draft_transforms",
    params(TransformDraftIdPath),
    responses((status = 200, description = "Deleted")))]
#[delete("/{transform_id}")]
pub async fn delete_transform_draft(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_owner(&app, transform_id, &jwt).await {
        return resp;
    }
    match app.transform_drafts_service.delete_transform_draft(transform_id).await {
        Ok(_) => HttpResponse::Ok().body("Deleted"),
        Err(e) => map_service_error(e),
    }
}

// ─── Route registration ───────────────────────────────────────────────────────

/// Mounted at `/draft_transforms` — bucket 2 (create/save/read/delete) plus
/// the actions that act on a draft (validate-source, validate-graph, publish).
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_transform_draft)
        .service(get_transform_draft)
        .service(get_transform_drafts)
        .service(save_primitive_draft)
        .service(save_composite_draft)
        .service(validate_source)
        .service(validate_graph_draft)
        .service(publish_primitive)
        .service(publish_composite)
        .service(delete_transform_draft);
}

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
                CheckSourceCodeParams, CreateTransformParams, PublishDraftParams, SaveDraftParams,
                TransformDraftIdPath, TransformDraftIdsRequest,
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
    match app
        .transform_drafts_service
        .create_transform_draft(
            p.name,
            p.description,
            p.icon,
            p.kind,
            UserId::from(jwt.user_id),
        )
        .await
    {
        Ok(t) => HttpResponse::Ok().json(TransformDraftDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(get, path = "/draft_transforms/{draft_transform_id}", tag = "draft_transforms",
    params(TransformDraftIdPath),
    responses((status = 200, description = "Transform draft", body = serde_json::Value)))]
#[get("/{draft_transform_id}")]
pub async fn get_transform_draft(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    app: web::Data<TransformDraftsAppData>,
    access: TransformAccessContext,
) -> HttpResponse {
    let draft_transform_id = path.into_inner().draft_transform_id;
    if let Err(resp) = require_access(&app, &access, draft_transform_id, &jwt).await {
        return resp;
    }
    match app
        .transform_drafts_service
        .get_transform_draft(draft_transform_id)
        .await
    {
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
    match app
        .transform_drafts_service
        .get_transform_drafts(&request.ids)
        .await
    {
        Ok(drafts) => HttpResponse::Ok().json(TransformDraftsResponse {
            drafts: drafts.into_iter().map(TransformDraftDto::from).collect(),
        }),
        Err(e) => map_service_error(e),
    }
}

// name/description are optional fields on this payload (both variants) — see
// agents/decisions/0012-draft-name-description-editable.md. Omitted/`null`
// leaves the existing value untouched; provided, they're validated
// (non-blank name) and take priority over the existing/compiled value.
#[utoipa::path(put, path = "/draft_transforms/{draft_transform_id}/save", tag = "draft_transforms",
    params(TransformDraftIdPath),
    request_body = SaveDraftParams,
    responses(
        (status = 200, description = "Saved transform draft state", body = serde_json::Value),
        (status = 400, description = "Payload does not match the draft kind, WASM is invalid, or name is blank")
    ))]
#[put("/{draft_transform_id}/save")]
pub async fn save_draft(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    body: web::Json<SaveDraftParams>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let draft_transform_id = path.into_inner().draft_transform_id;
    if let Err(resp) = require_owner(&app, draft_transform_id, &jwt).await {
        return resp;
    }
    match app
        .transform_drafts_service
        .save_draft(draft_transform_id, body.into_inner())
        .await
    {
        Ok(t) => HttpResponse::Ok().json(TransformDraftDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/draft_transforms/{draft_transform_id}/validate-source-code", tag = "draft_transforms",
    params(TransformDraftIdPath),
    request_body = CheckSourceCodeParams,
    responses((status = 200, description = "Source code compiles cleanly"), (status = 400, description = "Compiler diagnostics")))]
#[post("/{draft_transform_id}/validate-source")]
pub async fn validate_transform_draft_source_code(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    body: web::Json<CheckSourceCodeParams>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let draft_transform_id = path.into_inner().draft_transform_id;
    if let Err(resp) = require_owner(&app, draft_transform_id, &jwt).await {
        return resp;
    }
    match app
        .transform_drafts_service
        .validate_source_code(body.into_inner().source_code)
        .await
    {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => map_service_error(e),
    }
}

/// Returns the binary already produced by a successful primitive-draft Save.
/// This is a read-only owner-only retrieval; it never compiles, saves, or
/// publishes a transform.
#[utoipa::path(get, path = "/draft_transforms/{draft_transform_id}/binary", tag = "draft_transforms",
    params(TransformDraftIdPath),
    responses((status = 200, description = "Saved primitive draft WASM binary"),
              (status = 400, description = "Draft is not primitive or has not been saved successfully")))]
#[get("/{draft_transform_id}/binary")]
pub async fn get_primitive_draft_binary(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let draft_transform_id = path.into_inner().draft_transform_id;
    if let Err(resp) = require_owner(&app, draft_transform_id, &jwt).await {
        return resp;
    }
    match app
        .transform_drafts_service
        .get_transform_draft(draft_transform_id)
        .await
    {
        Ok(draft) if draft.kind != "primitive" => {
            HttpResponse::BadRequest().body("only primitive drafts have a preview binary")
        }
        Ok(draft) => match draft.wasm_bytecode {
            Some(wasm) => HttpResponse::Ok()
                .insert_header(("Content-Type", "application/wasm"))
                .body(wasm),
            None => HttpResponse::BadRequest().body("save a successfully compiled draft before previewing"),
        },
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/draft_transforms/{draft_transform_id}/validate-graph", tag = "draft_transforms",
    params(TransformDraftIdPath),
    request_body = crate::transform_drafts::dto::requests::ValidateGraphParams,
    responses((status = 200, description = "Derived composite ports", body = serde_json::Value), (status = 400, description = "Invalid graph")))]
#[post("/{draft_transform_id}/validate-graph")]
pub async fn validate_graph_draft(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    body: web::Json<crate::transform_drafts::dto::requests::ValidateGraphParams>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let draft_transform_id = path.into_inner().draft_transform_id;
    if let Err(resp) = require_owner(&app, draft_transform_id, &jwt).await {
        return resp;
    }
    match app
        .transform_drafts_service
        .validate_graph_draft(draft_transform_id, body.into_inner().graph_json)
        .await
    {
        Ok(ports) => HttpResponse::Ok().json(ValidateGraphResponse { ports }),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(post, path = "/draft_transforms/{draft_transform_id}/publish", tag = "draft_transforms",
    params(TransformDraftIdPath),
    request_body = PublishDraftParams,
    responses(
        (status = 200, description = "Published transform", body = serde_json::Value),
        (status = 400, description = "Nothing saved yet (or with a successful build, for a primitive), the saved graph no longer validates, or the declared kind doesn't match the draft's kind")
    ))]
#[post("/{draft_transform_id}/publish")]
pub async fn publish_draft(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    body: web::Json<PublishDraftParams>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let draft_transform_id = path.into_inner().draft_transform_id;
    if let Err(resp) = require_owner(&app, draft_transform_id, &jwt).await {
        return resp;
    }
    match app
        .transform_drafts_service
        .publish(draft_transform_id, body.into_inner())
        .await
    {
        Ok(t) => HttpResponse::Ok().json(TransformDto::from(t)),
        Err(e) => map_service_error(e),
    }
}

#[utoipa::path(delete, path = "/draft_transforms/{draft_transform_id}", tag = "draft_transforms",
    params(TransformDraftIdPath),
    responses((status = 204, description = "Deleted")))]
#[delete("/{draft_transform_id}")]
pub async fn delete_transform_draft(
    jwt: JwtContext,
    path: web::Path<TransformDraftIdPath>,
    app: web::Data<TransformDraftsAppData>,
) -> HttpResponse {
    let draft_transform_id = path.into_inner().draft_transform_id;
    if let Err(resp) = require_owner(&app, draft_transform_id, &jwt).await {
        return resp;
    }
    match app
        .transform_drafts_service
        .delete_transform_draft(draft_transform_id)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
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
        .service(save_draft)
        .service(validate_transform_draft_source_code)
        .service(get_primitive_draft_binary)
        .service(validate_graph_draft)
        .service(publish_draft)
        .service(delete_transform_draft);
}

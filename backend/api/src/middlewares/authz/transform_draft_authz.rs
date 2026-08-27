use actix_web::HttpResponse;
use domain::db::{db_transform::TransformId, db_transform_draft::TransformDraftId};

use crate::{
    middlewares::{authz::transform_access_context::TransformAccessContext, jwt::jwt_context::JwtContext},
    transform_drafts::transform_drafts_app_data::TransformDraftsAppData,
};

/// Only the owner (or a SuperAdmin) may ever mutate a transform's draft —
/// editing never has a tiered/shared mode, unlike read access below.
pub async fn require_owner(
    app: &TransformDraftsAppData,
    transform_id: TransformDraftId,
    jwt: &JwtContext,
) -> Result<(), HttpResponse> {
    if jwt.is_admin {
        return Ok(());
    }

    match app.transform_drafts_service.get_transform_draft_owner(transform_id).await {
        Ok(owner_id) if owner_id == domain::domain_user::UserId::from(jwt.user_id) => Ok(()),
        Ok(_) => Err(HttpResponse::Forbidden().body("not the transform owner")),
        Err(crate::domain::service_error::ServiceError::NotFound) => {
            Err(HttpResponse::NotFound().body("not found"))
        }
        Err(_) => Err(HttpResponse::InternalServerError().body("failed to resolve transform owner")),
    }
}

/// Owner, SuperAdmin, or an active grant (direct-to-user or via a workspace
/// the caller belongs to) — used to gate reads (definition/binary/etc), never
/// mutation. The fast path checks the request-scoped `TransformAccessContext`
/// (loaded once by `TransformAccessMiddleware`); a miss falls back to a
/// point lookup only to tell "doesn't exist" apart from "exists, no access".
///
/// Grants are keyed by the published `TransformId` — a draft and its
/// transform share the same underlying row (see `TransformDraftId`'s doc
/// comment), so `TransformId::from(transform_id)` and the context built for
/// `/transforms` are both exact for drafts too.
pub async fn require_access(
    app: &TransformDraftsAppData,
    access: &TransformAccessContext,
    transform_id: TransformDraftId,
    jwt: &JwtContext,
) -> Result<(), HttpResponse> {
    if jwt.is_admin || access.contains(TransformId::from(transform_id)) {
        return Ok(());
    }

    match app.transform_drafts_service.get_transform_draft_owner(transform_id).await {
        Ok(_) => Err(HttpResponse::Forbidden().body("access denied to this transform")),
        Err(crate::domain::service_error::ServiceError::NotFound) => {
            Err(HttpResponse::NotFound().body("not found"))
        }
        Err(_) => Err(HttpResponse::InternalServerError().body("failed to resolve transform access")),
    }
}

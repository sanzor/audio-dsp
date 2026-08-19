use actix_web::HttpResponse;
use domain::db::TransformId;

use crate::{
    middlewares::jwt::jwt_context::JwtContext,
    transform_grants::transform_grants_app_data::TransformGrantsAppData,
    transforms::transforms_app_data::TransformsAppData,
};

/// Only the owner (or a SuperAdmin) may ever mutate a transform's draft —
/// editing never has a tiered/shared mode, unlike read access below.
pub async fn require_owner(
    app: &TransformsAppData,
    transform_id: TransformId,
    jwt: &JwtContext,
) -> Result<(), HttpResponse> {
    if jwt.is_admin {
        return Ok(());
    }

    match app.transforms_service.get_transform_owner(transform_id).await {
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
/// mutation.
pub async fn require_access(
    app: &TransformsAppData,
    grants: &TransformGrantsAppData,
    transform_id: TransformId,
    jwt: &JwtContext,
) -> Result<(), HttpResponse> {
    if jwt.is_admin {
        return Ok(());
    }

    match app.transforms_service.get_transform_owner(transform_id).await {
        Ok(owner_id) if owner_id == domain::domain_user::UserId::from(jwt.user_id) => return Ok(()),
        Ok(_) => {}
        Err(crate::domain::service_error::ServiceError::NotFound) => {
            return Err(HttpResponse::NotFound().body("not found"));
        }
        Err(_) => {
            return Err(HttpResponse::InternalServerError().body("failed to resolve transform owner"));
        }
    }

    match grants.transform_grants_service.has_access(transform_id, jwt.user_id).await {
        Ok(true) => Ok(()),
        Ok(false) => Err(HttpResponse::Forbidden().body("access denied to this transform")),
        Err(_) => Err(HttpResponse::InternalServerError().body("failed to resolve transform access")),
    }
}

use actix_web::{delete, get, post, web, HttpResponse};
use domain::db::{db_transform::TransformId, DbTransformGrant};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::{IntoParams, ToSchema};

use crate::{
    middlewares::{authz::transform_authz::require_owner, jwt::jwt_context::JwtContext},
    transform_grants::{
        transform_grants_app_data::TransformGrantsAppData,
        transform_grants_provider::CreateGrantParams,
    },
    transforms::transforms_app_data::TransformsAppData,
};

#[derive(Deserialize, IntoParams)]
pub struct TransformIdPath {
    pub transform_id: TransformId,
}

#[derive(Deserialize, IntoParams)]
pub struct GrantIdPath {
    pub transform_id: TransformId,
    pub grant_id: i64,
}

/// Exactly one of `grantee_user_id`/`grantee_workspace_id` must be set — the
/// same rule the DB CHECK constraint enforces, checked here first so a
/// malformed request gets a clear 400 instead of a raw constraint-violation
/// error from Postgres.
#[derive(Deserialize, ToSchema)]
pub struct CreateGrantInput {
    pub grantee_user_id: Option<domain::domain_user::UserId>,
    pub grantee_workspace_id: Option<i32>,
}

#[derive(Serialize, ToSchema)]
pub struct GrantDto {
    pub grant_id: i64,
    pub transform_id: TransformId,
    pub grantee_user_id: Option<domain::domain_user::UserId>,
    pub grantee_workspace_id: Option<i32>,
    pub granted_by: domain::domain_user::UserId,
}

impl From<DbTransformGrant> for GrantDto {
    fn from(value: DbTransformGrant) -> Self {
        Self {
            grant_id: value.grant_id,
            transform_id: value.transform_id,
            grantee_user_id: value.grantee_user_id,
            grantee_workspace_id: value.grantee_workspace_id,
            granted_by: value.granted_by,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct GrantsListResponse {
    pub grants: Vec<GrantDto>,
}

#[utoipa::path(post, path = "/transforms/{transform_id}/grants", tag = "transform_grants",
    params(TransformIdPath),
    request_body = CreateGrantInput,
    responses((status = 201, description = "Grant created", body = GrantDto), (status = 400), (status = 403)))]
#[post("/{transform_id}/grants")]
pub async fn create_grant(
    jwt: JwtContext,
    path: web::Path<TransformIdPath>,
    body: web::Json<CreateGrantInput>,
    app: web::Data<TransformsAppData>,
    grants: web::Data<TransformGrantsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_owner(&app, transform_id, &jwt).await {
        return resp;
    }

    let input = body.into_inner();
    let grantee_set_count =
        input.grantee_user_id.is_some() as u8 + input.grantee_workspace_id.is_some() as u8;
    if grantee_set_count != 1 {
        return HttpResponse::BadRequest()
            .body("exactly one of grantee_user_id or grantee_workspace_id must be set");
    }

    match grants
        .transform_grants_service
        .create_grant(CreateGrantParams {
            transform_id,
            grantee_user_id: input.grantee_user_id,
            grantee_workspace_id: input.grantee_workspace_id,
            granted_by: jwt.user_id,
        })
        .await
    {
        Ok(grant) => HttpResponse::Created().json(GrantDto::from(grant)),
        Err(e) => {
            error!(error = %e, "create grant failed");
            HttpResponse::InternalServerError().body("create grant failed")
        }
    }
}

#[utoipa::path(delete, path = "/transforms/{transform_id}/grants/{grant_id}", tag = "transform_grants",
    params(GrantIdPath),
    responses((status = 204), (status = 403), (status = 404)))]
#[delete("/{transform_id}/grants/{grant_id}")]
pub async fn delete_grant(
    jwt: JwtContext,
    path: web::Path<GrantIdPath>,
    app: web::Data<TransformsAppData>,
    grants: web::Data<TransformGrantsAppData>,
) -> HttpResponse {
    let path = path.into_inner();
    if let Err(resp) = require_owner(&app, path.transform_id, &jwt).await {
        return resp;
    }

    match grants
        .transform_grants_service
        .delete_grant(path.transform_id, path.grant_id)
        .await
    {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("grant not found"),
        Err(e) => {
            error!(error = %e, "delete grant failed");
            HttpResponse::InternalServerError().body("delete grant failed")
        }
    }
}

#[utoipa::path(get, path = "/transforms/{transform_id}/grants", tag = "transform_grants",
    params(TransformIdPath),
    responses((status = 200, body = GrantsListResponse), (status = 403)))]
#[get("/{transform_id}/grants")]
pub async fn list_grants(
    jwt: JwtContext,
    path: web::Path<TransformIdPath>,
    app: web::Data<TransformsAppData>,
    grants: web::Data<TransformGrantsAppData>,
) -> HttpResponse {
    let transform_id = path.into_inner().transform_id;
    if let Err(resp) = require_owner(&app, transform_id, &jwt).await {
        return resp;
    }

    match grants
        .transform_grants_service
        .list_grants(transform_id)
        .await
    {
        Ok(list) => HttpResponse::Ok().json(GrantsListResponse {
            grants: list.into_iter().map(GrantDto::from).collect(),
        }),
        Err(e) => {
            error!(error = %e, "list grants failed");
            HttpResponse::InternalServerError().body("list grants failed")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_grant)
        .service(delete_grant)
        .service(list_grants);
}

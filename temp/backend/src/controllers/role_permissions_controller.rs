use actix_web::{delete, get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

use crate::domain::db::db_permission::PermissionId;
use crate::domain::db::db_role::RoleId;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;
use crate::role_permissions::create_role_permission_params::CreateRolePermissionParams;
use crate::role_permissions::role_permissions_app_data::RolePermissionsAppData;
use crate::role_permissions::DbRolePermission;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RolePermissionPath {
    pub role_id: RoleId,
    pub permission_id: PermissionId,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RolePermissionQuery {
    pub role_id: Option<RoleId>,
    pub permission_id: Option<PermissionId>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RolePermissionResult {
    pub role_id: RoleId,
    pub permission_id: PermissionId,
}

impl From<DbRolePermission> for RolePermissionResult {
    fn from(value: DbRolePermission) -> Self {
        Self {
            role_id: value.role_id,
            permission_id: value.permission_id,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RolePermissionsResult {
    pub role_permissions: Vec<RolePermissionResult>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct CreateRolePermissionInput {
    pub role_id: RoleId,
    pub permission_id: PermissionId,
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CreateRolePermissionResult {
    pub role_permission: RolePermissionResult,
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct GetRolePermissionResult {
    pub role_permission: RolePermissionResult,
}

#[utoipa::path(
    post,
    path = "/role-permissions",
    tag = "RolePermissions",
    request_body = CreateRolePermissionInput,
    responses(
        (status = 201, description = "Role permission created", body = CreateRolePermissionResult),
        (status = 400, description = "Invalid input parameters"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("")]
pub async fn create_role_permission(
    perms: PermissionsContext,
    payload: web::Json<CreateRolePermissionInput>,
    app_state: web::Data<RolePermissionsAppData>,
) -> HttpResponse {
    if !perms.has("role_permissions:create") {
        return HttpResponse::Forbidden()
            .body("missing required permission: role_permissions:create");
    }
    info!("create role permission request received");
    let input = payload.into_inner();

    if input.role_id == 0 || input.permission_id == 0 {
        warn!("create role permission rejected: missing ids");
        return HttpResponse::BadRequest().body("role_id and permission_id are required");
    }

    let params = CreateRolePermissionParams {
        role_id: input.role_id,
        permission_id: input.permission_id,
    };

    match app_state
        .role_permissions_provider
        .create_role_permission(params)
        .await
    {
        Ok(result) => HttpResponse::Created().json(CreateRolePermissionResult {
            role_permission: RolePermissionResult::from(result),
        }),
        Err(e) => {
            error!(error = %e, "create role permission failed");
            HttpResponse::InternalServerError().body("failed to create role permission")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/role-permissions/{role_id}/{permission_id}",
    tag = "RolePermissions",
    params(
        ("role_id" = u64, Path, description = "Role id"),
        ("permission_id" = u64, Path, description = "Permission id"),
    ),
    responses(
        (status = 204, description = "Role permission deleted"),
        (status = 404, description = "Role permission not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[delete("/{role_id}/{permission_id}")]
pub async fn delete_role_permission(
    perms: PermissionsContext,
    path: web::Path<RolePermissionPath>,
    app_state: web::Data<RolePermissionsAppData>,
) -> HttpResponse {
    if !perms.has("role_permissions:delete") {
        return HttpResponse::Forbidden()
            .body("missing required permission: role_permissions:delete");
    }
    let path = path.into_inner();
    info!(
        role_id = path.role_id,
        permission_id = path.permission_id,
        "delete role permission request received"
    );

    match app_state
        .role_permissions_provider
        .delete_role_permission(path.role_id, path.permission_id)
        .await
    {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("role permission not found"),
        Err(e) => {
            error!(error = %e, "delete role permission failed");
            HttpResponse::InternalServerError().body("failed to delete role permission")
        }
    }
}

#[utoipa::path(
    get,
    path = "/role-permissions/{role_id}/{permission_id}",
    tag = "RolePermissions",
    params(
        ("role_id" = u64, Path, description = "Role id"),
        ("permission_id" = u64, Path, description = "Permission id"),
    ),
    responses(
        (status = 200, description = "Role permission retrieved", body = GetRolePermissionResult),
        (status = 404, description = "Role permission not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/{role_id}/{permission_id}")]
pub async fn get_role_permission(
    perms: PermissionsContext,
    path: web::Path<RolePermissionPath>,
    app_state: web::Data<RolePermissionsAppData>,
) -> HttpResponse {
    if !perms.has("role_permissions:read") {
        return HttpResponse::Forbidden()
            .body("missing required permission: role_permissions:read");
    }
    let path = path.into_inner();
    info!(
        role_id = path.role_id,
        permission_id = path.permission_id,
        "get role permission request received"
    );

    match app_state
        .role_permissions_provider
        .get_role_permission(path.role_id, path.permission_id)
        .await
    {
        Ok(Some(result)) => HttpResponse::Ok().json(GetRolePermissionResult {
            role_permission: RolePermissionResult::from(result),
        }),
        Ok(None) => HttpResponse::NotFound().body("role permission not found"),
        Err(e) => {
            error!(error = %e, "get role permission failed");
            HttpResponse::InternalServerError().body("failed to get role permission")
        }
    }
}

#[utoipa::path(
    get,
    path = "/role-permissions",
    tag = "RolePermissions",
    params(
        ("role_id" = Option<u64>, Query, description = "Filter by role id"),
        ("permission_id" = Option<u64>, Query, description = "Filter by permission id"),
    ),
    responses(
        (status = 200, description = "Role permissions retrieved", body = RolePermissionsResult),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
pub async fn list_role_permissions(
    perms: PermissionsContext,
    query: web::Query<RolePermissionQuery>,
    app_state: web::Data<RolePermissionsAppData>,
) -> HttpResponse {
    if !perms.has("role_permissions:read") {
        return HttpResponse::Forbidden()
            .body("missing required permission: role_permissions:read");
    }
    let query = query.into_inner();
    info!(role_id = ?query.role_id, permission_id = ?query.permission_id, "list role permissions request received");

    match app_state
        .role_permissions_provider
        .list_role_permissions(query.role_id, query.permission_id)
        .await
    {
        Ok(result) => HttpResponse::Ok().json(RolePermissionsResult {
            role_permissions: result.into_iter().map(RolePermissionResult::from).collect(),
        }),
        Err(e) => {
            error!(error = %e, "list role permissions failed");
            HttpResponse::InternalServerError().body("failed to fetch role permissions")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_role_permission)
        .service(delete_role_permission)
        .service(get_role_permission)
        .service(list_role_permissions);
}

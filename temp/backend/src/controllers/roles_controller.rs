use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_role::RoleId;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;
use crate::roles::create_role_params::CreateRoleParams;
use crate::roles::roles_app_data::RolesAppData;
use crate::roles::update_role_params::UpdateRoleParams;
use crate::roles::DbRole;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

#[derive(ToSchema, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CreateRoleResult {
    pub role: RoleResult,
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct UpdateRoleResult {
    pub role: RoleResult,
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct GetRoleResult {
    pub role: RoleResult,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateRoleInput {
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct CreateRoleInput {
    pub org_id: OrganizationId,
    pub name: String,
    pub is_system: Option<bool>,
}

#[utoipa::path(
    post,
    path = "/roles",
    tag = "Roles",
    request_body = CreateRoleInput,
    responses(
        (status = 201, description = "Role created", body = CreateRoleResult),
        (status = 400, description = "Invalid input parameters"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("")]
pub async fn create_role(
    perms: PermissionsContext,
    payload: web::Json<CreateRoleInput>,
    app_state: web::Data<RolesAppData>,
) -> HttpResponse {
    if !perms.has("roles:create") {
        return HttpResponse::Forbidden().body("missing required permission: roles:create");
    }
    info!("create role request received");
    let input = payload.into_inner();

    if input.org_id == 0 {
        warn!("create role rejected: missing org_id");
        return HttpResponse::BadRequest().body("org_id is required");
    }
    if input.name.trim().is_empty() {
        warn!("create role rejected: missing name");
        return HttpResponse::BadRequest().body("name is required");
    }

    let params = CreateRoleParams {
        org_id: input.org_id,
        name: input.name,
        is_system: input.is_system,
    };

    match app_state.roles_provider.create_role(params).await {
        Ok(result) => HttpResponse::Created().json(CreateRoleResult {
            role: RoleResult::from(result),
        }),
        Err(e) => {
            error!(error = %e, "create role failed");
            HttpResponse::InternalServerError().body("failed to create role")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RolePath {
    pub role_id: RoleId,
}

#[utoipa::path(
    put,
    path = "/roles/{role_id}",
    tag = "Roles",
    request_body = UpdateRoleInput,
    params(
        ("role_id" = RoleId, Path, description = "Role id"),
    ),
    responses(
        (status = 200, description = "Role updated", body = UpdateRoleResult),
        (status = 404, description = "Role not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[put("/{role_id}")]
pub async fn update_role(
    perms: PermissionsContext,
    path: web::Path<RolePath>,
    payload: web::Json<UpdateRoleInput>,
    app_state: web::Data<RolesAppData>,
) -> HttpResponse {
    if !perms.has("roles:update") {
        return HttpResponse::Forbidden().body("missing required permission: roles:update");
    }
    let path = path.into_inner();
    info!(role_id = path.role_id, "update role request received");
    let input = payload.into_inner();

    if input.name.trim().is_empty() {
        warn!("update role rejected: missing name");
        return HttpResponse::BadRequest().body("name is required");
    }

    let params = UpdateRoleParams { name: input.name };

    match app_state
        .roles_provider
        .update_role(path.role_id, params)
        .await
    {
        Ok(Some(result)) => HttpResponse::Ok().json(UpdateRoleResult {
            role: RoleResult::from(result),
        }),
        Ok(None) => HttpResponse::NotFound().body("role not found"),
        Err(e) => {
            error!(error = %e, "update role failed");
            HttpResponse::InternalServerError().body("failed to update role")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/roles/{role_id}",
    tag = "Roles",
    params(
        ("role_id" = RoleId, Path, description = "Role id"),
    ),
    responses(
        (status = 204, description = "Role deleted"),
        (status = 404, description = "Role not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[delete("/{role_id}")]
pub async fn delete_role(
    perms: PermissionsContext,
    path: web::Path<RolePath>,
    app_state: web::Data<RolesAppData>,
) -> HttpResponse {
    if !perms.has("roles:delete") {
        return HttpResponse::Forbidden().body("missing required permission: roles:delete");
    }
    let path = path.into_inner();
    info!(role_id = path.role_id, "delete role request received");

    match app_state.roles_provider.delete_role(path.role_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("role not found"),
        Err(e) => {
            error!(error = %e, "delete role failed");
            HttpResponse::InternalServerError().body("failed to delete role")
        }
    }
}

#[utoipa::path(
    get,
    path = "/roles/{role_id}",
    tag = "Roles",
    params(
        ("role_id" = RoleId, Path, description = "Role id"),
    ),
    responses(
        (status = 200, description = "Role retrieved", body = GetRoleResult),
        (status = 404, description = "Role not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/{role_id}")]
pub async fn get_role(
    perms: PermissionsContext,
    path: web::Path<RolePath>,
    app_state: web::Data<RolesAppData>,
) -> HttpResponse {
    if !perms.has("roles:read") {
        return HttpResponse::Forbidden().body("missing required permission: roles:read");
    }
    let path = path.into_inner();
    info!(role_id = path.role_id, "get role request received");

    match app_state.roles_provider.get_role(path.role_id).await {
        Ok(Some(result)) => HttpResponse::Ok().json(GetRoleResult {
            role: RoleResult::from(result),
        }),
        Ok(None) => HttpResponse::NotFound().body("role not found"),
        Err(e) => {
            error!(error = %e, "get role failed");
            HttpResponse::InternalServerError().body("failed to get role")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RoleQuery {
    pub org_id: Option<OrganizationId>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RolesResult {
    pub roles: Vec<RoleResult>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct RoleResult {
    pub id: RoleId,
    pub org_id: Option<OrganizationId>,
    pub name: String,
    pub is_system: bool,
}

impl From<DbRole> for RoleResult {
    fn from(value: DbRole) -> Self {
        Self {
            id: value.id,
            org_id: value.org_id,
            name: value.name,
            is_system: value.is_system,
        }
    }
}

#[utoipa::path(
    get,
    path = "/roles",
    tag = "Roles",
    params(
        ("org_id" = Option<OrganizationId>, Query, description = "Filter by org id"),
    ),
    responses(
        (status = 200, description = "Roles retrieved", body = RolesResult),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
pub async fn list_roles(
    perms: PermissionsContext,
    query: web::Query<RoleQuery>,
    app_state: web::Data<RolesAppData>,
) -> HttpResponse {
    if !perms.has("roles:read") {
        return HttpResponse::Forbidden().body("missing required permission: roles:read");
    }
    let query = query.into_inner();

    match app_state.roles_provider.list_roles(query.org_id).await {
        Ok(result) => HttpResponse::Ok().json(RolesResult {
            roles: result.into_iter().map(RoleResult::from).collect(),
        }),
        Err(e) => {
            error!(error = %e, "list roles failed");
            HttpResponse::InternalServerError().body("failed to fetch roles")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_role)
        .service(update_role)
        .service(delete_role)
        .service(get_role)
        .service(list_roles);
}

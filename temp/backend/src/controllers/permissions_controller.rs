use crate::domain::db::db_permission::PermissionId;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;
use crate::permissions::create_permission_params::CreatePermissionParams;
use crate::permissions::permissions_app_data::PermissionsAppData;
use crate::permissions::update_permission_params::UpdatePermissionParams;
use crate::permissions::DbPermission;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct PermissionPath {
    pub permission_id: PermissionId,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct PermissionResult {
    pub id: PermissionId,
    pub key: String,
    pub description: Option<String>,
}

impl From<DbPermission> for PermissionResult {
    fn from(value: DbPermission) -> Self {
        Self {
            id: value.id,
            key: value.key,
            description: value.description,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct PermissionsResult {
    pub permissions: Vec<PermissionResult>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct CreatePermissionInput {
    pub key: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdatePermissionInput {
    pub description: Option<String>,
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CreatePermissionResult {
    pub permission: PermissionResult,
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct UpdatePermissionResult {
    pub permission: PermissionResult,
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct GetPermissionResult {
    pub permission: PermissionResult,
}

#[utoipa::path(
    post,
    path = "/permissions",
    tag = "Permissions",
    request_body = CreatePermissionInput,
    responses(
        (status = 201, description = "Permission created", body = CreatePermissionResult),
        (status = 400, description = "Invalid input parameters"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("")]
pub async fn create_permission(
    perms: PermissionsContext,
    payload: web::Json<CreatePermissionInput>,
    app_state: web::Data<PermissionsAppData>,
) -> HttpResponse {
    if !perms.has("permissions:create") {
        return HttpResponse::Forbidden().body("missing required permission: permissions:create");
    }
    info!("create permission request received");
    let input = payload.into_inner();

    if input.key.trim().is_empty() {
        warn!("create permission rejected: missing key");
        return HttpResponse::BadRequest().body("key is required");
    }

    let params = CreatePermissionParams {
        key: input.key,
        description: input.description,
    };

    match app_state
        .permissions_provider
        .create_permission(params)
        .await
    {
        Ok(result) => HttpResponse::Created().json(CreatePermissionResult {
            permission: PermissionResult::from(result),
        }),
        Err(e) => {
            error!(error = %e, "create permission failed");
            HttpResponse::InternalServerError().body("failed to create permission")
        }
    }
}

#[utoipa::path(
    put,
    path = "/permissions/{permission_id}",
    tag = "Permissions",
    request_body = UpdatePermissionInput,
    params(
        ("permission_id" = u64, Path, description = "Permission id"),
    ),
    responses(
        (status = 200, description = "Permission updated", body = UpdatePermissionResult),
        (status = 404, description = "Permission not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[put("/{permission_id}")]
pub async fn update_permission(
    perms: PermissionsContext,
    path: web::Path<PermissionPath>,
    payload: web::Json<UpdatePermissionInput>,
    app_state: web::Data<PermissionsAppData>,
) -> HttpResponse {
    if !perms.has("permissions:update") {
        return HttpResponse::Forbidden().body("missing required permission: permissions:update");
    }
    let path = path.into_inner();
    info!(
        permission_id = path.permission_id,
        "update permission request received"
    );
    let input = payload.into_inner();

    let params = UpdatePermissionParams {
        description: input.description,
    };

    match app_state
        .permissions_provider
        .update_permission(path.permission_id, params)
        .await
    {
        Ok(Some(result)) => HttpResponse::Ok().json(UpdatePermissionResult {
            permission: PermissionResult::from(result),
        }),
        Ok(None) => HttpResponse::NotFound().body("permission not found"),
        Err(e) => {
            error!(error = %e, "update permission failed");
            HttpResponse::InternalServerError().body("failed to update permission")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/permissions/{permission_id}",
    tag = "Permissions",
    params(
        ("permission_id" = u64, Path, description = "Permission id"),
    ),
    responses(
        (status = 204, description = "Permission deleted"),
        (status = 404, description = "Permission not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[delete("/{permission_id}")]
pub async fn delete_permission(
    perms: PermissionsContext,
    path: web::Path<PermissionPath>,
    app_state: web::Data<PermissionsAppData>,
) -> HttpResponse {
    if !perms.has("permissions:delete") {
        return HttpResponse::Forbidden().body("missing required permission: permissions:delete");
    }
    let path = path.into_inner();
    info!(
        permission_id = path.permission_id,
        "delete permission request received"
    );

    match app_state
        .permissions_provider
        .delete_permission(path.permission_id)
        .await
    {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("permission not found"),
        Err(e) => {
            error!(error = %e, "delete permission failed");
            HttpResponse::InternalServerError().body("failed to delete permission")
        }
    }
}

#[utoipa::path(
    get,
    path = "/permissions/{permission_id}",
    tag = "Permissions",
    params(
        ("permission_id" = u64, Path, description = "Permission id"),
    ),
    responses(
        (status = 200, description = "Permission retrieved", body = GetPermissionResult),
        (status = 404, description = "Permission not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/{permission_id}")]
pub async fn get_permission(
    perms: PermissionsContext,
    path: web::Path<PermissionPath>,
    app_state: web::Data<PermissionsAppData>,
) -> HttpResponse {
    if !perms.has("permissions:read") {
        return HttpResponse::Forbidden().body("missing required permission: permissions:read");
    }
    let path = path.into_inner();
    info!(
        permission_id = path.permission_id,
        "get permission request received"
    );

    match app_state
        .permissions_provider
        .get_permission(path.permission_id)
        .await
    {
        Ok(Some(result)) => HttpResponse::Ok().json(GetPermissionResult {
            permission: PermissionResult::from(result),
        }),
        Ok(None) => HttpResponse::NotFound().body("permission not found"),
        Err(e) => {
            error!(error = %e, "get permission failed");
            HttpResponse::InternalServerError().body("failed to get permission")
        }
    }
}

#[utoipa::path(
    get,
    path = "/permissions",
    tag = "Permissions",
    responses(
        (status = 200, description = "Permissions retrieved", body = PermissionsResult),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
pub async fn list_permissions(
    perms: PermissionsContext,
    app_state: web::Data<PermissionsAppData>,
) -> HttpResponse {
    if !perms.has("permissions:read") {
        return HttpResponse::Forbidden().body("missing required permission: permissions:read");
    }
    info!("list permissions request received");

    match app_state.permissions_provider.list_permissions().await {
        Ok(result) => HttpResponse::Ok().json(PermissionsResult {
            permissions: result.into_iter().map(PermissionResult::from).collect(),
        }),
        Err(e) => {
            error!(error = %e, "list permissions failed");
            HttpResponse::InternalServerError().body("failed to fetch permissions")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_permission)
        .service(update_permission)
        .service(delete_permission)
        .service(get_permission)
        .service(list_permissions);
}

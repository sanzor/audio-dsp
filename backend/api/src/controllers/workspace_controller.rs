use actix_web::{delete, get, patch, post, web, HttpResponse};
use domain::workspace_role::WorkspaceRole;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use utoipa::ToSchema;

use crate::{
    auth::{auth_app_data::AuthAppData, invite_user_params::InviteUserParams, service_error::ServiceError},
    middlewares::{jwt::jwt_context::JwtContext, membership::membership_context::RoleContext},
    transforms::transforms_app_data::TransformsAppData,
    workspaces::workspaces_app_data::WorkspacesAppData,
};

// ── list members ───────────────────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
pub struct MemberOutput {
    pub user_id: i64,
    pub role: WorkspaceRole,
}

#[utoipa::path(get, path = "/v1/workspaces/{workspace_id}/members", tag = "Workspaces",
    responses((status = 200, body = Vec<MemberOutput>), (status = 403)))]
#[get("/{workspace_id}/members")]
pub async fn list_members(
    jwt: JwtContext,
    role: RoleContext,
    path: web::Path<i32>,
    app: web::Data<WorkspacesAppData>,
) -> HttpResponse {
    let workspace_id = path.into_inner();

    if !role.can_view() {
        return HttpResponse::Forbidden().body("access denied");
    }

    info!(user_id = %jwt.user_id, workspace_id = %workspace_id, "list members");

    match app.memberships_service.list_memberships(Some(workspace_id), None).await {
        Ok(members) => HttpResponse::Ok().json(
            members
                .into_iter()
                .map(|m| MemberOutput { user_id: m.user_id, role: m.role })
                .collect::<Vec<_>>(),
        ),
        Err(e) => {
            error!(error = %e, "list members failed");
            HttpResponse::InternalServerError().body("list members failed")
        }
    }
}

// ── invite member ──────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct InviteInput {
    pub email: String,
    pub role: WorkspaceRole,
}

#[derive(Serialize, ToSchema)]
pub struct InviteOutput {
    pub invitee_email: String,
}

#[utoipa::path(post, path = "/v1/workspaces/{workspace_id}/invite", tag = "Workspaces",
    request_body = InviteInput,
    responses((status = 200, body = InviteOutput), (status = 403)))]
#[post("/{workspace_id}/invite")]
pub async fn invite_member(
    jwt: JwtContext,
    role: RoleContext,
    path: web::Path<i32>,
    payload: web::Json<InviteInput>,
    auth: web::Data<AuthAppData>,
) -> HttpResponse {
    let workspace_id = path.into_inner();
    let input = payload.into_inner();

    if input.email.trim().is_empty() {
        return HttpResponse::BadRequest().body("email required");
    }
    if !role.is_owner() {
        return HttpResponse::Forbidden().body("owner role required");
    }

    info!(user_id = %jwt.user_id, workspace_id = %workspace_id, invitee = %input.email, "invite member");

    match auth
        .auth_provider
        .invite_user(InviteUserParams { email: input.email, workspace_id, role: input.role })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(InviteOutput { invitee_email: r.invitee_email }),
        Err(ServiceError::NotFound) => HttpResponse::NotFound().body("not found"),
        Err(e) => {
            error!(error = %e, "invite failed");
            HttpResponse::InternalServerError().body("invite failed")
        }
    }
}

// ── delete workspace ───────────────────────────────────────────────────────────

#[utoipa::path(delete, path = "/v1/workspaces/{workspace_id}", tag = "Workspaces",
    responses((status = 204), (status = 403), (status = 404)))]
#[delete("/{workspace_id}")]
pub async fn delete_workspace(
    jwt: JwtContext,
    role: RoleContext,
    path: web::Path<i32>,
    app: web::Data<WorkspacesAppData>,
) -> HttpResponse {
    let workspace_id = path.into_inner();

    if !role.is_owner() {
        return HttpResponse::Forbidden().body("owner role required");
    }

    info!(user_id = %jwt.user_id, workspace_id = %workspace_id, "delete workspace");

    match app.workspaces_service.delete_workspace(&workspace_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("workspace not found"),
        Err(e) => {
            error!(error = %e, "delete workspace failed");
            HttpResponse::InternalServerError().body("delete workspace failed")
        }
    }
}

// ── remove member ──────────────────────────────────────────────────────────────

#[utoipa::path(delete, path = "/v1/workspaces/{workspace_id}/members/{user_id}", tag = "Workspaces",
    responses((status = 204), (status = 403), (status = 404)))]
#[delete("/{workspace_id}/members/{user_id}")]
pub async fn remove_member(
    jwt: JwtContext,
    role: RoleContext,
    path: web::Path<(i32, i32)>,
    app: web::Data<WorkspacesAppData>,
) -> HttpResponse {
    let (workspace_id, target_user_id) = path.into_inner();

    if !role.is_owner() {
        return HttpResponse::Forbidden().body("owner role required");
    }
    if target_user_id == jwt.user_id {
        return HttpResponse::BadRequest().body("cannot remove yourself as owner");
    }

    info!(user_id = %jwt.user_id, workspace_id = %workspace_id, target = %target_user_id, "remove member");

    match app.memberships_service.delete_membership(workspace_id, target_user_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("member not found"),
        Err(e) => {
            error!(error = %e, "remove member failed");
            HttpResponse::InternalServerError().body("remove member failed")
        }
    }
}

// ── change role ────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct ChangeRoleInput {
    pub role: WorkspaceRole,
}

#[utoipa::path(patch, path = "/v1/workspaces/{workspace_id}/members/{user_id}/role", tag = "Workspaces",
    request_body = ChangeRoleInput,
    responses((status = 200, body = MemberOutput), (status = 403), (status = 404)))]
#[patch("/{workspace_id}/members/{user_id}/role")]
pub async fn change_role(
    jwt: JwtContext,
    role: RoleContext,
    path: web::Path<(i32, i32)>,
    payload: web::Json<ChangeRoleInput>,
    app: web::Data<WorkspacesAppData>,
) -> HttpResponse {
    let (workspace_id, target_user_id) = path.into_inner();
    let input = payload.into_inner();

    if !role.is_owner() {
        return HttpResponse::Forbidden().body("owner role required");
    }
    if target_user_id == jwt.user_id && input.role != WorkspaceRole::Owner {
        return HttpResponse::BadRequest().body("cannot demote yourself");
    }

    info!(user_id = %jwt.user_id, workspace_id = %workspace_id, target = %target_user_id, "change role");

    match app.memberships_service.update_role(workspace_id, target_user_id, input.role).await {
        Ok(Some(m)) => HttpResponse::Ok().json(MemberOutput { user_id: m.user_id, role: m.role }),
        Ok(None) => HttpResponse::NotFound().body("member not found"),
        Err(e) => {
            error!(error = %e, "change role failed");
            HttpResponse::InternalServerError().body("change role failed")
        }
    }
}

// ── list workspace transforms ─────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
pub struct WorkspaceTransformSummaryDto {
    pub transform_id: domain::db::db_transform::TransformId,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    /// "primitive" | "composite".
    pub kind: String,
}

impl From<domain::db::DbTransform> for WorkspaceTransformSummaryDto {
    fn from(value: domain::db::DbTransform) -> Self {
        Self {
            transform_id: value.transform_id,
            name: value.name,
            description: value.description,
            icon: value.icon,
            kind: value.kind,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct WorkspaceTransformsResponse {
    pub transforms: Vec<WorkspaceTransformSummaryDto>,
}

/// Transforms usable inside this workspace: owned by the caller, granted
/// directly to the caller, or granted to this workspace. Workspace
/// membership is already verified by `membership_middleware` before this
/// handler runs.
#[utoipa::path(get, path = "/v1/workspaces/{workspace_id}/transforms", tag = "Workspaces",
    responses((status = 200, body = WorkspaceTransformsResponse), (status = 403)))]
#[get("/{workspace_id}/transforms")]
pub async fn list_workspace_transforms(
    jwt: JwtContext,
    _role: RoleContext,
    path: web::Path<i32>,
    transforms_app: web::Data<TransformsAppData>,
) -> HttpResponse {
    let workspace_id = path.into_inner();

    match transforms_app
        .transforms_service
        .get_transforms_for_workspace_and_user(domain::domain_user::UserId::from(jwt.user_id), workspace_id)
        .await
    {
        Ok(transforms) => HttpResponse::Ok().json(WorkspaceTransformsResponse {
            transforms: transforms.into_iter().map(WorkspaceTransformSummaryDto::from).collect(),
        }),
        Err(e) => {
            error!(error = %e, "list workspace transforms failed");
            HttpResponse::InternalServerError().body("list workspace transforms failed")
        }
    }
}

// ── init ───────────────────────────────────────────────────────────────────────

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_members)
        .service(invite_member)
        .service(delete_workspace)
        .service(remove_member)
        .service(change_role)
        .service(list_workspace_transforms);
}

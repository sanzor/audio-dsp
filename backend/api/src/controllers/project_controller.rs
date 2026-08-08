use actix_web::{delete, get, patch, post, web, HttpResponse};
use domain::project_role::ProjectRole;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use utoipa::ToSchema;

use crate::{
    auth::{auth_app_data::AuthAppData, invite_user_params::InviteUserParams, service_error::ServiceError},
    middlewares::{jwt::jwt_context::JwtContext, membership::membership_context::RoleContext},
    projects::project_app_data::ProjectAppData,
};

// ── list members ───────────────────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
pub struct MemberOutput {
    pub user_id: i32,
    pub role: ProjectRole,
}

#[utoipa::path(get, path = "/v1/projects/{project_id}/members", tag = "Projects",
    responses((status = 200, body = Vec<MemberOutput>), (status = 403)))]
#[get("/{project_id}/members")]
pub async fn list_members(
    jwt: JwtContext,
    role: RoleContext,
    path: web::Path<i32>,
    app: web::Data<ProjectAppData>,
) -> HttpResponse {
    let project_id = path.into_inner();

    if !role.can_view() {
        return HttpResponse::Forbidden().body("access denied");
    }

    info!(user_id = %jwt.user_id, project_id = %project_id, "list members");

    match app.memberships_service.list_memberships(Some(project_id), None).await {
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
    pub role: ProjectRole,
}

#[derive(Serialize, ToSchema)]
pub struct InviteOutput {
    pub invitee_email: String,
}

#[utoipa::path(post, path = "/v1/projects/{project_id}/invite", tag = "Projects",
    request_body = InviteInput,
    responses((status = 200, body = InviteOutput), (status = 403)))]
#[post("/{project_id}/invite")]
pub async fn invite_member(
    jwt: JwtContext,
    role: RoleContext,
    path: web::Path<i32>,
    payload: web::Json<InviteInput>,
    auth: web::Data<AuthAppData>,
) -> HttpResponse {
    let project_id = path.into_inner();
    let input = payload.into_inner();

    if input.email.trim().is_empty() {
        return HttpResponse::BadRequest().body("email required");
    }
    if !role.is_owner() {
        return HttpResponse::Forbidden().body("owner role required");
    }

    info!(user_id = %jwt.user_id, project_id = %project_id, invitee = %input.email, "invite member");

    match auth
        .auth_provider
        .invite_user(InviteUserParams { email: input.email, project_id, role: input.role })
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

// ── delete project ─────────────────────────────────────────────────────────────

#[utoipa::path(delete, path = "/v1/projects/{project_id}", tag = "Projects",
    responses((status = 204), (status = 403), (status = 404)))]
#[delete("/{project_id}")]
pub async fn delete_project(
    jwt: JwtContext,
    role: RoleContext,
    path: web::Path<i32>,
    app: web::Data<ProjectAppData>,
) -> HttpResponse {
    let project_id = path.into_inner();

    if !role.is_owner() {
        return HttpResponse::Forbidden().body("owner role required");
    }

    info!(user_id = %jwt.user_id, project_id = %project_id, "delete project");

    match app.projects_service.delete_project(&project_id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("project not found"),
        Err(e) => {
            error!(error = %e, "delete project failed");
            HttpResponse::InternalServerError().body("delete project failed")
        }
    }
}

// ── remove member ──────────────────────────────────────────────────────────────

#[utoipa::path(delete, path = "/v1/projects/{project_id}/members/{user_id}", tag = "Projects",
    responses((status = 204), (status = 403), (status = 404)))]
#[delete("/{project_id}/members/{user_id}")]
pub async fn remove_member(
    jwt: JwtContext,
    role: RoleContext,
    path: web::Path<(i32, i32)>,
    app: web::Data<ProjectAppData>,
) -> HttpResponse {
    let (project_id, target_user_id) = path.into_inner();

    if !role.is_owner() {
        return HttpResponse::Forbidden().body("owner role required");
    }
    if target_user_id == jwt.user_id {
        return HttpResponse::BadRequest().body("cannot remove yourself as owner");
    }

    info!(user_id = %jwt.user_id, project_id = %project_id, target = %target_user_id, "remove member");

    match app.memberships_service.delete_membership(project_id, target_user_id).await {
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
    pub role: ProjectRole,
}

#[utoipa::path(patch, path = "/v1/projects/{project_id}/members/{user_id}/role", tag = "Projects",
    request_body = ChangeRoleInput,
    responses((status = 200, body = MemberOutput), (status = 403), (status = 404)))]
#[patch("/{project_id}/members/{user_id}/role")]
pub async fn change_role(
    jwt: JwtContext,
    role: RoleContext,
    path: web::Path<(i32, i32)>,
    payload: web::Json<ChangeRoleInput>,
    app: web::Data<ProjectAppData>,
) -> HttpResponse {
    let (project_id, target_user_id) = path.into_inner();
    let input = payload.into_inner();

    if !role.is_owner() {
        return HttpResponse::Forbidden().body("owner role required");
    }
    if target_user_id == jwt.user_id && input.role != ProjectRole::Owner {
        return HttpResponse::BadRequest().body("cannot demote yourself");
    }

    info!(user_id = %jwt.user_id, project_id = %project_id, target = %target_user_id, "change role");

    match app.memberships_service.update_role(project_id, target_user_id, input.role).await {
        Ok(Some(m)) => HttpResponse::Ok().json(MemberOutput { user_id: m.user_id, role: m.role }),
        Ok(None) => HttpResponse::NotFound().body("member not found"),
        Err(e) => {
            error!(error = %e, "change role failed");
            HttpResponse::InternalServerError().body("change role failed")
        }
    }
}

// ── init ───────────────────────────────────────────────────────────────────────

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_members)
        .service(invite_member)
        .service(delete_project)
        .service(remove_member)
        .service(change_role);
}

use actix_web::{get, post, web, HttpResponse};
use domain::project_role::ProjectRole;
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::ToSchema;

use crate::{
    app_data::AppData,
    auth::{
        accept_invite_params::AcceptInviteParams,
        auth_app_data::AuthAppData,
        service_error::ServiceError,
    },
    me::me_app_data::MeAppData,
    me::MeBootstrapResult,
    memberships::memberships_provider::CreateMembershipParams,
    middlewares::jwt::jwt_context::JwtContext,
    projects::projects_provider::CreateProjectParams,
};

#[utoipa::path(get, path = "/v1/me/bootstrap", tag = "Me",
    responses((status = 200, body = MeBootstrapResult), (status = 401), (status = 500)))]
#[get("/bootstrap")]
pub async fn bootstrap(
    auth: JwtContext,
    app_state: web::Data<MeAppData>,
) -> HttpResponse {
    match app_state.me_data_provider.get_bootstrap_data(&auth.user_id).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            error!(user_id = %auth.user_id, error = %e, "bootstrap failed");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// ── create project ─────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateProjectInput {
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct ProjectOutput {
    pub project_id: String,
    pub name: String,
}

#[utoipa::path(post, path = "/v1/me/projects", tag = "Me",
    request_body = CreateProjectInput,
    responses((status = 201, body = ProjectOutput), (status = 401)))]
#[post("/projects")]
pub async fn create_project(
    auth: JwtContext,
    payload: web::Json<CreateProjectInput>,
    app: web::Data<AppData>,
) -> HttpResponse {
    let input = payload.into_inner();
    if input.name.trim().is_empty() {
        return HttpResponse::BadRequest().body("name required");
    }

    let project = match app
        .projects_service
        .create_project(CreateProjectParams { name: input.name, created_by: auth.user_id.clone() })
        .await
    {
        Ok(p) => p,
        Err(e) => {
            error!(error = %e, "create project failed");
            return HttpResponse::InternalServerError().body("create project failed");
        }
    };

    if let Err(e) = app
        .memberships_service
        .create_membership(CreateMembershipParams {
            project_id: project.project_id.clone(),
            user_id: auth.user_id.clone(),
            role: ProjectRole::Owner,
        })
        .await
    {
        error!(error = %e, "failed to add owner membership");
        return HttpResponse::InternalServerError().body("failed to set owner");
    }

    HttpResponse::Created().json(ProjectOutput { project_id: project.project_id, name: project.name })
}

// ── accept invite ──────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct AcceptInviteInput {
    pub invite_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct AcceptInviteOutput {
    pub project_id: String,
    pub role: ProjectRole,
}

#[utoipa::path(post, path = "/v1/me/accept-invite", tag = "Me",
    request_body = AcceptInviteInput,
    responses((status = 200, body = AcceptInviteOutput), (status = 400), (status = 403)))]
#[post("/accept-invite")]
pub async fn accept_invite(
    auth: JwtContext,
    payload: web::Json<AcceptInviteInput>,
    auth_data: web::Data<AuthAppData>,
) -> HttpResponse {
    let input = payload.into_inner();
    if input.invite_token.trim().is_empty() {
        return HttpResponse::BadRequest().body("invite_token required");
    }

    match auth_data
        .auth_provider
        .accept_invite(AcceptInviteParams {
            invite_token: input.invite_token,
            caller_user_id: auth.user_id.clone(),
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(AcceptInviteOutput { project_id: r.project_id, role: r.role }),
        Err(ServiceError::NotFound) => HttpResponse::NotFound().body("user not found"),
        Err(ServiceError::Forbidden) => HttpResponse::Forbidden().body("this invite is not for you"),
        Err(e) => {
            error!(error = %e, "accept-invite failed");
            HttpResponse::InternalServerError().body("accept-invite failed")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(bootstrap)
        .service(create_project)
        .service(accept_invite);
}

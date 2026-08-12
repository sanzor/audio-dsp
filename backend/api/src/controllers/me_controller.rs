use actix_web::{get, post, web, HttpResponse};
use domain::workspace_role::WorkspaceRole;
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
    workspaces::workspaces_provider::CreateWorkspaceParams,
};

#[utoipa::path(get, path = "/v1/me/bootstrap", tag = "Me",
    responses((status = 200, body = MeBootstrapResult), (status = 401), (status = 500)))]
#[get("/bootstrap")]
pub async fn bootstrap(
    auth: JwtContext,
    app_state: web::Data<MeAppData>,
) -> HttpResponse {
    match app_state.me_data_provider.get_bootstrap_data(auth.user_id).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            error!(user_id = %auth.user_id, error = %e, "bootstrap failed");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// ── create workspace ───────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateWorkspaceInput {
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct WorkspaceOutput {
    pub workspace_id: i32,
    pub name: String,
}

#[utoipa::path(post, path = "/v1/me/workspaces", tag = "Me",
    request_body = CreateWorkspaceInput,
    responses((status = 201, body = WorkspaceOutput), (status = 401)))]
#[post("/workspaces")]
pub async fn create_workspace(
    auth: JwtContext,
    payload: web::Json<CreateWorkspaceInput>,
    app: web::Data<AppData>,
) -> HttpResponse {
    let input = payload.into_inner();
    if input.name.trim().is_empty() {
        return HttpResponse::BadRequest().body("name required");
    }

    let workspace = match app
        .workspaces_service
        .create_workspace(CreateWorkspaceParams { name: input.name, created_by: auth.user_id })
        .await
    {
        Ok(w) => w,
        Err(e) => {
            error!(error = %e, "create workspace failed");
            return HttpResponse::InternalServerError().body("create workspace failed");
        }
    };

    if let Err(e) = app
        .memberships_service
        .create_membership(CreateMembershipParams {
            workspace_id: workspace.workspace_id,
            user_id: auth.user_id,
            role: WorkspaceRole::Owner,
        })
        .await
    {
        error!(error = %e, "failed to add owner membership");
        return HttpResponse::InternalServerError().body("failed to set owner");
    }

    HttpResponse::Created().json(WorkspaceOutput { workspace_id: workspace.workspace_id, name: workspace.name })
}

// ── accept invite ──────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct AcceptInviteInput {
    pub invite_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct AcceptInviteOutput {
    pub workspace_id: i32,
    pub role: WorkspaceRole,
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
            caller_user_id: auth.user_id,
        })
        .await
    {
        Ok(r) => HttpResponse::Ok().json(AcceptInviteOutput { workspace_id: r.workspace_id, role: r.role }),
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
        .service(create_workspace)
        .service(accept_invite);
}

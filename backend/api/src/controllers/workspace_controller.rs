use actix_web::{get, web, HttpResponse};
use tracing::{error, info};

use crate::{
    middlewares::{jwt::jwt_context::JwtContext, membership::membership_context::RoleContext},
    workspace::workspace_app_data::WorkspaceAppData,
};

#[utoipa::path(get, path = "/v1/projects/{project_id}/workspace", tag = "Workspace",
    responses((status = 200), (status = 403), (status = 500)))]
#[get("/workspace")]
pub async fn get_project_workspace(
    jwt: JwtContext,
    role: RoleContext,
    path: web::Path<i32>,
    app: web::Data<WorkspaceAppData>,
) -> HttpResponse {
    let project_id = path.into_inner();

    if !role.can_view() {
        return HttpResponse::Forbidden().body("access denied");
    }

    info!(user_id = %jwt.user_id, project_id = %project_id, "get project workspace");

    match app.workspace_service.get_project_workspace(project_id).await {
        Ok(tracks) => HttpResponse::Ok().json(tracks),
        Err(e) => {
            error!(error = %e, project_id = %project_id, "get project workspace failed");
            HttpResponse::InternalServerError().body("failed to load workspace")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_project_workspace);
}

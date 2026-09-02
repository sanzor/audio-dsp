use actix_web::{get, web, HttpResponse};
use tracing::{error, info};

use crate::{
    middlewares::{jwt::jwt_context::JwtContext, membership::membership_context::RoleContext},
    workspace::workspace_app_data::WorkspaceAppData,
};

#[utoipa::path(get, path = "/v1/workspaces/{workspace_id}/tracks", tag = "Workspace",
    responses((status = 200), (status = 403), (status = 500)))]
#[get("/tracks")]
pub async fn get_workspace_tracks(
    jwt: JwtContext,
    role: RoleContext,
    path: web::Path<i32>,
    app: web::Data<WorkspaceAppData>,
) -> HttpResponse {
    let workspace_id = path.into_inner();

    if !role.can_view() {
        return HttpResponse::Forbidden().body("access denied");
    }

    info!(user_id = %jwt.user_id, workspace_id = %workspace_id, "get workspace tracks");

    match app
        .workspace_service
        .get_workspace_tracks(workspace_id)
        .await
    {
        Ok(tracks) => HttpResponse::Ok().json(tracks),
        Err(e) => {
            error!(error = %e, workspace_id = %workspace_id, "get workspace tracks failed");
            HttpResponse::InternalServerError().body("failed to load workspace tracks")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_workspace_tracks);
}

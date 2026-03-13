use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::ToSchema;

use crate::me::me_app_data::MeAppData;
use crate::me::{MeBootstrapResult, MeSelectProjectResult};

#[utoipa::path(
    get,
    path = "/v1/me/bootstrap",
    tag = "Me",
    responses(
        (status = 200, description = "Bootstrap success", body = MeBootstrapResult),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/bootstrap")]
pub async fn bootstrap(
    auth: JWTContext,
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

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SelectProjectInput {
    pub project_id: String,
}

#[utoipa::path(
    post,
    path = "/v1/me/select-project",
    tag = "Me",
    request_body = SelectProjectInput,
    responses(
        (status = 200, description = "Project selected", body = MeSelectProjectResult),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Membership not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("/select-project")]
pub async fn select_project(
    auth: JWTContext,
    payload: web::Json<SelectProjectInput>,
    app_state: web::Data<MeAppData>,
) -> HttpResponse {
    match app_state
        .me_data_provider
        .select_project(&auth.user_id, &payload.project_id)
        .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) if e.contains("not found") => HttpResponse::NotFound().body(e),
        Err(e) => {
            error!(user_id = %auth.user_id, project_id = %payload.project_id, error = %e, "select project failed");
            HttpResponse::InternalServerError().body("failed to select project")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(bootstrap).service(select_project);
}

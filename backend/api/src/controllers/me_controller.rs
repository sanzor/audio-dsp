use actix_web::{get, web, HttpResponse};
use tracing::error;

use crate::me::me_app_data::MeAppData;
use crate::me::MeBootstrapResult;
use crate::middlewares::jwt::jwt_context::JwtContext;

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

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(bootstrap);
}

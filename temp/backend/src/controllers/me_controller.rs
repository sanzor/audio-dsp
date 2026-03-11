use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::me::me_app_data::MeAppData;
use crate::me::{MeMembershipResult, OrganizationDataResult};
use crate::middlewares::jwt::jwt_context::JWTContext;
use crate::users::user_result::UserResult;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct BootstrapInput {
    pub user_id: UserId,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct BootstrapOutput {
    pub user: UserResult,
    pub organizations: Vec<MeMembershipResult>,
}
#[utoipa::path(
    get,
    path = "/v1/me/bootstrap",
    tag = "Me",
    request_body = BootstrapInput,
    responses(
        (status = 200, description = "Bootstrap success", body = BootstrapOutput),
        (status = 401, description = "Missing or invalid user header"),
        (status = 404, description = "Membership not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/bootstrap")] // Changed from post to get
pub async fn bootstrap(
    auth: JWTContext, // Use the middleware's context!
    app_state: web::Data<MeAppData>,
) -> HttpResponse {
    let user_id = auth.user_id; // Identity is guaranteed by the token

    match app_state.me_data_provider.get_bootstrap_data(user_id).await {
        Ok(data) => HttpResponse::Ok().json(BootstrapOutput {
            user: data.user,
            organizations: data.organizations,
        }),
        Err(e) => {
            error!(user_id, error = %e, "bootstrap failed");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SelectOrgInput {
    pub org_id: OrganizationId,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct SelectOrgResult {
    pub org_id: OrganizationId,
    pub token: String,
}
#[utoipa::path(
    post,
    path = "/v1/me/select-org",
    tag = "Me",
    request_body = SelectOrgInput,
    responses(
        (status = 200, description = "Organization selected", body = OrganizationDataResult),
        (status = 401, description = "Missing or invalid user header"),
        (status = 404, description = "Membership not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("/select-org")]
pub async fn select_org(
    auth: JWTContext,
    payload: web::Json<SelectOrgInput>,
    app_state: web::Data<MeAppData>,
) -> HttpResponse {
    let user_id = auth.user_id;
    let org_id = payload.org_id;

    info!(user_id, org_id, "select org request received");
    match app_state
        .me_data_provider
        .get_membership_details(user_id, org_id)
        .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(ServiceError::NotFound) => HttpResponse::NotFound().body("membership not found"),
        Err(e) => {
            error!(user_id, org_id, error = %e, "select org failed");
            HttpResponse::InternalServerError().body("failed to select organization")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(select_org).service(bootstrap);
}

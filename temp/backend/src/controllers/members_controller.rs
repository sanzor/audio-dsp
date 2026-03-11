use actix_web::{get, web, HttpResponse};
use serde::Serialize;
use tracing::{error, info};
use utoipa::ToSchema;

use crate::members::members_app_data::MembersAppData;
use crate::members::org_member::OrgMember;
use crate::middlewares::organization::organization_context::OrganizationContext;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;

#[derive(Serialize, ToSchema)]
pub struct ListMembersOutput {
    pub members: Vec<OrgMember>,
}

#[utoipa::path(
    get,
    path = "/members",
    tag = "Members",
    responses(
        (status = 200, description = "Members retrieved", body = ListMembersOutput),
        (status = 401, description = "Unauthorized or org context missing"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
pub async fn list_members(
    perms: PermissionsContext,
    org_ctx: OrganizationContext,
    app_state: web::Data<MembersAppData>,
) -> HttpResponse {
    if !perms.has("memberships:read") {
        return HttpResponse::Forbidden().body("missing required permission: memberships:read");
    }
    let org_id = org_ctx.id;
    info!(org_id, "list members request received");

    match app_state.members_provider.list_org_members(org_id).await {
        Ok(members) => HttpResponse::Ok().json(ListMembersOutput { members }),
        Err(e) => {
            error!(org_id, error = %e, "list members failed");
            HttpResponse::InternalServerError().body("failed to fetch members")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list_members);
}

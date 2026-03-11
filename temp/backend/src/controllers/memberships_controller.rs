use actix_web::{delete, get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::memberships::memberships_app_data::MembershipsAppData;
use crate::memberships::DbMembership;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MembershipResult {
    pub user_id: UserId,
    pub org_id: OrganizationId,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MembershipsResult {
    pub memberships: Vec<MembershipResult>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MembershipPath {
    pub user_id: UserId,
    pub org_id: OrganizationId,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MembershipQuery {
    pub user_id: Option<UserId>,
    pub org_id: Option<OrganizationId>,
}

fn map_membership(membership: DbMembership) -> MembershipResult {
    MembershipResult {
        user_id: membership.user_id,
        org_id: membership.org_id,
    }
}

#[utoipa::path(
    delete,
    path = "/memberships/{user_id}/{org_id}",
    tag = "Memberships",
    params(
        ("user_id" = UserId, Path, description = "User id"),
        ("org_id" = OrganizationId, Path, description = "Organization id"),
    ),
    responses(
        (status = 204, description = "Membership deleted"),
        (status = 404, description = "Membership not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[delete("/{user_id}/{org_id}")]
pub async fn delete_membership(
    perms: PermissionsContext,
    path: web::Path<MembershipPath>,
    app_state: web::Data<MembershipsAppData>,
) -> HttpResponse {
    if !perms.has("memberships:delete") {
        return HttpResponse::Forbidden().body("missing required permission: memberships:delete");
    }
    let path = path.into_inner();
    info!(
        user_id = path.user_id,
        org_id = path.org_id,
        "delete membership request received"
    );

    match app_state
        .memberships_provider
        .delete_membership(path.user_id, path.org_id)
        .await
    {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("membership not found"),
        Err(e) => {
            error!(error = %e, "delete membership failed");
            HttpResponse::InternalServerError().body("failed to delete membership")
        }
    }
}

#[utoipa::path(
    get,
    path = "/memberships",
    tag = "Memberships",
    params(
        ("user_id" = Option<u64>, Query, description = "Filter by user id"),
        ("org_id" = Option<u64>, Query, description = "Filter by org id"),
    ),
    responses(
        (status = 200, description = "Memberships retrieved", body = MembershipsResult),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
pub async fn list_memberships(
    perms: PermissionsContext,
    query: web::Query<MembershipQuery>,
    app_state: web::Data<MembershipsAppData>,
) -> HttpResponse {
    if !perms.has("memberships:read") {
        return HttpResponse::Forbidden().body("missing required permission: memberships:read");
    }
    let query = query.into_inner();
    info!(
        user_id = ?query.user_id,
        org_id = ?query.org_id,
        "list memberships request received"
    );

    match app_state
        .memberships_provider
        .list_memberships(query.user_id, query.org_id)
        .await
    {
        Ok(result) => HttpResponse::Ok().json(MembershipsResult {
            memberships: result.into_iter().map(map_membership).collect(),
        }),
        Err(e) => {
            error!(error = %e, "list memberships failed");
            HttpResponse::InternalServerError().body("failed to fetch memberships")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(delete_membership).service(list_memberships);
}

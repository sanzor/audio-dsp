use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_role::RoleId;
use crate::domain::db::db_user::UserId;
use crate::membership_roles::create_membership_role_params::CreateMembershipRoleParams;
use crate::membership_roles::membership_roles_app_data::MembershipRolesAppData;
use crate::membership_roles::DbMembershipRole;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;
use actix_web::{delete, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MembershipRolePath {
    pub user_id: UserId,
    pub org_id: OrganizationId,
    pub role_id: RoleId,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct MembershipRoleResult {
    pub user_id: UserId,
    pub org_id: OrganizationId,
    pub role_id: RoleId,
}

impl TryFrom<DbMembershipRole> for MembershipRoleResult {
    type Error = String;

    fn try_from(value: DbMembershipRole) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: value.user_id,
            org_id: value.org_id,
            role_id: RoleId::try_from(value.role_id)
                .map_err(|_| "role id out of range".to_string())?,
        })
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct CreateMembershipRoleInput {
    pub user_id: UserId,
    pub org_id: OrganizationId,
    pub role_id: RoleId,
}

#[derive(ToSchema, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CreateMembershipRoleResult {
    pub membership_role: MembershipRoleResult,
}

#[utoipa::path(
    post,
    path = "/membership-roles",
    tag = "MembershipRoles",
    request_body = CreateMembershipRoleInput,
    responses(
        (status = 201, description = "Membership role created", body = CreateMembershipRoleResult),
        (status = 400, description = "Invalid input parameters"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("")]
pub async fn create_membership_role(
    perms: PermissionsContext,
    payload: web::Json<CreateMembershipRoleInput>,
    app_state: web::Data<MembershipRolesAppData>,
) -> HttpResponse {
    if !perms.has("membership_roles:create") {
        return HttpResponse::Forbidden()
            .body("missing required permission: membership_roles:create");
    }
    info!("create membership role request received");
    let input = payload.into_inner();

    if input.user_id <= 0 || input.org_id <= 0 || input.role_id == 0 {
        warn!("create membership role rejected: missing ids");
        return HttpResponse::BadRequest().body("user_id, org_id, and role_id are required");
    }

    let params = CreateMembershipRoleParams {
        user_id: input.user_id,
        org_id: input.org_id,
        role_id: input.role_id,
    };

    match app_state
        .membership_roles_provider
        .create_membership_role(params)
        .await
    {
        Ok(result) => HttpResponse::Created().json(CreateMembershipRoleResult {
            membership_role: match MembershipRoleResult::try_from(result) {
                Ok(mapped) => mapped,
                Err(e) => {
                    error!(error = %e, "create membership role returned invalid record");
                    return HttpResponse::InternalServerError()
                        .body("failed to create membership role");
                }
            },
        }),
        Err(e) => {
            error!(error = %e, "create membership role failed");
            HttpResponse::InternalServerError().body("failed to create membership role")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/membership-roles/{user_id}/{org_id}/{role_id}",
    tag = "MembershipRoles",
    params(
        ("user_id" = i64, Path, description = "User id"),
        ("org_id" = i64, Path, description = "Organization id"),
        ("role_id" = u64, Path, description = "Role id"),
    ),
    responses(
        (status = 204, description = "Membership role deleted"),
        (status = 404, description = "Membership role not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[delete("/{user_id}/{org_id}/{role_id}")]
pub async fn delete_membership_role(
    perms: PermissionsContext,
    path: web::Path<MembershipRolePath>,
    app_state: web::Data<MembershipRolesAppData>,
) -> HttpResponse {
    if !perms.has("membership_roles:delete") {
        return HttpResponse::Forbidden()
            .body("missing required permission: membership_roles:delete");
    }
    let path = path.into_inner();
    info!(
        user_id = path.user_id,
        org_id = path.org_id,
        role_id = path.role_id,
        "delete membership role request received"
    );

    match app_state
        .membership_roles_provider
        .delete_membership_role(path.user_id, path.org_id, path.role_id)
        .await
    {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("membership role not found"),
        Err(e) => {
            error!(error = %e, "delete membership role failed");
            HttpResponse::InternalServerError().body("failed to delete membership role")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_membership_role)
        .service(delete_membership_role);
}

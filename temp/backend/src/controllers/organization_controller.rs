use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::middlewares::jwt::jwt_context::JWTContext;
use crate::middlewares::permissions_context::permissions_context::PermissionsContext;
use crate::organizations::create_organization_params::CreateOrganizationParams;
use crate::organizations::organizations_app_data::OrganizationsAppData;
use crate::organizations::update_organization_params::UpdateOrganizationParams;
use crate::organizations::DbOrganization;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct OrganizationResult {
    pub id: OrganizationId,
    pub name: String,
    pub slug: String,
    pub billing_email: String,
    pub stripe_customer_id: Option<String>,
    pub status: String,
}
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct OrganizationsResult {
    pub organizations: Vec<OrganizationResult>,
    pub total: i64,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct OrgListQuery {
    pub index: Option<i64>,
    pub count: Option<i64>,
    pub search: Option<String>,
}
fn map_organization(org: DbOrganization) -> OrganizationResult {
    OrganizationResult {
        id: org.id,
        name: org.name,
        slug: org.slug,
        billing_email: org.billing_email,
        stripe_customer_id: org.stripe_customer_id,
        status: org.status,
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct CreateOrganizationInput {
    pub name: String,
    pub slug: String,
    pub billing_email: String,
    pub stripe_customer_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct OrganizationMembershipResult {
    pub user_id: UserId,
    pub org_id: OrganizationId,
    pub roles: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct CreateOrganizationResult {
    pub organization: OrganizationResult,
    pub membership: Option<OrganizationMembershipResult>,
}

#[utoipa::path(
    post,
    path = "/organizations",
    tag = "Organizations",
    request_body = CreateOrganizationInput,
    responses(
        (status = 201, description = "Organization created", body = CreateOrganizationResult),
        (status = 400, description = "Invalid input parameters"),
        (status = 500, description = "Internal server error"),
    )
)]
#[post("")]
pub async fn create_organization(
    auth: JWTContext,
    payload: web::Json<CreateOrganizationInput>,
    app_state: web::Data<OrganizationsAppData>,
) -> HttpResponse {
    info!("create organization request received");
    let input = payload.into_inner();
    if input.name.trim().is_empty() {
        warn!("create organization rejected: missing name");
        return HttpResponse::BadRequest().body("name is required");
    }
    if input.slug.trim().is_empty() {
        warn!("create organization rejected: missing slug");
        return HttpResponse::BadRequest().body("slug is required");
    }
    if input.billing_email.trim().is_empty() {
        warn!("create organization rejected: missing billing_email");
        return HttpResponse::BadRequest().body("billing_email is required");
    }

    let params = CreateOrganizationParams {
        name: input.name,
        slug: input.slug,
        billing_email: input.billing_email,
        stripe_customer_id: input.stripe_customer_id,
        status: input.status,
    };
    match app_state
        .organization_provider
        .create_organization_with_owner(params, auth.user_id)
        .await
    {
        Ok(result) => HttpResponse::Created().json(CreateOrganizationResult {
            organization: map_organization(result.organization),
            membership: Some(OrganizationMembershipResult {
                user_id: result.membership.user_id,
                org_id: result.membership.org_id,
                roles: vec![result.owner_role.name],
            }),
        }),
        Err(ServiceError::Conflict(_)) => HttpResponse::Conflict()
            .body("an organization with this slug or billing email already exists"),
        Err(e) => {
            error!(error = %e, "create organization failed");
            HttpResponse::InternalServerError().body("failed to create organization")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateOrganizationInput {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub billing_email: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateOrganizationResult {
    pub organization: OrganizationResult,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct UpdateOrganizationPath {
    pub org_id: OrganizationId,
}

#[utoipa::path(
    put,
    path = "/organizations/{org_id}",
    tag = "Organizations",
    request_body = UpdateOrganizationInput,
    params(
        ("org_id" = OrganizationId, Path, description = "Organization id"),
    ),
    responses(
        (status = 200, description = "Organization updated", body = UpdateOrganizationResult),
        (status = 404, description = "Organization not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[put("/{org_id}")]
pub async fn update_organization(
    perms: PermissionsContext,
    path: web::Path<UpdateOrganizationPath>,
    payload: web::Json<UpdateOrganizationInput>,
    app_state: web::Data<OrganizationsAppData>,
) -> HttpResponse {
    if !perms.has("organizations:update") {
        return HttpResponse::Forbidden().body("missing required permission: organizations:update");
    }
    let org_id = path.into_inner().org_id;
    info!(org_id, "update organization request received");
    let input = payload.into_inner();

    if input.name.is_none()
        && input.slug.is_none()
        && input.billing_email.is_none()
        && input.stripe_customer_id.is_none()
        && input.status.is_none()
    {
        warn!(org_id, "update organization rejected: no fields provided");
        return HttpResponse::BadRequest().body("at least one field is required");
    }

    let params = UpdateOrganizationParams {
        name: input.name,
        slug: input.slug,
        billing_email: input.billing_email,
        stripe_customer_id: input.stripe_customer_id,
        status: input.status,
    };
    match app_state
        .organization_provider
        .update_organization(org_id, params)
        .await
    {
        Ok(Some(result)) => HttpResponse::Ok().json(UpdateOrganizationResult {
            organization: map_organization(result.organization),
        }),
        Ok(None) => HttpResponse::NotFound().body("organization not found"),
        Err(e) => {
            error!(org_id, error = %e, "update organization failed");
            HttpResponse::InternalServerError().body("failed to update organization")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct DeleteOrganizationPath {
    pub org_id: OrganizationId,
}

#[utoipa::path(
    delete,
    path = "/organizations/{org_id}",
    tag = "Organizations",
    params(
        ("org_id" = OrganizationId, Path, description = "Organization id"),
    ),
    responses(
        (status = 204, description = "Organization deleted"),
        (status = 404, description = "Organization not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[delete("/{org_id}")]
pub async fn delete_organization(
    perms: PermissionsContext,
    path: web::Path<DeleteOrganizationPath>,
    app_state: web::Data<OrganizationsAppData>,
) -> HttpResponse {
    if !perms.has("organizations:delete") {
        return HttpResponse::Forbidden().body("missing required permission: organizations:delete");
    }
    let org_id = path.into_inner().org_id;
    info!(org_id, "delete organization request received");

    match app_state
        .organization_provider
        .delete_organization(org_id)
        .await
    {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("organization not found"),
        Err(e) => {
            error!(org_id, error = %e, "delete organization failed");
            HttpResponse::InternalServerError().body("failed to delete organization")
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GetOrganizationPath {
    pub org_id: OrganizationId,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GetOrganizationResult {
    pub organization: OrganizationResult,
}

#[utoipa::path(
    get,
    path = "/organizations/{org_id}",
    tag = "Organizations",
    params(
        ("org_id" = OrganizationId, Path, description = "Organization id"),
    ),
    responses(
        (status = 200, description = "Organization retrieved", body = GetOrganizationResult),
        (status = 404, description = "Organization not found"),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("/{org_id}")]
pub async fn get_organization(
    perms: PermissionsContext,
    path: web::Path<GetOrganizationPath>,
    app_state: web::Data<OrganizationsAppData>,
) -> HttpResponse {
    if !perms.has("organizations:read") {
        return HttpResponse::Forbidden().body("missing required permission: organizations:read");
    }
    let org_id = path.into_inner().org_id;
    info!(org_id, "get organization request received");

    match app_state
        .organization_provider
        .get_organization(org_id)
        .await
    {
        Ok(Some(result)) => HttpResponse::Ok().json(GetOrganizationResult {
            organization: map_organization(result.organization),
        }),
        Ok(None) => HttpResponse::NotFound().body("organization not found"),
        Err(e) => {
            error!(org_id, error = %e, "get organization failed");
            HttpResponse::InternalServerError().body("failed to fetch organization")
        }
    }
}

#[utoipa::path(
    get,
    path = "/organizations",
    tag = "Organizations",
    responses(
        (status = 200, description = "Organizations retrieved", body = OrganizationsResult),
        (status = 500, description = "Internal server error"),
    )
)]
#[get("")]
pub async fn get_all_organizations(
    perms: PermissionsContext,
    query: web::Query<OrgListQuery>,
    app_state: web::Data<OrganizationsAppData>,
) -> HttpResponse {
    if !perms.has("organizations:read") {
        return HttpResponse::Forbidden().body("missing required permission: organizations:read");
    }
    let offset = query.index.unwrap_or(0).max(0) as usize;
    let limit = query.count.unwrap_or(20).clamp(1, 100) as usize;
    let search = query.search.as_deref().map(str::to_lowercase);
    info!(offset, limit, "list organizations request received");

    match app_state
        .organization_provider
        .get_all_organizations()
        .await
    {
        Ok(result) => {
            let filtered: Vec<_> = result
                .into_iter()
                .filter(|o| {
                    search.as_deref().map_or(true, |s| {
                        o.name.to_lowercase().contains(s) || o.slug.to_lowercase().contains(s)
                    })
                })
                .collect();
            let total = filtered.len() as i64;
            let page: Vec<_> = filtered
                .into_iter()
                .skip(offset)
                .take(limit)
                .map(map_organization)
                .collect();
            HttpResponse::Ok().json(OrganizationsResult {
                organizations: page,
                total,
            })
        }
        Err(e) => {
            error!(error = %e, "list organizations failed");
            HttpResponse::InternalServerError().body("failed to fetch organizations")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_organization)
        .service(update_organization)
        .service(delete_organization)
        .service(get_organization)
        .service(get_all_organizations);
}

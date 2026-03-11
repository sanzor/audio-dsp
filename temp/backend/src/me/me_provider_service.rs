use std::sync::Arc;

use async_trait::async_trait;
use tracing::{error, warn};

use crate::access_control::access_control_provider::AccessControlProvider;
use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::me::me_data_provider::MeProvider;
use crate::me::{
    me_bootstrap_result::MeBootstrapResult, me_membership_result::MeMembershipResult,
    me_organization_result::MeOrganizationResult, organization_data_result::OrganizationDataResult,
};
use crate::membership_roles::membership_roles_provider::MembershipRolesProvider;
use crate::memberships::memberships_provider::MembershipsProvider;
use crate::middlewares::jwt::jwt_provider::JwtProvider;
use crate::organizations::organization_provider::OrganizationProvider;
use crate::roles::roles_provider::RolesProvider;
use crate::users::user_provider::UserProvider;
use crate::users::user_result::UserResult;

pub struct MeProviderService {
    users_provider: Arc<dyn UserProvider>,
    memberships_provider: Arc<dyn MembershipsProvider>,
    membership_roles_provider: Arc<dyn MembershipRolesProvider>,
    roles_provider: Arc<dyn RolesProvider>,
    organizations_provider: Arc<dyn OrganizationProvider>,
    access_control_provider: Arc<dyn AccessControlProvider>,
    jwt_provider: Arc<dyn JwtProvider>,
}

impl MeProviderService {
    pub fn new(
        users_provider: Arc<dyn UserProvider>,
        memberships_provider: Arc<dyn MembershipsProvider>,
        membership_roles_provider: Arc<dyn MembershipRolesProvider>,
        roles_provider: Arc<dyn RolesProvider>,
        organizations_provider: Arc<dyn OrganizationProvider>,
        access_control_provider: Arc<dyn AccessControlProvider>,
        jwt_provider: Arc<dyn JwtProvider>,
    ) -> Self {
        Self {
            users_provider,
            memberships_provider,
            membership_roles_provider,
            roles_provider,
            organizations_provider,
            access_control_provider,
            jwt_provider,
        }
    }

    async fn resolve_role_names(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<Vec<String>, ServiceError> {
        let membership_roles = self
            .membership_roles_provider
            .list_membership_roles(Some(user_id), Some(org_id), None)
            .await?;

        let mut role_names = Vec::new();
        for mr in membership_roles {
            match self.roles_provider.get_role(mr.role_id).await {
                Ok(Some(role)) => role_names.push(role.name),
                Ok(None) => warn!(
                    user_id,
                    org_id,
                    role_id = mr.role_id,
                    "role not found for membership role"
                ),
                Err(e) => warn!(
                    user_id,
                    org_id,
                    role_id = mr.role_id,
                    error = %e,
                    "role lookup failed"
                ),
            }
        }

        Ok(role_names)
    }

    async fn resolve_org(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Option<MeOrganizationResult> {
        match self.organizations_provider.get_organization(org_id).await {
            Ok(Some(org)) => Some(MeOrganizationResult {
                id: org.organization.id,
                name: org.organization.name,
                slug: org.organization.slug,
                billing_email: org.organization.billing_email,
                stripe_customer_id: org.organization.stripe_customer_id,
                status: org.organization.status,
            }),
            Ok(None) => None,
            Err(e) => {
                warn!(user_id, org_id, error = %e, "org lookup failed");
                None
            }
        }
    }
}

#[async_trait]
impl MeProvider for MeProviderService {
    async fn get_bootstrap_data(&self, user_id: UserId) -> Result<MeBootstrapResult, ServiceError> {
        let user = self
            .users_provider
            .get_user(user_id)
            .await?
            .ok_or_else(|| "user not found".to_string())?;

        let memberships = self
            .memberships_provider
            .list_memberships(Some(user_id), None)
            .await?;

        let mut organizations = Vec::new();
        for membership in memberships {
            let org_id = OrganizationId::try_from(membership.org_id)
                .map_err(|_| "invalid org id".to_string())?;
            let roles = self
                .resolve_role_names(user_id, org_id)
                .await
                .unwrap_or_else(|e| {
                    warn!(user_id, org_id, error = %e, "resolve roles failed");
                    Vec::new()
                });
            let organization = self.resolve_org(user_id, org_id).await;

            organizations.push(MeMembershipResult {
                user_id,
                org_id,
                roles,
                organization,
            });
        }

        Ok(MeBootstrapResult {
            user: UserResult {
                id: user.id,
                email: user.email,
                full_name: user.full_name,
                is_active: user.is_active,
                is_verified: user.is_verified,
            },
            organizations,
        })
    }

    async fn get_membership_details(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<OrganizationDataResult, ServiceError> {
        let membership = self
            .memberships_provider
            .get_membership(user_id, org_id)
            .await?
            .ok_or(ServiceError::NotFound)?;

        let roles = self.resolve_role_names(user_id, org_id).await?;
        let organization = self.resolve_org(user_id, org_id).await;

        let permissions = self
            .access_control_provider
            .permissions_for_user(user_id, org_id)
            .await?;

        let token = self
            .jwt_provider
            .issue_org_token(user_id, org_id)
            .map_err(|e| {
                error!(user_id, org_id, error = %e, "issue scoped token failed");
                "failed to issue scoped token".to_string()
            })?;

        Ok(OrganizationDataResult {
            org_id,
            token,
            membership: MeMembershipResult {
                user_id,
                org_id,
                roles,
                organization,
            },
            permissions,
        })
    }
}

use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_role::RoleId;
use crate::domain::db::db_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::membership_roles::create_membership_role_params::CreateMembershipRoleParams;
use crate::membership_roles::data_provider::membership_roles_data_provider::MembershipRolesDataProvider;
use crate::membership_roles::{
    membership_roles_provider::MembershipRolesProvider, DbMembershipRole,
};

pub struct MembershipRolesProviderService {
    data_provider: Arc<dyn MembershipRolesDataProvider>,
}

impl MembershipRolesProviderService {
    pub fn new(data_provider: Arc<dyn MembershipRolesDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait]
impl MembershipRolesProvider for MembershipRolesProviderService {
    async fn create_membership_role(
        &self,
        params: CreateMembershipRoleParams,
    ) -> Result<DbMembershipRole, ServiceError> {
        info!(
            user_id = params.user_id,
            org_id = params.org_id,
            role_id = params.role_id,
            "create membership role requested"
        );
        Ok(self
            .data_provider
            .create_membership_role(params.user_id, params.org_id, params.role_id)
            .await?)
    }

    async fn delete_membership_role(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        role_id: RoleId,
    ) -> Result<bool, ServiceError> {
        info!(user_id, org_id, role_id, "delete membership role requested");
        Ok(self
            .data_provider
            .delete_membership_role(user_id, org_id, role_id)
            .await?)
    }

    async fn get_membership_role(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        role_id: RoleId,
    ) -> Result<Option<DbMembershipRole>, ServiceError> {
        info!(user_id, org_id, role_id, "get membership role requested");
        Ok(self
            .data_provider
            .get_membership_role(user_id, org_id, role_id)
            .await?)
    }

    async fn list_membership_roles(
        &self,
        user_id: Option<UserId>,
        org_id: Option<OrganizationId>,
        role_id: Option<RoleId>,
    ) -> Result<Vec<DbMembershipRole>, ServiceError> {
        info!(user_id = ?user_id, org_id = ?org_id, role_id = ?role_id, "list membership roles requested");
        Ok(self
            .data_provider
            .list_membership_roles(user_id, org_id, role_id)
            .await?)
    }
}

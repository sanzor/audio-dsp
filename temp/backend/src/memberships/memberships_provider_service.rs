use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::db::db_user::UserId;
use crate::domain::service_error::ServiceError;
use crate::memberships::create_membership_params::CreateMembershipParams;
use crate::memberships::data_provider::memberships_data_provider::MembershipsDataProvider;
use crate::memberships::{memberships_provider::MembershipsProvider, DbMembership};

pub struct MembershipsProviderService {
    data_provider: Arc<dyn MembershipsDataProvider>,
}

impl MembershipsProviderService {
    pub fn new(data_provider: Arc<dyn MembershipsDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait]
impl MembershipsProvider for MembershipsProviderService {
    async fn create_membership(
        &self,
        params: CreateMembershipParams,
    ) -> Result<DbMembership, ServiceError> {
        info!(
            user_id = params.user_id,
            org_id = params.org_id,
            "create membership requested"
        );
        Ok(self
            .data_provider
            .create_membership(params.user_id, params.org_id)
            .await?)
    }

    async fn delete_membership(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<bool, ServiceError> {
        info!(user_id, org_id, "delete membership requested");
        Ok(self
            .data_provider
            .delete_membership(user_id, org_id)
            .await?)
    }

    async fn get_membership(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<Option<DbMembership>, ServiceError> {
        info!(user_id, org_id, "get membership requested");
        Ok(self.data_provider.get_membership(user_id, org_id).await?)
    }

    async fn list_memberships(
        &self,
        user_id: Option<UserId>,
        org_id: Option<OrganizationId>,
    ) -> Result<Vec<DbMembership>, ServiceError> {
        info!(user_id = ?user_id, org_id = ?org_id, "list memberships requested");
        Ok(self.data_provider.list_memberships(user_id, org_id).await?)
    }
}

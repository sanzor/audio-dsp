use std::sync::Arc;

use domain::{db::DbMembership, domain_user::UserId};

use super::{
    data_provider::memberships_data_provider::MembershipsDataProvider,
    memberships_provider::{CreateMembershipParams, MembershipsProvider, WorkspaceRole},
};

pub struct MembershipsProviderService {
    data_provider: Arc<dyn MembershipsDataProvider>,
}

impl MembershipsProviderService {
    pub fn new(data_provider: Arc<dyn MembershipsDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl MembershipsProvider for MembershipsProviderService {
    async fn create_membership(
        &self,
        params: CreateMembershipParams,
    ) -> Result<DbMembership, String> {
        self.data_provider.create_membership(params).await
    }

    async fn delete_membership(&self, workspace_id: i32, user_id: UserId) -> Result<bool, String> {
        self.data_provider
            .delete_membership(workspace_id, user_id)
            .await
    }

    async fn get_membership(
        &self,
        workspace_id: i32,
        user_id: UserId,
    ) -> Result<Option<DbMembership>, String> {
        self.data_provider
            .get_membership(workspace_id, user_id)
            .await
    }

    async fn list_memberships(
        &self,
        workspace_id: Option<i32>,
        user_id: Option<UserId>,
    ) -> Result<Vec<DbMembership>, String> {
        self.data_provider
            .list_memberships(workspace_id, user_id)
            .await
    }

    async fn get_role(
        &self,
        workspace_id: i32,
        user_id: UserId,
    ) -> Result<Option<WorkspaceRole>, String> {
        self.data_provider.get_role(workspace_id, user_id).await
    }

    async fn update_role(
        &self,
        workspace_id: i32,
        user_id: UserId,
        role: WorkspaceRole,
    ) -> Result<Option<DbMembership>, String> {
        self.data_provider
            .update_role(workspace_id, user_id, role)
            .await
    }
}

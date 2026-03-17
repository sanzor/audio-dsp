use std::sync::Arc;

use domain::db::DbMembership;

use super::{
    data_provider::memberships_data_provider::MembershipsDataProvider,
    memberships_provider::{CreateMembershipParams, MembershipsProvider, ProjectRole},
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

    async fn delete_membership(&self, project_id: &str, user_id: &str) -> Result<bool, String> {
        self.data_provider.delete_membership(project_id, user_id).await
    }

    async fn get_membership(
        &self,
        project_id: &str,
        user_id: &str,
    ) -> Result<Option<DbMembership>, String> {
        self.data_provider.get_membership(project_id, user_id).await
    }

    async fn list_memberships(
        &self,
        project_id: Option<&str>,
        user_id: Option<&str>,
    ) -> Result<Vec<DbMembership>, String> {
        self.data_provider.list_memberships(project_id, user_id).await
    }

    async fn get_role(
        &self,
        project_id: &str,
        user_id: &str,
    ) -> Result<Option<ProjectRole>, String> {
        self.data_provider.get_role(project_id, user_id).await
    }

    async fn update_role(
        &self,
        project_id: &str,
        user_id: &str,
        role: ProjectRole,
    ) -> Result<Option<DbMembership>, String> {
        self.data_provider.update_role(project_id, user_id, role).await
    }
}

use domain::{db::DbMembership, workspace_role::WorkspaceRole};

use crate::memberships::memberships_provider::CreateMembershipParams;

#[async_trait::async_trait]
pub trait MembershipsDataProvider: Send + Sync {
    async fn create_membership(
        &self,
        params: CreateMembershipParams,
    ) -> Result<DbMembership, String>;
    async fn delete_membership(&self, workspace_id: i32, user_id: i32) -> Result<bool, String>;
    async fn get_membership(
        &self,
        workspace_id: i32,
        user_id: i32,
    ) -> Result<Option<DbMembership>, String>;
    async fn list_memberships(
        &self,
        workspace_id: Option<i32>,
        user_id: Option<i32>,
    ) -> Result<Vec<DbMembership>, String>;
    async fn get_role(
        &self,
        workspace_id: i32,
        user_id: i32,
    ) -> Result<Option<WorkspaceRole>, String>;
    async fn update_role(
        &self,
        workspace_id: i32,
        user_id: i32,
        role: WorkspaceRole,
    ) -> Result<Option<DbMembership>, String>;
}

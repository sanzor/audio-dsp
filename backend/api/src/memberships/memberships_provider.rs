use domain::{db::DbMembership, domain_user::UserId};
pub use domain::workspace_role::WorkspaceRole;

pub struct CreateMembershipParams {
    pub workspace_id: i32,
    pub user_id: UserId,
    pub role: WorkspaceRole,
}

#[async_trait::async_trait]
pub trait MembershipsProvider: Send + Sync {
    async fn create_membership(
        &self,
        params: CreateMembershipParams,
    ) -> Result<DbMembership, String>;
    async fn delete_membership(&self, workspace_id: i32, user_id: UserId) -> Result<bool, String>;
    async fn get_membership(
        &self,
        workspace_id: i32,
        user_id: UserId,
    ) -> Result<Option<DbMembership>, String>;
    async fn list_memberships(
        &self,
        workspace_id: Option<i32>,
        user_id: Option<UserId>,
    ) -> Result<Vec<DbMembership>, String>;
    async fn get_role(
        &self,
        workspace_id: i32,
        user_id: UserId,
    ) -> Result<Option<WorkspaceRole>, String>;
    async fn update_role(
        &self,
        workspace_id: i32,
        user_id: UserId,
        role: WorkspaceRole,
    ) -> Result<Option<DbMembership>, String>;
}

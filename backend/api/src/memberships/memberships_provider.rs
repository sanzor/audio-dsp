use domain::db::DbMembership;
pub use domain::project_role::ProjectRole;

pub struct CreateMembershipParams {
    pub project_id: i64,
    pub user_id: i64,
    pub role: ProjectRole,
}

#[async_trait::async_trait]
pub trait MembershipsProvider: Send + Sync {
    async fn create_membership(
        &self,
        params: CreateMembershipParams,
    ) -> Result<DbMembership, String>;
    async fn delete_membership(&self, project_id: i64, user_id: i64) -> Result<bool, String>;
    async fn get_membership(
        &self,
        project_id: i64,
        user_id: i64,
    ) -> Result<Option<DbMembership>, String>;
    async fn list_memberships(
        &self,
        project_id: Option<i64>,
        user_id: Option<i64>,
    ) -> Result<Vec<DbMembership>, String>;
    async fn get_role(
        &self,
        project_id: i64,
        user_id: i64,
    ) -> Result<Option<ProjectRole>, String>;
    async fn update_role(
        &self,
        project_id: i64,
        user_id: i64,
        role: ProjectRole,
    ) -> Result<Option<DbMembership>, String>;
}

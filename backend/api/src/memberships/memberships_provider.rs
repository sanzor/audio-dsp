use domain::db::DbMembership;
pub use domain::project_role::ProjectRole;

pub struct CreateMembershipParams {
    pub project_id: String,
    pub user_id: String,
    pub role: ProjectRole,
}

#[async_trait::async_trait]
pub trait MembershipsProvider: Send + Sync {
    async fn create_membership(
        &self,
        params: CreateMembershipParams,
    ) -> Result<DbMembership, String>;
    async fn delete_membership(&self, project_id: &str, user_id: &str) -> Result<bool, String>;
    async fn get_membership(
        &self,
        project_id: &str,
        user_id: &str,
    ) -> Result<Option<DbMembership>, String>;
    async fn list_memberships(
        &self,
        project_id: Option<&str>,
        user_id: Option<&str>,
    ) -> Result<Vec<DbMembership>, String>;
    async fn get_role(
        &self,
        project_id: &str,
        user_id: &str,
    ) -> Result<Option<ProjectRole>, String>;
}

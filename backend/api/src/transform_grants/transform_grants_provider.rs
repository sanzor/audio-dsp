use domain::db::{DbTransformGrant, TransformId};

pub struct CreateGrantParams {
    pub transform_id: TransformId,
    pub grantee_user_id: Option<i32>,
    pub grantee_workspace_id: Option<i32>,
    pub granted_by: i32,
}

#[async_trait::async_trait]
pub trait TransformGrantsProvider: Send + Sync {
    async fn create_grant(&self, params: CreateGrantParams) -> Result<DbTransformGrant, String>;
    async fn delete_grant(&self, transform_id: TransformId, grant_id: i64) -> Result<bool, String>;
    async fn list_grants(&self, transform_id: TransformId) -> Result<Vec<DbTransformGrant>, String>;
    /// Owner/admin bypass is the caller's responsibility — this only answers
    /// "does a grant (direct or via workspace membership) exist for this user".
    async fn has_access(&self, transform_id: TransformId, user_id: i32) -> Result<bool, String>;
}

use async_trait::async_trait;

use crate::me::{
    me_bootstrap_result::MeBootstrapResult,
    me_select_project_result::MeSelectProjectResult,
};

#[async_trait]
pub trait MeProvider: Send + Sync {
    /// Called immediately after login — returns the user + list of projects with roles.
    async fn get_bootstrap_data(&self, user_id: &str) -> Result<MeBootstrapResult, String>;

    /// Called when the user selects a project — issues a project-scoped JWT.
    async fn select_project(
        &self,
        user_id: &str,
        project_id: &str,
    ) -> Result<MeSelectProjectResult, String>;
}

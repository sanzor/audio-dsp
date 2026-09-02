use async_trait::async_trait;

use crate::me::me_bootstrap_result::MeBootstrapResult;

#[async_trait]
pub trait MeProvider: Send + Sync {
    /// Called immediately after login — returns the user + list of workspaces with roles.
    async fn get_bootstrap_data(
        &self,
        user_id: domain::domain_user::UserId,
    ) -> Result<MeBootstrapResult, String>;
}

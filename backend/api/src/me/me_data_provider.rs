use async_trait::async_trait;

use crate::me::me_bootstrap_result::MeBootstrapResult;

#[async_trait]
pub trait MeProvider: Send + Sync {
    /// Called immediately after login — returns the user + list of projects with roles.
    async fn get_bootstrap_data(&self, user_id: i32) -> Result<MeBootstrapResult, String>;
}

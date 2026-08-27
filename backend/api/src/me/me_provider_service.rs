use std::sync::Arc;

use async_trait::async_trait;
use domain::domain_user::UserId;


use crate::me::me_bootstrap_result::{MeBootstrapResult, MeUserResult};
use crate::me::me_data_provider::MeProvider;
use crate::me::me_workspace_result::MeWorkspaceResult;
use crate::memberships::memberships_provider::MembershipsProvider;
use crate::users::user_provider::UserProvider;
use crate::workspaces::workspaces_provider::WorkspacesProvider;

pub struct MeProviderService {
    users_provider: Arc<dyn UserProvider>,
    memberships_provider: Arc<dyn MembershipsProvider>,
    workspaces_provider: Arc<dyn WorkspacesProvider>,
}

impl MeProviderService {
    pub fn new(
        users_provider: Arc<dyn UserProvider>,
        memberships_provider: Arc<dyn MembershipsProvider>,
        workspaces_provider: Arc<dyn WorkspacesProvider>,
    ) -> Self {
        Self {
            users_provider,
            memberships_provider,
            workspaces_provider,
        }
    }
}

#[async_trait]
impl MeProvider for MeProviderService {
    async fn get_bootstrap_data(&self, user_id: UserId) -> Result<MeBootstrapResult, String> {
        let user = self
            .users_provider
            .get_user(user_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("user {} not found", user_id))?;

        let memberships = self
            .memberships_provider
            .list_memberships(None, Some(user_id))
            .await?;

        let mut workspaces = Vec::new();
        for m in memberships {
            match self.workspaces_provider.get_workspace(&m.workspace_id).await {
                Ok(Some(workspace)) => workspaces.push(MeWorkspaceResult {
                    workspace_id: workspace.workspace_id,
                    name: workspace.name,
                    role: m.role,
                }),
                Ok(None) => {
                    tracing::warn!(workspace_id = %m.workspace_id, "workspace not found for membership, skipping");
                }
                Err(e) => {
                    tracing::warn!(workspace_id = %m.workspace_id, error = %e, "workspace lookup failed, skipping");
                }
            }
        }

        Ok(MeBootstrapResult {
            user: MeUserResult {
                id: user.id,
                name: user.full_name,
                email: user.email,
                is_admin: false,
                is_verified: user.is_verified,
            },
            workspaces,
        })
    }
}

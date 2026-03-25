use std::sync::Arc;

use async_trait::async_trait;

use crate::me::me_bootstrap_result::{MeBootstrapResult, MeUserResult};
use crate::me::me_data_provider::MeProvider;
use crate::me::me_project_result::MeProjectResult;
use crate::memberships::memberships_provider::MembershipsProvider;
use crate::projects::projects_provider::ProjectsProvider;
use crate::users::user_provider::UserProvider;

pub struct MeProviderService {
    users_provider: Arc<dyn UserProvider>,
    memberships_provider: Arc<dyn MembershipsProvider>,
    projects_provider: Arc<dyn ProjectsProvider>,
}

impl MeProviderService {
    pub fn new(
        users_provider: Arc<dyn UserProvider>,
        memberships_provider: Arc<dyn MembershipsProvider>,
        projects_provider: Arc<dyn ProjectsProvider>,
    ) -> Self {
        Self {
            users_provider,
            memberships_provider,
            projects_provider,
        }
    }
}

#[async_trait]
impl MeProvider for MeProviderService {
    async fn get_bootstrap_data(&self, user_id: i32) -> Result<MeBootstrapResult, String> {
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

        let mut projects = Vec::new();
        for m in memberships {
            match self.projects_provider.get_project(&m.project_id).await {
                Ok(Some(project)) => projects.push(MeProjectResult {
                    project_id: project.project_id,
                    name: project.name,
                    role: m.role,
                }),
                Ok(None) => {
                    tracing::warn!(project_id = %m.project_id, "project not found for membership, skipping");
                }
                Err(e) => {
                    tracing::warn!(project_id = %m.project_id, error = %e, "project lookup failed, skipping");
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
            projects,
        })
    }
}

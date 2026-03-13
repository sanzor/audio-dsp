use std::sync::Arc;

use async_trait::async_trait;

use crate::auth::jwt_provider::JwtProvider;
use crate::me::me_bootstrap_result::{MeBootstrapResult, MeUserResult};
use crate::me::me_data_provider::MeProvider;
use crate::me::me_project_result::MeProjectResult;
use crate::me::me_select_project_result::MeSelectProjectResult;
use crate::memberships::memberships_provider::MembershipsProvider;
use crate::projects::projects_provider::ProjectsProvider;
use crate::users::user_provider::UserProvider;

pub struct MeProviderService {
    users_provider: Arc<dyn UserProvider>,
    memberships_provider: Arc<dyn MembershipsProvider>,
    projects_provider: Arc<dyn ProjectsProvider>,
    jwt_provider: Arc<dyn JwtProvider>,
}

impl MeProviderService {
    pub fn new(
        users_provider: Arc<dyn UserProvider>,
        memberships_provider: Arc<dyn MembershipsProvider>,
        projects_provider: Arc<dyn ProjectsProvider>,
        jwt_provider: Arc<dyn JwtProvider>,
    ) -> Self {
        Self {
            users_provider,
            memberships_provider,
            projects_provider,
            jwt_provider,
        }
    }
}

#[async_trait]
impl MeProvider for MeProviderService {
    async fn get_bootstrap_data(&self, user_id: &str) -> Result<MeBootstrapResult, String> {
        let user = self
            .users_provider
            .get_user_by_id(user_id)
            .await
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
                name: user.name,
                email: user.email,
                is_admin: user.is_admin,
                is_verified: user.is_verified,
            },
            projects,
        })
    }

    async fn select_project(
        &self,
        user_id: &str,
        project_id: &str,
    ) -> Result<MeSelectProjectResult, String> {
        let membership = self
            .memberships_provider
            .get_membership(project_id, user_id)
            .await?
            .ok_or_else(|| "membership not found".to_string())?;

        let user = self
            .users_provider
            .get_user_by_id(user_id)
            .await
            .ok_or_else(|| format!("user {} not found", user_id))?;

        let token = self.jwt_provider.issue_project_token(
            user_id,
            Some(&user.name),
            Some(&user.email),
            user.is_admin,
            project_id,
            &membership.role.to_string(),
        )?;

        Ok(MeSelectProjectResult {
            project_id: project_id.to_string(),
            token,
        })
    }
}

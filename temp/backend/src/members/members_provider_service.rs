use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::service_error::ServiceError;
use crate::members::data_provider::members_data_provider::MembersDataProvider;
use crate::members::members_provider::MembersProvider;
use crate::members::org_member::OrgMember;

pub struct MembersProviderService {
    data_provider: Arc<dyn MembersDataProvider>,
}

impl MembersProviderService {
    pub fn new(data_provider: Arc<dyn MembersDataProvider>) -> Self {
        Self { data_provider }
    }
}

#[async_trait]
impl MembersProvider for MembersProviderService {
    async fn list_org_members(
        &self,
        org_id: OrganizationId,
    ) -> Result<Vec<OrgMember>, ServiceError> {
        info!(org_id, "list org members requested");
        Ok(self.data_provider.list_org_members(org_id).await?)
    }
}

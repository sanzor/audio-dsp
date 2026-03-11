use async_trait::async_trait;

use crate::domain::data_error::DataError;
use crate::domain::db::db_organization::OrganizationId;
use crate::members::org_member::OrgMember;

#[async_trait]
pub trait MembersDataProvider: Send + Sync {
    async fn list_org_members(&self, org_id: OrganizationId) -> Result<Vec<OrgMember>, DataError>;
}

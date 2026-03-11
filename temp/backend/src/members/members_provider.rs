use async_trait::async_trait;

use crate::domain::db::db_organization::OrganizationId;
use crate::domain::service_error::ServiceError;
use crate::members::org_member::OrgMember;

#[async_trait]
pub trait MembersProvider: Send + Sync {
    async fn list_org_members(
        &self,
        org_id: OrganizationId,
    ) -> Result<Vec<OrgMember>, ServiceError>;
}

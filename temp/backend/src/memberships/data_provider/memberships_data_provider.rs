use async_trait::async_trait;

use crate::{
    domain::{
        data_error::DataError,
        db::{db_organization::OrganizationId, db_user::UserId},
    },
    memberships::DbMembership,
};

#[async_trait]
pub trait MembershipsDataProvider: Send + Sync {
    async fn create_membership(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<DbMembership, DataError>;
    async fn delete_membership(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<bool, DataError>;
    async fn get_membership(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<Option<DbMembership>, DataError>;
    async fn list_memberships(
        &self,
        user_id: Option<UserId>,
        org_id: Option<OrganizationId>,
    ) -> Result<Vec<DbMembership>, DataError>;
}

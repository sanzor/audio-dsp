use async_trait::async_trait;

use crate::domain::{
    data_error::DataError,
    db::{db_organization::OrganizationId, db_user::UserId},
};

#[async_trait]
pub trait AccessControlDataProvider: Send + Sync {
    async fn has_permission(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        permission_key: &str,
    ) -> Result<bool, DataError>;

    async fn permissions_for_user(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<Vec<String>, DataError>;
}

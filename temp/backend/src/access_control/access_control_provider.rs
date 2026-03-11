use async_trait::async_trait;

use crate::domain::db::{db_organization::OrganizationId, db_user::UserId};
use crate::domain::service_error::ServiceError;

#[async_trait]
pub trait AccessControlProvider: Send + Sync {
    async fn has_permission(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
        permission_key: &str,
    ) -> Result<bool, ServiceError>;

    async fn permissions_for_user(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<Vec<String>, ServiceError>;
}

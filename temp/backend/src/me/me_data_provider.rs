use async_trait::async_trait;

use crate::{
    domain::{
        data_error::DataError,
        db::{db_organization::OrganizationId, db_user::UserId},
        service_error::ServiceError,
    },
    me::{
        me_bootstrap_result::MeBootstrapResult, organization_data_result::OrganizationDataResult,
    },
};

#[async_trait]
pub trait MeProvider {
    /// 1. The "Bootstrap" call: Used immediately after login to populate AuthStore
    /// and show the Organization Switcher.
    async fn get_bootstrap_data(&self, user_id: UserId) -> Result<MeBootstrapResult, ServiceError>;

    /// 2. The "Switch" call: Used when a user selects an Org to populate OrgStore
    /// and generate the Scoped JWT.
    async fn get_membership_details(
        &self,
        user_id: UserId,
        org_id: OrganizationId,
    ) -> Result<OrganizationDataResult, ServiceError>;
}

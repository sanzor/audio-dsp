use std::sync::Arc;

use crate::membership_roles::membership_roles_provider::MembershipRolesProvider;
#[derive(Clone)]
pub struct MembershipRolesAppData {
    pub membership_roles_provider: Arc<dyn MembershipRolesProvider>,
}

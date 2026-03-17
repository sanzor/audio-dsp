use std::sync::Arc;

use super::memberships_provider::MembershipsProvider;

#[derive(Clone)]
pub struct MembershipsAppData {
    pub memberships_service: Arc<dyn MembershipsProvider>,
}

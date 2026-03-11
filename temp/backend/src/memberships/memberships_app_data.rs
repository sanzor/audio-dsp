use std::sync::Arc;

use crate::memberships::memberships_provider::MembershipsProvider;

#[derive(Clone)]
pub struct MembershipsAppData {
    pub memberships_provider: Arc<dyn MembershipsProvider>,
}

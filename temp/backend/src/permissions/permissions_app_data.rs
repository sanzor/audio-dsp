use std::sync::Arc;

use crate::permissions::permissions_provider::PermissionsProvider;

#[derive(Clone)]
pub struct PermissionsAppData {
    pub permissions_provider: Arc<dyn PermissionsProvider>,
}

use std::sync::Arc;

use crate::role_permissions::role_permissions_provider::RolePermissionsProvider;
#[derive(Clone)]
pub struct RolePermissionsAppData {
    pub role_permissions_provider: Arc<dyn RolePermissionsProvider>,
}

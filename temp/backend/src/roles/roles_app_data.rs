use std::sync::Arc;

use crate::roles::roles_provider::RolesProvider;

#[derive(Clone)]
pub struct RolesAppData {
    pub roles_provider: Arc<dyn RolesProvider>,
}

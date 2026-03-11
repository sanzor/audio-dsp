use std::sync::Arc;

use crate::members::members_provider::MembersProvider;

#[derive(Clone)]
pub struct MembersAppData {
    pub members_provider: Arc<dyn MembersProvider>,
}

use std::sync::Arc;

use super::regions_provider::RegionsProvider;

#[derive(Clone)]
pub struct RegionsAppData {
    pub regions_service: Arc<dyn RegionsProvider>,
}

use std::sync::Arc;

use super::region_sets_provider::RegionSetsProvider;

#[derive(Clone)]
pub struct RegionSetsAppData {
    pub region_sets_service: Arc<dyn RegionSetsProvider>,
}

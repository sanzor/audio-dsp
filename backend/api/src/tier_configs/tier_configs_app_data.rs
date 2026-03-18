use std::sync::Arc;
use crate::tier_configs::tier_configs_provider::TierConfigsProvider;

#[derive(Clone)]
pub struct TierConfigsAppData {
    pub tier_configs_provider: Arc<dyn TierConfigsProvider>,
}

use crate::tier_configs::tier_configs_provider::TierConfigsProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct TierConfigsAppData {
    pub tier_configs_provider: Arc<dyn TierConfigsProvider>,
}

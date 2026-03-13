use crate::me::me_data_provider::MeProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct MeAppData {
    pub me_data_provider: Arc<dyn MeProvider>,
}

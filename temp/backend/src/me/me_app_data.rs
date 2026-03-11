use crate::me::MeProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct MeAppData {
    pub me_data_provider: Arc<dyn MeProvider>,
}

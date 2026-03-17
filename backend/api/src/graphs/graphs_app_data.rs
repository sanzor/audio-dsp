use std::sync::Arc;

use super::graphs_provider::GraphsProvider;

#[derive(Clone)]
pub struct GraphsAppData {
    pub graphs_service: Arc<dyn GraphsProvider>,
}

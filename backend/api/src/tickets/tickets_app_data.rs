use super::tickets_provider::TicketsProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct TicketsAppData {
    pub tickets_service: Arc<dyn TicketsProvider>,
}

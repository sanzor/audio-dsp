use std::sync::Arc;
use super::tickets_provider::TicketsProvider;

#[derive(Clone)]
pub struct TicketsAppData {
    pub tickets_service: Arc<dyn TicketsProvider>,
}

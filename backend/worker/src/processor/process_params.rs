use crate::events::ticket_created_event::TicketCreatedEvent;

pub struct ProcessParams {
    pub event: TicketCreatedEvent,
}

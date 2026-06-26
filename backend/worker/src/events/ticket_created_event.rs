use domain::db::{ticket::db_ticket::TicketId, TransformId};

pub struct TicketCreatedEvent {
    pub ticket_id: TicketId,
    pub transform_id: TransformId,
    pub source_code: String,
}

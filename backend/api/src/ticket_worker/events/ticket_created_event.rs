use domain::db::{TransformId, ticket::db_ticket::TicketId};

pub struct TicketCreatedEvent {
    pub ticket_id: TicketId,
    pub transform_id: TransformId,
    pub source_code: String,
}

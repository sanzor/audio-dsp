use domain::db::{TransformId, db_transform_draft::TransformDraftId, ticket::db_ticket::TicketId};

pub struct TicketCreatedEvent {
    pub ticket_id: TicketId,
    pub transform_id: TransformDraftId,
    pub source_code: String,
}

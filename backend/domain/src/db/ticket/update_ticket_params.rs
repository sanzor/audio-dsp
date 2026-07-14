use crate::db::ticket::{db_ticket::TicketId, ticket_status::TicketStatus};

pub struct UpdateTicketParams {
    pub ticket_id: TicketId,
    pub status: TicketStatus,
}

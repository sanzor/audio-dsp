use domain::db::ticket::{db_resource::ResourceId, db_ticket::TicketId};

pub struct CompileResult {
    pub resource_id: ResourceId,
    pub ticket_id: TicketId,
}

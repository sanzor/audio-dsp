use serde::{Deserialize, Serialize};

use crate::{db::ticket::ticket_status::TicketStatus, domain_user::UserId};

pub type TicketId = i64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbTicket {
    pub id: TicketId,
    pub issued_by: UserId,
    pub status: TicketStatus,
    pub timestamp: i64,
}

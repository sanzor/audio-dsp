use serde::{Deserialize, Serialize};

use crate::db::ticket::db_ticket::TicketId;

pub type ResourceId = i64;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbResource {
    pub id: ResourceId,
    pub ticket_id: TicketId,
}

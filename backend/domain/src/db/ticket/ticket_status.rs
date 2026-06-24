use serde::{Deserialize, Serialize};

use crate::db::ticket::db_resource::ResourceId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketStatus {
    Processing,
    Failed,
    Successful { resource_id: ResourceId },
}

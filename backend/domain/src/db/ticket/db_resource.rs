use serde::{Deserialize, Serialize};

use crate::db::{
    ticket::db_ticket::TicketId
};

pub type ResourceId = i64;

/// Bucket 1 — "compile check". An immutable snapshot of what one specific
/// successful compile ticket produced. Never touched by save or publish;
/// history/audit trail only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbResource {
    pub id: ResourceId,
    pub ticket_id: TicketId,
    pub wasm_bytecode: Vec<u8>,
    pub name: String,
    pub description: Option<String>
}
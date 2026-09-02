use serde::{Deserialize, Serialize};

use crate::db::ticket::db_ticket::TicketId;

pub type ResourceId = i64;

/// Bucket 1 — "compile check". An immutable snapshot of what one specific
/// successful compile ticket produced. Never touched by save or publish;
/// history/audit trail only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbResource {
    pub id: ResourceId,
    pub ticket_id: TicketId,
    /// Exact source submitted with the ticket that produced this artifact.
    /// It travels with the temporary Creator-side compile package, rather
    /// than being inferred from whatever source happens to be saved later.
    pub source_code: Option<String>,
    pub wasm_bytecode: Vec<u8>,
    pub name: String,
    pub description: Option<String>,
}

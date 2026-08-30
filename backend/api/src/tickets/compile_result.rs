use domain::db::ticket::{db_resource::ResourceId, db_ticket::TicketId};

pub struct CompileResult {
    pub resource_id: ResourceId,
    pub ticket_id: TicketId,
    pub source_code: String,
    /// The compiled resource's WASM for the temporary Creator-side
    /// source/WASM package — see agents/decisions/0009-temporary-frontend-compile-package.md.
    pub wasm_bytecode: Vec<u8>,
}

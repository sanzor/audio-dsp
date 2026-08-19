use domain::db::ticket::{db_resource::ResourceId, db_ticket::TicketId};

pub struct CompileResult {
    pub resource_id: ResourceId,
    pub ticket_id: TicketId,
    /// The compiled resource's wasm bytecode, so the "Try it" preview flow
    /// can be handed a base64 encoding of it — see
    /// agents/decisions/0003-transform-preview-flow.md.
    pub wasm_bytecode: Vec<u8>,
}

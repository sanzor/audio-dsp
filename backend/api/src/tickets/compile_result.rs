use domain::db::{
    ticket::{db_resource::ResourceId, db_ticket::TicketId},
    transform_snapshot::ParamSnapshot,
};

pub struct CompileResult {
    pub resource_id: ResourceId,
    pub ticket_id: TicketId,
    /// The compiled resource's wasm bytecode, so the "Try it" preview flow
    /// can be handed a base64 encoding of it — see
    /// agents/decisions/0003-transform-preview-flow.md.
    pub wasm_bytecode: Vec<u8>,
    /// Introspected params (with default_value), so the preview flow can
    /// seed the one-node graph with the transform's real declared defaults
    /// instead of zeros — matters because zeros are a garbage stand-in for
    /// params like a filter cutoff or mix level, and the whole point of
    /// "Try it" is that preview shouldn't diverge from real usage.
    pub params: Vec<ParamSnapshot>,
}

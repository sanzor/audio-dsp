use crate::ticket_worker::processor::transform_metadata::PortMetadataJson;

/// What the compiler needs to know about each transform referenced by a
/// `Node::Primitive` or `Node::Composite` in the graph — sourced from the
/// referenced transform's own already-published metadata
/// (`TransformMetadataJson::ports`). Existence in the caller-built map
/// already implies "exists and is published" (an unpublished/deleted
/// transform_id is simply absent), so there's no separate `published` flag
/// to check here.
pub struct TransformInfo {
    /// "primitive" | "composite" — must match which `Node` variant
    /// referenced it (`build_node_ports` in `mod.rs` rejects a mismatch,
    /// e.g. a `Node::Primitive` pointing at a transform whose actual kind
    /// is "composite").
    pub kind: String,
    pub ports: Vec<PortMetadataJson>,
}

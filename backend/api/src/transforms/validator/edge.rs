use serde::{Deserialize, Serialize};

/// One wire in a composite's graph: `from_node_id`'s `from_port` (an
/// output) feeds `to_node_id`'s `to_port` (an input). `from_port`/`to_port`
/// is the fixed `"signal"` pseudo-port name (`IO_PORT_NAME`) whenever that
/// side is a `Node::Input`/`Node::Output`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Edge {
    pub from_node_id: i64,
    pub from_port: String,
    pub to_node_id: i64,
    pub to_port: String,
}

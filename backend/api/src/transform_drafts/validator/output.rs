use serde::{Deserialize, Serialize};

use super::node_position::NodePosition;

/// A pass-through marker with a single implicit `"signal"` input port
/// (`IO_PORT_NAME`), used to mark which internal wire the composite exposes
/// as one of its own external output ports, named `name`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Output {
    pub node_id: i64,
    pub name: String,
    #[serde(default)]
    pub position: NodePosition,
}

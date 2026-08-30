use serde::{Deserialize, Serialize};

/// A node's canvas position, persisted server-side purely for the frontend
/// to redraw the graph editor where the user left it — never read by
/// validation itself. Old saved graphs predating this field deserialize it
/// as `{x:0, y:0}` via `#[serde(default)]`.
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct NodePosition {
    pub x: f64,
    pub y: f64,
}

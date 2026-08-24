use domain::db::db_transform::TransformId;
use serde::{Deserialize, Serialize};

use super::node_position::NodePosition;

/// A node referencing an already-published primitive transform by
/// `transform_id`, resolved to its real ports (`build_node_ports` in
/// `mod.rs` rejects `transform_id` resolving to anything but a primitive).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Primitive {
    pub node_id: i64,
    pub transform_id: TransformId,
    #[serde(default)]
    pub position: NodePosition,
}

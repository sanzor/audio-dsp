use domain::db::db_transform::TransformId;
use serde::{Deserialize, Serialize};

use super::node_position::NodePosition;

/// A node referencing an already-published composite transform by
/// `transform_id` — validated flat, one level: resolving *that* composite's
/// own nested graph is the caller's job (via how it populates
/// `LeafTransformInfo`), not something `mod.rs` recurses into itself.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Composite {
    pub node_id: i64,
    pub transform_id: TransformId,
    #[serde(default)]
    pub position: NodePosition,
}

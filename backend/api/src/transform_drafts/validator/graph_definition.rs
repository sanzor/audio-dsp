use domain::db::db_transform::TransformId;
use serde::{Deserialize, Serialize};

use super::{edge::Edge, node::Node};

/// The whole shape a composite draft's `metadata` JSON decodes to when
/// `kind == "composite"` — what `compile_composite_metadata` parses before
/// validating.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GraphDefinition {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl GraphDefinition {
    /// Every `transform_id` referenced by a `Node::Primitive`/`Node::Composite`
    /// in this graph — what the caller needs fetched (kind + ports) before
    /// `Validator::validate` can run. `Node::Input`/`Node::Output` reference
    /// no transform.
    pub fn referenced_transform_ids(&self) -> Vec<TransformId> {
        self.nodes
            .iter()
            .filter_map(|n| match n {
                Node::Primitive(p) => Some(p.transform_id),
                Node::Composite(c) => Some(c.transform_id),
                Node::Input(_) | Node::Output(_) => None,
            })
            .collect()
    }
}

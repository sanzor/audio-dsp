use serde::{Deserialize, Serialize};

/// A port/param shape shared by the two places a compiled-but-not-yet-live
/// artifact snapshot is stored: a ticket's `transform_resource` row (bucket 1,
/// compile check) and `transform_draft` (bucket 2, save). Neither has a
/// persisted port_id/param_id — those only exist once published into
/// `transform_port`/`transform_param` (bucket 3).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortSnapshot {
    pub name: String,
    pub direction: String,
    pub port_order: i32,
    pub description: Option<String>,
    /// "program" | "sidechain". See `transform_sdk::PortKind`.
    pub kind: String,
    /// "single" | "many". See `transform_sdk::PortCardinality`.
    pub cardinality: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamSnapshot {
    pub name: String,
    pub param_order: i32,
    pub default_value: f32,
    pub min_value: Option<f32>,
    pub max_value: Option<f32>,
    pub description: Option<String>,
}

/// A composite transform's wiring — stored as the draft/live payload instead
/// of source code. Leaf nodes reference other transforms by transform_id and
/// their ports by name (never port_id, which is reassigned on every
/// republish — see composite_validator.rs). v1 has no per-node param
/// overrides.
///
/// A composite's own externally-visible ports are not a separate list —
/// they're derived from the graph's Input/Output nodes (see `CompositeNode`
/// below): an Input node's one implicit output handle wired into a leaf's
/// real input port, or a leaf's real output port wired into an Output
/// node's one implicit input handle. This replaced an earlier model where
/// exposing a port meant marking a leaf's own port in a separate
/// `exposed_ports: Vec<CompositeExposedPort>` list with no participation in
/// `edges` at all — see composite_validator.rs's module docs for why
/// collapsing that into "just another node wired by a normal edge" is a
/// real simplification, not just a UI preference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeGraphDefinition {
    pub nodes: Vec<CompositeNode>,
    pub edges: Vec<CompositeEdge>,
}

/// A node in a composite's wiring graph: either a leaf (wired to a real
/// published primitive transform) or an Input/Output boundary node (a
/// literal node type marking where one of the composite's own external
/// ports lives). Internally tagged on `node_kind`, so the JSON shape is a
/// flat `{"node_kind": "leaf", "node_id": 1, "transform_id": 3}` /
/// `{"node_kind": "input", "node_id": 5, "name": "In"}` object — kept in
/// exact lockstep with the TS mirror `CompositeNode` in
/// `frontend/src/domain/Transform/CompositeGraphDefinition.ts` (same tag
/// field name, same variant tag values, same field names per variant),
/// since `save_composite_draft`/`publish_composite_transform` deserialize
/// this directly from what the frontend sends, with no translation layer.
///
/// Named `node_kind` (not `kind`) to avoid colliding with the unrelated
/// "primitive" | "composite" `kind` field that already means something else
/// on `DbTransform`/`TransformSummaryDto` elsewhere in this domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "node_kind", rename_all = "lowercase")]
pub enum CompositeNode {
    Leaf {
        /// Canvas-local instance id — distinct from transform_id since one
        /// leaf transform can be placed as multiple instances in the same
        /// composite.
        node_id: i64,
        transform_id: i64,
    },
    Input {
        node_id: i64,
        /// The externally-visible port name this Input node produces once
        /// wired to a leaf — replaces the old
        /// `CompositeExposedPort.exposed_name`. Must be non-empty and
        /// unique across all Input/Output nodes in the graph (see
        /// composite_validator.rs).
        name: String,
    },
    Output {
        node_id: i64,
        name: String,
    },
}

impl CompositeNode {
    pub fn node_id(&self) -> i64 {
        match self {
            CompositeNode::Leaf { node_id, .. } => *node_id,
            CompositeNode::Input { node_id, .. } => *node_id,
            CompositeNode::Output { node_id, .. } => *node_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeEdge {
    pub from_node_id: i64,
    pub from_port: String,
    pub to_node_id: i64,
    pub to_port: String,
}

#[cfg(test)]
mod tests {
    //! Deserializes the exact JSON produced by
    //! `database/audio_db/migrations/0020_composite_io_nodes.up.sql` against
    //! the real "Vocal Chain" (transform_id = 32) rows in local dev
    //! Postgres, verbatim, as a lockstep check between this struct's serde
    //! shape and what's actually stored in `graph_definition` JSONB.
    use super::*;

    #[test]
    fn deserializes_the_migrated_vocal_chain_draft_row() {
        let json = r#"{"edges": [{"to_port": "In", "from_port": "Out", "to_node_id": 4, "from_node_id": 1}, {"to_port": "In", "from_port": "Out", "to_node_id": 3, "from_node_id": 4}, {"to_port": "In", "from_port": "signal", "to_node_id": 1, "from_node_id": 5}, {"to_port": "signal", "from_port": "Out", "to_node_id": 6, "from_node_id": 3}], "nodes": [{"node_id": 1, "node_kind": "leaf", "transform_id": 3}, {"node_id": 3, "node_kind": "leaf", "transform_id": 5}, {"node_id": 4, "node_kind": "leaf", "transform_id": 1}, {"name": "In", "node_id": 5, "node_kind": "input"}, {"name": "Out", "node_id": 6, "node_kind": "output"}]}"#;
        let parsed: CompositeGraphDefinition = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(parsed.nodes.len(), 5);
        assert_eq!(parsed.edges.len(), 4);
        assert!(matches!(parsed.nodes[3], CompositeNode::Input { node_id: 5, .. }));
        assert!(matches!(parsed.nodes[4], CompositeNode::Output { node_id: 6, .. }));

        // Round-trips back through our own Serialize impl too.
        let reserialized = serde_json::to_string(&parsed).expect("should reserialize");
        let reparsed: CompositeGraphDefinition = serde_json::from_str(&reserialized).expect("should reparse");
        assert_eq!(reparsed.nodes.len(), 5);
    }

    #[test]
    fn deserializes_the_migrated_vocal_chain_published_row() {
        let json = r#"{"edges": [{"to_port": "In", "from_port": "Out", "to_node_id": 2, "from_node_id": 1}, {"to_port": "In", "from_port": "Out", "to_node_id": 3, "from_node_id": 2}, {"to_port": "In", "from_port": "signal", "to_node_id": 1, "from_node_id": 4}, {"to_port": "signal", "from_port": "Out", "to_node_id": 5, "from_node_id": 3}], "nodes": [{"node_id": 1, "node_kind": "leaf", "transform_id": 3}, {"node_id": 2, "node_kind": "leaf", "transform_id": 4}, {"node_id": 3, "node_kind": "leaf", "transform_id": 5}, {"name": "In", "node_id": 4, "node_kind": "input"}, {"name": "Out", "node_id": 5, "node_kind": "output"}]}"#;
        let parsed: CompositeGraphDefinition = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(parsed.nodes.len(), 5);
        assert_eq!(parsed.edges.len(), 4);
        assert!(matches!(parsed.nodes[3], CompositeNode::Input { node_id: 4, .. }));
        assert!(matches!(parsed.nodes[4], CompositeNode::Output { node_id: 5, .. }));
    }
}

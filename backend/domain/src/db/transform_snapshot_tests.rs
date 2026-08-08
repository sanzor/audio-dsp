//! Deserializes the exact JSON produced by
//! `database/audio_db/migrations/0020_composite_io_nodes.up.sql` against
//! the real "Vocal Chain" (transform_id = 32) rows in local dev
//! Postgres, verbatim, as a lockstep check between this struct's serde
//! shape and what's actually stored in `graph_definition` JSONB.
use super::*;

#[test]
fn deserializes_the_migrated_vocal_chain_draft_row() {
    let json = r#"{"edges": [{"to_port": "In", "from_port": "Out", "to_node_id": 4, "from_node_id": 1}, {"to_port": "In", "from_port": "Out", "to_node_id": 3, "from_node_id": 4}, {"to_port": "In", "from_port": "signal", "to_node_id": 1, "from_node_id": 5}, {"to_port": "signal", "from_port": "Out", "to_node_id": 6, "from_node_id": 3}], "nodes": [{"node_id": 1, "node_kind": "leaf", "transform_id": 3}, {"node_id": 3, "node_kind": "leaf", "transform_id": 5}, {"node_id": 4, "node_kind": "leaf", "transform_id": 1}, {"name": "In", "node_id": 5, "node_kind": "input"}, {"name": "Out", "node_id": 6, "node_kind": "output"}]}"#;
    let parsed: CompositeTransformDefinition = serde_json::from_str(json).expect("should deserialize");
    assert_eq!(parsed.nodes.len(), 5);
    assert_eq!(parsed.edges.len(), 4);
    assert!(matches!(parsed.nodes[3], CompositeNode::Input { node_id: 5, .. }));
    assert!(matches!(parsed.nodes[4], CompositeNode::Output { node_id: 6, .. }));
    // None of these legacy rows have a "position" key at all — every node
    // must fall back to CompositeNodePosition::default() (0, 0) rather than
    // failing to deserialize. This is the real regression case #[serde(default)]
    // guards against once position was added to CompositeNode.
    for node in &parsed.nodes {
        assert_eq!(node.position(), CompositeNodePosition::default());
    }

    // Round-trips back through our own Serialize impl too.
    let reserialized = serde_json::to_string(&parsed).expect("should reserialize");
    let reparsed: CompositeTransformDefinition = serde_json::from_str(&reserialized).expect("should reparse");
    assert_eq!(reparsed.nodes.len(), 5);
}

#[test]
fn deserializes_the_migrated_vocal_chain_published_row() {
    let json = r#"{"edges": [{"to_port": "In", "from_port": "Out", "to_node_id": 2, "from_node_id": 1}, {"to_port": "In", "from_port": "Out", "to_node_id": 3, "from_node_id": 2}, {"to_port": "In", "from_port": "signal", "to_node_id": 1, "from_node_id": 4}, {"to_port": "signal", "from_port": "Out", "to_node_id": 5, "from_node_id": 3}], "nodes": [{"node_id": 1, "node_kind": "leaf", "transform_id": 3}, {"node_id": 2, "node_kind": "leaf", "transform_id": 4}, {"node_id": 3, "node_kind": "leaf", "transform_id": 5}, {"name": "In", "node_id": 4, "node_kind": "input"}, {"name": "Out", "node_id": 5, "node_kind": "output"}]}"#;
    let parsed: CompositeTransformDefinition = serde_json::from_str(json).expect("should deserialize");
    assert_eq!(parsed.nodes.len(), 5);
    assert_eq!(parsed.edges.len(), 4);
    assert!(matches!(parsed.nodes[3], CompositeNode::Input { node_id: 4, .. }));
    assert!(matches!(parsed.nodes[4], CompositeNode::Output { node_id: 5, .. }));
}

#[test]
fn deserializes_and_round_trips_an_explicit_position() {
    // A row saved after position support landed — "position" present on
    // every node kind, non-default values, must survive a round-trip.
    // Node 1's position is deliberately non-integer: ReactFlow's
    // screenToFlowPosition/drag deltas are always continuous floats, never
    // integers, so this is the actual shape a real Save sends — an i32
    // field here previously made every drag-then-Save fail with a 400.
    let json = r#"{"edges": [], "nodes": [
        {"node_id": 1, "node_kind": "leaf", "transform_id": 3, "position": {"x": 120.4142, "y": -40.71}},
        {"node_id": 2, "node_kind": "input", "name": "In", "position": {"x": -10, "y": 5}},
        {"node_id": 3, "node_kind": "output", "name": "Out", "position": {"x": 300, "y": 60}}
    ]}"#;
    let parsed: CompositeTransformDefinition = serde_json::from_str(json).expect("should deserialize");
    assert_eq!(parsed.nodes[0].position(), CompositeNodePosition { x: 120.4142, y: -40.71 });
    assert_eq!(parsed.nodes[1].position(), CompositeNodePosition { x: -10.0, y: 5.0 });
    assert_eq!(parsed.nodes[2].position(), CompositeNodePosition { x: 300.0, y: 60.0 });

    let reserialized = serde_json::to_string(&parsed).expect("should reserialize");
    let reparsed: CompositeTransformDefinition = serde_json::from_str(&reserialized).expect("should reparse");
    assert_eq!(reparsed.nodes[0].position(), CompositeNodePosition { x: 120.4142, y: -40.71 });
}

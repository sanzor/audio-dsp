use super::*;
use domain::db::transform_snapshot::{CompositeEdge, CompositeNode, CompositeNodePosition};

fn port(name: &str, direction: &str, kind: &str, cardinality: &str) -> DbTransformPort {
    DbTransformPort {
        port_id: 1,
        transform_id: 1,
        name: name.to_string(),
        direction: direction.to_string(),
        port_order: 0,
        description: None,
        kind: kind.to_string(),
        cardinality: cardinality.to_string(),
    }
}

fn gain_leaf() -> LeafTransformInfo {
    LeafTransformInfo {
        kind: "primitive".to_string(),
        published: true,
        ports: vec![
            port("in", "input", "program", "single"),
            port("out", "output", "program", "single"),
        ],
    }
}

fn leaf(node_id: i64, transform_id: i64) -> CompositeNode {
    CompositeNode::Leaf { node_id, transform_id, position: CompositeNodePosition::default() }
}

fn input(node_id: i64, name: &str) -> CompositeNode {
    CompositeNode::Input { node_id, name: name.to_string(), position: CompositeNodePosition::default() }
}

fn output(node_id: i64, name: &str) -> CompositeNode {
    CompositeNode::Output { node_id, name: name.to_string(), position: CompositeNodePosition::default() }
}

fn edge(from_node_id: i64, from_port: &str, to_node_id: i64, to_port: &str) -> CompositeEdge {
    CompositeEdge {
        from_node_id,
        from_port: from_port.to_string(),
        to_node_id,
        to_port: to_port.to_string(),
    }
}

#[test]
fn rejects_empty_graph() {
    let graph = CompositeTransformDefinition { nodes: vec![], edges: vec![] };
    let result = validate_composite_graph(&graph, &HashMap::new());
    assert!(result.is_err());
}

#[test]
fn rejects_unpublished_leaf() {
    let mut gain = gain_leaf();
    gain.published = false;
    let leaves = HashMap::from([(1, gain)]);
    let graph = CompositeTransformDefinition { nodes: vec![leaf(1, 1)], edges: vec![] };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_composite_leaf() {
    let mut gain = gain_leaf();
    gain.kind = "composite".to_string();
    let leaves = HashMap::from([(1, gain)]);
    let graph = CompositeTransformDefinition { nodes: vec![leaf(1, 1)], edges: vec![] };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn accepts_a_simple_chain_and_derives_io_ports() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeTransformDefinition {
        nodes: vec![leaf(1, 1), leaf(2, 1), input(3, "in"), output(4, "out")],
        edges: vec![
            edge(1, "out", 2, "in"),
            edge(3, "signal", 1, "in"),
            edge(2, "out", 4, "signal"),
        ],
    };
    let result = validate_composite_graph(&graph, &leaves);
    let ports = result.expect("expected a valid chain to validate");
    assert_eq!(ports.len(), 2);
    assert!(ports.iter().any(|p| p.name == "in" && p.direction == "input"));
    assert!(ports.iter().any(|p| p.name == "out" && p.direction == "output"));
}

#[test]
fn rejects_unconnected_program_input() {
    // "in" is a Program-kind input left completely unwired — no Input node,
    // no edge at all. Isolates the dangling-input check from every
    // Input/Output-node check below (none are exercised by this graph).
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeTransformDefinition { nodes: vec![leaf(1, 1)], edges: vec![] };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_a_second_edge_into_a_single_cardinality_leaf_input() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeTransformDefinition {
        nodes: vec![leaf(1, 1), leaf(2, 1), leaf(3, 1)],
        edges: vec![edge(1, "out", 3, "in"), edge(2, "out", 3, "in")],
    };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn requires_at_least_one_output_node() {
    // "in" is wired via a valid Input node (so the dangling-input check
    // passes), but no Output node exists anywhere in the graph — isolates
    // the "must have at least one Output node" rule from the "Output node
    // present but disconnected" case covered separately below.
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeTransformDefinition {
        nodes: vec![leaf(1, 1), input(2, "in")],
        edges: vec![edge(2, "signal", 1, "in")],
    };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_input_node_with_no_outgoing_edge() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeTransformDefinition {
        nodes: vec![leaf(1, 1), input(2, "in"), output(3, "out")],
        // Input node 2 has no outgoing edge at all — node 1's "in" is left
        // dangling too, but the Input-node check should surface first
        // since it's the more specific error.
        edges: vec![edge(1, "out", 3, "signal")],
    };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_output_node_with_no_incoming_edge() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeTransformDefinition {
        nodes: vec![leaf(1, 1), input(2, "in"), output(3, "out")],
        // Output node 3 has no incoming edge — node 1's "out" is left
        // unconnected (fine, "out" isn't Program-input-checked) but the
        // Output node itself is unusable.
        edges: vec![edge(2, "signal", 1, "in")],
    };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_output_node_with_two_incoming_edges() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeTransformDefinition {
        nodes: vec![leaf(1, 1), leaf(2, 1), input(3, "in1"), input(4, "in2"), output(5, "out")],
        edges: vec![
            edge(3, "signal", 1, "in"),
            edge(4, "signal", 2, "in"),
            edge(1, "out", 5, "signal"),
            edge(2, "out", 5, "signal"),
        ],
    };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_empty_io_node_name() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeTransformDefinition {
        nodes: vec![leaf(1, 1), input(2, ""), output(3, "out")],
        edges: vec![edge(2, "signal", 1, "in"), edge(1, "out", 3, "signal")],
    };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_duplicate_io_node_names() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeTransformDefinition {
        nodes: vec![leaf(1, 1), input(2, "shared"), output(3, "shared")],
        edges: vec![edge(2, "signal", 1, "in"), edge(1, "out", 3, "signal")],
    };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn input_node_may_fan_out_to_multiple_leaf_inputs() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeTransformDefinition {
        nodes: vec![leaf(1, 1), leaf(2, 1), input(3, "in"), output(4, "out1"), output(5, "out2")],
        edges: vec![
            edge(3, "signal", 1, "in"),
            edge(3, "signal", 2, "in"),
            edge(1, "out", 4, "signal"),
            edge(2, "out", 5, "signal"),
        ],
    };
    let result = validate_composite_graph(&graph, &leaves);
    let ports = result.expect("an Input node fanning out to two leaf inputs should be valid");
    assert_eq!(ports.len(), 3);
}

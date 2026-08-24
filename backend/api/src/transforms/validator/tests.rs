use std::collections::HashMap;

use domain::db::db_transform::TransformId;

use crate::ticket_worker::processor::transform_metadata::{DirectionJson, PortCardinalityJson, PortKindJson, PortMetadataJson};
use crate::transforms::validator::{
    composite::Composite,
    edge::Edge,
    graph_definition::GraphDefinition,
    input::Input,
    transnform_info::TransformInfo,
    node::Node,
    node_position::NodePosition,
    output::Output,
    primitive::Primitive,
    validator::{Validator, ValidatorInput},
};


fn port(name: &str, direction: DirectionJson, kind: PortKindJson, cardinality: PortCardinalityJson) -> PortMetadataJson {
    PortMetadataJson { name: name.to_string(), direction, order: 0, description: None, kind, cardinality }
}

fn gain_leaf() -> TransformInfo {
    TransformInfo {
        kind: "primitive".to_string(),
        ports: vec![
            port("in", DirectionJson::Input, PortKindJson::Program, PortCardinalityJson::Single),
            port("out", DirectionJson::Output, PortKindJson::Program, PortCardinalityJson::Single),
        ],
    }
}

fn nested_composite_leaf() -> TransformInfo {
    TransformInfo {
        kind: "composite".to_string(),
        ports: vec![
            port("in", DirectionJson::Input, PortKindJson::Program, PortCardinalityJson::Single),
            port("out", DirectionJson::Output, PortKindJson::Program, PortCardinalityJson::Single),
        ],
    }
}

fn primitive(node_id: i64, transform_id: i64) -> Node {
    Node::Primitive(Primitive { node_id, transform_id, position: NodePosition::default() })
}

fn composite(node_id: i64, transform_id: i64) -> Node {
    Node::Composite(Composite { node_id, transform_id, position: NodePosition::default() })
}

fn input(node_id: i64, name: &str) -> Node {
    Node::Input(Input { node_id, name: name.to_string(), position: NodePosition::default() })
}

fn output(node_id: i64, name: &str) -> Node {
    Node::Output(Output { node_id, name: name.to_string(), position: NodePosition::default() })
}

fn edge(from_node_id: i64, from_port: &str, to_node_id: i64, to_port: &str) -> Edge {
    Edge { from_node_id, from_port: from_port.to_string(), to_node_id, to_port: to_port.to_string() }
}

/// Serializes `graph` to JSON and runs it through `Validator::validate` —
/// exercises the real JSON entry point instead of poking at internals, so
/// these tests double as coverage of `Node`/`Edge`/`GraphDefinition`'s wire
/// format.
fn validate(graph: GraphDefinition, leaf_defs: HashMap<TransformId, TransformInfo>) -> Result<Vec<PortMetadataJson>, String> {
    let metadata_json = serde_json::to_string(&graph).expect("test graph should serialize");
    Validator::new().validate(ValidatorInput { metadata_json, leaf_defs })
}

#[test]
fn rejects_empty_graph() {
    let graph = GraphDefinition { nodes: vec![], edges: vec![] };
    let result = validate(graph, HashMap::new());
    assert!(result.is_err());
}

#[test]
fn rejects_unknown_leaf_transform() {
    // No leaf_defs entry at all — stands in for "does not exist or was
    // never published", since existence in the caller-built map is the only
    // signal this module has for either condition now.
    let graph = GraphDefinition { nodes: vec![primitive(1, 1)], edges: vec![] };
    let result = validate(graph, HashMap::new());
    assert!(result.is_err());
}

#[test]
fn rejects_primitive_node_pointing_at_a_composite_transform() {
    let leaves = HashMap::from([(1, nested_composite_leaf())]);
    let graph = GraphDefinition { nodes: vec![primitive(1, 1)], edges: vec![] };
    let result = validate(graph, leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_composite_node_pointing_at_a_primitive_transform() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = GraphDefinition { nodes: vec![composite(1, 1)], edges: vec![] };
    let result = validate(graph, leaves);
    assert!(result.is_err());
}

#[test]
fn accepts_a_composite_node_referencing_a_published_composite() {
    let leaves = HashMap::from([(1, nested_composite_leaf())]);
    let graph = GraphDefinition {
        nodes: vec![composite(1, 1), input(2, "in"), output(3, "out")],
        edges: vec![edge(2, "signal", 1, "in"), edge(1, "out", 3, "signal")],
    };
    let result = validate(graph, leaves);
    let ports = result.expect("a Composite node referencing a published composite should validate");
    assert_eq!(ports.len(), 2);
}

#[test]
fn accepts_a_simple_chain_and_derives_io_ports() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = GraphDefinition {
        nodes: vec![primitive(1, 1), primitive(2, 1), input(3, "in"), output(4, "out")],
        edges: vec![edge(1, "out", 2, "in"), edge(3, "signal", 1, "in"), edge(2, "out", 4, "signal")],
    };
    let result = validate(graph, leaves);
    let ports = result.expect("expected a valid chain to validate");
    assert_eq!(ports.len(), 2);
    assert!(ports.iter().any(|p| p.name == "in" && p.direction == DirectionJson::Input));
    assert!(ports.iter().any(|p| p.name == "out" && p.direction == DirectionJson::Output));
}

#[test]
fn rejects_unconnected_program_input() {
    // "in" is a Program-kind input left completely unwired — no Input node,
    // no edge at all. Isolates the dangling-input check from every
    // Input/Output-node check below (none are exercised by this graph).
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = GraphDefinition { nodes: vec![primitive(1, 1)], edges: vec![] };
    let result = validate(graph, leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_a_second_edge_into_a_single_cardinality_leaf_input() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = GraphDefinition {
        nodes: vec![primitive(1, 1), primitive(2, 1), primitive(3, 1)],
        edges: vec![edge(1, "out", 3, "in"), edge(2, "out", 3, "in")],
    };
    let result = validate(graph, leaves);
    assert!(result.is_err());
}

#[test]
fn requires_at_least_one_output_node() {
    // "in" is wired via a valid Input node (so the dangling-input check
    // passes), but no Output node exists anywhere in the graph — isolates
    // the "must have at least one Output node" rule from the "Output node
    // present but disconnected" case covered separately below.
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = GraphDefinition { nodes: vec![primitive(1, 1), input(2, "in")], edges: vec![edge(2, "signal", 1, "in")] };
    let result = validate(graph, leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_input_node_with_no_outgoing_edge() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = GraphDefinition {
        nodes: vec![primitive(1, 1), input(2, "in"), output(3, "out")],
        // Input node 2 has no outgoing edge at all — node 1's "in" is left
        // dangling too, but the Input-node check should surface first
        // since it's the more specific error.
        edges: vec![edge(1, "out", 3, "signal")],
    };
    let result = validate(graph, leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_output_node_with_no_incoming_edge() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = GraphDefinition {
        nodes: vec![primitive(1, 1), input(2, "in"), output(3, "out")],
        // Output node 3 has no incoming edge — node 1's "out" is left
        // unconnected (fine, "out" isn't Program-input-checked) but the
        // Output node itself is unusable.
        edges: vec![edge(2, "signal", 1, "in")],
    };
    let result = validate(graph, leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_output_node_with_two_incoming_edges() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = GraphDefinition {
        nodes: vec![primitive(1, 1), primitive(2, 1), input(3, "in1"), input(4, "in2"), output(5, "out")],
        edges: vec![
            edge(3, "signal", 1, "in"),
            edge(4, "signal", 2, "in"),
            edge(1, "out", 5, "signal"),
            edge(2, "out", 5, "signal"),
        ],
    };
    let result = validate(graph, leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_empty_io_node_name() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = GraphDefinition {
        nodes: vec![primitive(1, 1), input(2, ""), output(3, "out")],
        edges: vec![edge(2, "signal", 1, "in"), edge(1, "out", 3, "signal")],
    };
    let result = validate(graph, leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_duplicate_io_node_names() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = GraphDefinition {
        nodes: vec![primitive(1, 1), input(2, "shared"), output(3, "shared")],
        edges: vec![edge(2, "signal", 1, "in"), edge(1, "out", 3, "signal")],
    };
    let result = validate(graph, leaves);
    assert!(result.is_err());
}

#[test]
fn input_node_may_fan_out_to_multiple_leaf_inputs() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = GraphDefinition {
        nodes: vec![primitive(1, 1), primitive(2, 1), input(3, "in"), output(4, "out1"), output(5, "out2")],
        edges: vec![
            edge(3, "signal", 1, "in"),
            edge(3, "signal", 2, "in"),
            edge(1, "out", 4, "signal"),
            edge(2, "out", 5, "signal"),
        ],
    };
    let result = validate(graph, leaves);
    let ports = result.expect("an Input node fanning out to two leaf inputs should be valid");
    assert_eq!(ports.len(), 3);
}

#[test]
fn rejects_malformed_json() {
    let result = Validator::new().validate(ValidatorInput { metadata_json: "not json".to_string(), leaf_defs: HashMap::new() });
    assert!(result.is_err());
}

#[test]
fn validates_hand_written_json_directly() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let metadata_json = serde_json::json!({
        "nodes": [
            {"node_kind": "primitive", "node_id": 1, "transform_id": 1, "position": {"x": 0.0, "y": 0.0}},
            {"node_kind": "input", "node_id": 2, "name": "in", "position": {"x": 0.0, "y": 0.0}},
            {"node_kind": "output", "node_id": 3, "name": "out", "position": {"x": 0.0, "y": 0.0}}
        ],
        "edges": [
            {"from_node_id": 2, "from_port": "signal", "to_node_id": 1, "to_port": "in"},
            {"from_node_id": 1, "from_port": "out", "to_node_id": 3, "to_port": "signal"}
        ]
    })
    .to_string();

    let result = Validator::new().validate(ValidatorInput { metadata_json, leaf_defs: leaves });
    let ports = result.expect("expected a valid hand-written JSON graph to validate");
    assert_eq!(ports.len(), 2);
}

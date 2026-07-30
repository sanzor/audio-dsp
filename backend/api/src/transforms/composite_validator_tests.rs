use super::*;
use domain::db::transform_snapshot::{CompositeEdge, CompositeExposedPort, CompositeNode};

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

#[test]
fn rejects_empty_graph() {
    let graph = CompositeGraphDefinition { nodes: vec![], edges: vec![], exposed_ports: vec![] };
    let result = validate_composite_graph(&graph, &HashMap::new());
    assert!(result.is_err());
}

#[test]
fn rejects_unpublished_leaf() {
    let mut leaf = gain_leaf();
    leaf.published = false;
    let leaves = HashMap::from([(1, leaf)]);
    let graph = CompositeGraphDefinition {
        nodes: vec![CompositeNode { node_id: 1, transform_id: 1 }],
        edges: vec![],
        exposed_ports: vec![
            CompositeExposedPort { node_id: 1, port_name: "in".to_string(), exposed_name: "in".to_string() },
            CompositeExposedPort { node_id: 1, port_name: "out".to_string(), exposed_name: "out".to_string() },
        ],
    };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_composite_leaf() {
    let mut leaf = gain_leaf();
    leaf.kind = "composite".to_string();
    let leaves = HashMap::from([(1, leaf)]);
    let graph = CompositeGraphDefinition {
        nodes: vec![CompositeNode { node_id: 1, transform_id: 1 }],
        edges: vec![],
        exposed_ports: vec![],
    };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn accepts_a_simple_chain_and_derives_exposed_ports() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeGraphDefinition {
        nodes: vec![
            CompositeNode { node_id: 1, transform_id: 1 },
            CompositeNode { node_id: 2, transform_id: 1 },
        ],
        edges: vec![CompositeEdge {
            from_node_id: 1,
            from_port: "out".to_string(),
            to_node_id: 2,
            to_port: "in".to_string(),
        }],
        exposed_ports: vec![
            CompositeExposedPort { node_id: 1, port_name: "in".to_string(), exposed_name: "in".to_string() },
            CompositeExposedPort { node_id: 2, port_name: "out".to_string(), exposed_name: "out".to_string() },
        ],
    };
    let result = validate_composite_graph(&graph, &leaves);
    let ports = result.expect("expected a valid chain to validate");
    assert_eq!(ports.len(), 2);
    assert!(ports.iter().any(|p| p.name == "in" && p.direction == "input"));
    assert!(ports.iter().any(|p| p.name == "out" && p.direction == "output"));
}

#[test]
fn rejects_unconnected_unexposed_program_input() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeGraphDefinition {
        nodes: vec![CompositeNode { node_id: 1, transform_id: 1 }],
        edges: vec![],
        exposed_ports: vec![CompositeExposedPort {
            node_id: 1,
            port_name: "out".to_string(),
            exposed_name: "out".to_string(),
        }],
    };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn rejects_a_second_edge_into_a_single_cardinality_input() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeGraphDefinition {
        nodes: vec![
            CompositeNode { node_id: 1, transform_id: 1 },
            CompositeNode { node_id: 2, transform_id: 1 },
            CompositeNode { node_id: 3, transform_id: 1 },
        ],
        edges: vec![
            CompositeEdge { from_node_id: 1, from_port: "out".to_string(), to_node_id: 3, to_port: "in".to_string() },
            CompositeEdge { from_node_id: 2, from_port: "out".to_string(), to_node_id: 3, to_port: "in".to_string() },
        ],
        exposed_ports: vec![
            CompositeExposedPort { node_id: 1, port_name: "in".to_string(), exposed_name: "in1".to_string() },
            CompositeExposedPort { node_id: 2, port_name: "in".to_string(), exposed_name: "in2".to_string() },
            CompositeExposedPort { node_id: 3, port_name: "out".to_string(), exposed_name: "out".to_string() },
        ],
    };
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

#[test]
fn requires_at_least_one_exposed_output() {
    let leaves = HashMap::from([(1, gain_leaf())]);
    let graph = CompositeGraphDefinition {
        nodes: vec![CompositeNode { node_id: 1, transform_id: 1 }],
        edges: vec![],
        exposed_ports: vec![CompositeExposedPort {
            node_id: 1,
            port_name: "in".to_string(),
            exposed_name: "in".to_string(),
        }],
    };
    // "out" is left unconnected+unexposed here on purpose to isolate the
    // "no exposed output" failure — but that itself would already fail
    // first as an unconnected Program input on a *different* port only if
    // there were one; here there's exactly one input (exposed) and one
    // output (neither exposed nor connected), so the Program-input check
    // doesn't trip and this isolates the "at least one exposed output" rule.
    let result = validate_composite_graph(&graph, &leaves);
    assert!(result.is_err());
}

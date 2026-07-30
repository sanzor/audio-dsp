//! Validates a composite transform's wiring graph and derives its exposed
//! `transform_port` rows. Pure Rust, no DB/wasm access — mirrors
//! `ticket_worker::processor::metadata_introspector::validate_metadata`'s
//! style (a single validation entry point returning `Result<_, String>`,
//! unit-tested directly).
//!
//! A composite has no source code and no compiled binary; "compiling" one is
//! just this validation, run synchronously at save time (see
//! `TransformsProviderService::save_composite_draft`) — there's no cargo
//! build or wasmtime introspection step to make async, so this deliberately
//! does not go through the ticket_worker/ticket-polling machinery primitives
//! use.

use std::collections::{HashMap, HashSet};

use domain::db::{
    db_transform::{DbTransformPort, TransformId},
    transform_snapshot::{CompositeEdge, CompositeExposedPort, CompositeGraphDefinition, CompositeNode},
};

use super::data_provider::transforms_data_provider::NewTransformPort;

/// What the validator needs to know about each transform referenced as a
/// node in the composite's graph.
pub struct LeafTransformInfo {
    pub kind: String,
    pub published: bool,
    pub ports: Vec<DbTransformPort>,
}

/// node_id -> its transform's ports. Borrowed from `leaf_defs` for the
/// lifetime of one validation call.
type NodePorts<'a> = HashMap<i64, &'a Vec<DbTransformPort>>;

/// Every (node_id, port_name) touched by at least one edge, on either side.
type TouchedPorts = HashSet<(i64, String)>;

pub fn validate_composite_graph(
    graph: &CompositeGraphDefinition,
    leaf_defs: &HashMap<TransformId, LeafTransformInfo>,
) -> Result<Vec<NewTransformPort>, String> {
    if graph.nodes.is_empty() {
        return Err("a composite must contain at least one node".to_string());
    }

    let node_ports = build_node_ports(&graph.nodes, leaf_defs)?;
    let touched = validate_edges(&graph.edges, &node_ports)?;
    validate_no_dangling_program_inputs(&graph.nodes, &node_ports, &touched, &graph.exposed_ports)?;
    derive_exposed_ports(&graph.exposed_ports, &node_ports, &touched)
}

/// Checks every node references a transform that exists, is a primitive,
/// and is published — composites can only wire in other published
/// primitives in this version (finding 5 of the composite-canvas plan: no
/// composite-of-composite yet, that needs recursive graph resolution). On
/// success, returns node_id -> that transform's ports, for the rest of
/// validation to look up by name.
fn build_node_ports<'a>(
    nodes: &[CompositeNode],
    leaf_defs: &'a HashMap<TransformId, LeafTransformInfo>,
) -> Result<NodePorts<'a>, String> {
    let mut node_ports = NodePorts::new();

    for node in nodes {
        let Some(leaf) = leaf_defs.get(&node.transform_id) else {
            return Err(format!(
                "node {} references transform {} which does not exist",
                node.node_id, node.transform_id
            ));
        };
        if leaf.kind != "primitive" {
            return Err(format!(
                "node {} references transform {} which is not a primitive transform (composites can only wire in primitives in this version)",
                node.node_id, node.transform_id
            ));
        }
        if !leaf.published {
            return Err(format!(
                "node {} references transform {} which has never been published",
                node.node_id, node.transform_id
            ));
        }
        node_ports.insert(node.node_id, &leaf.ports);
    }

    Ok(node_ports)
}

/// Looks up a real port by name and direction on a node already known to
/// `node_ports` (i.e. already validated by `build_node_ports`).
fn find_port(node_ports: &NodePorts, node_id: i64, port_name: &str, want_direction: &str) -> Result<DbTransformPort, String> {
    let ports = node_ports.get(&node_id).ok_or_else(|| format!("edge references unknown node {node_id}"))?;
    ports
        .iter()
        .find(|p| p.name == port_name && p.direction == want_direction)
        .cloned()
        .ok_or_else(|| format!("node {node_id} has no {want_direction} port named '{port_name}'"))
}

/// Checks every edge connects a real output to a real input, and that no
/// `single`-cardinality input receives more than one edge. Returns every
/// (node_id, port_name) touched by any edge, so the dangling-input and
/// exposed-port checks can tell "genuinely unconnected" from "wired".
fn validate_edges(edges: &[CompositeEdge], node_ports: &NodePorts) -> Result<TouchedPorts, String> {
    let mut incoming_count: HashMap<(i64, String), usize> = HashMap::new();
    let mut touched = TouchedPorts::new();

    for edge in edges {
        find_port(node_ports, edge.from_node_id, &edge.from_port, "output")?;
        let to_port = find_port(node_ports, edge.to_node_id, &edge.to_port, "input")?;

        touched.insert((edge.from_node_id, edge.from_port.clone()));
        touched.insert((edge.to_node_id, edge.to_port.clone()));

        let count = incoming_count.entry((edge.to_node_id, edge.to_port.clone())).or_insert(0);
        *count += 1;
        if *count > 1 && to_port.cardinality == "single" {
            return Err(format!(
                "node {} port '{}' is a single-cardinality input but has more than one incoming edge",
                edge.to_node_id, edge.to_port
            ));
        }
    }

    Ok(touched)
}

/// Every unconnected, unexposed Program-kind input is a hard error — mirrors
/// `PortKind::Program`'s "fails closed" contract for a single transform's
/// own unwired ports. Sidechain inputs may stay unconnected (they resolve to
/// silence), so only Program-kind is checked here.
fn validate_no_dangling_program_inputs(
    nodes: &[CompositeNode],
    node_ports: &NodePorts,
    touched: &TouchedPorts,
    exposed_ports: &[CompositeExposedPort],
) -> Result<(), String> {
    let exposed_set: HashSet<(i64, String)> =
        exposed_ports.iter().map(|e| (e.node_id, e.port_name.clone())).collect();

    for node in nodes {
        let ports = node_ports[&node.node_id];
        for port in ports.iter().filter(|p| p.direction == "input") {
            let key = (node.node_id, port.name.clone());
            if touched.contains(&key) || exposed_set.contains(&key) {
                continue;
            }
            if port.kind == "program" {
                return Err(format!(
                    "node {} input port '{}' is a Program-kind input left unconnected and unexposed",
                    node.node_id, port.name
                ));
            }
        }
    }

    Ok(())
}

/// Turns the exposed-port mapping into the composite's own `NewTransformPort`
/// list — the same struct primitives' publish path already produces, so the
/// rest of the write path (publish, port-shape-diff) needs no changes to
/// consume it. Rejects exposing an already-connected port, a blank or
/// duplicate exposed name, and requires at least one exposed output (a
/// composite with none would be unusable — nothing could ever hear it).
fn derive_exposed_ports(
    exposed_ports: &[CompositeExposedPort],
    node_ports: &NodePorts,
    touched: &TouchedPorts,
) -> Result<Vec<NewTransformPort>, String> {
    let mut derived_ports = Vec::with_capacity(exposed_ports.len());
    let mut has_exposed_output = false;

    for (order, exposed) in exposed_ports.iter().enumerate() {
        if exposed.exposed_name.trim().is_empty() {
            return Err("an exposed port cannot have an empty name".to_string());
        }
        let key = (exposed.node_id, exposed.port_name.clone());
        if touched.contains(&key) {
            return Err(format!(
                "node {} port '{}' cannot be exposed — it already has an internal connection",
                exposed.node_id, exposed.port_name
            ));
        }
        let source_port = node_ports
            .get(&exposed.node_id)
            .ok_or_else(|| format!("exposed port references unknown node {}", exposed.node_id))?
            .iter()
            .find(|p| p.name == exposed.port_name)
            .cloned()
            .ok_or_else(|| format!("node {} has no port named '{}'", exposed.node_id, exposed.port_name))?;

        if source_port.direction == "output" {
            has_exposed_output = true;
        }
        derived_ports.push(NewTransformPort {
            name: exposed.exposed_name.clone(),
            direction: source_port.direction,
            order: order as i32,
            description: None,
            kind: source_port.kind,
            cardinality: source_port.cardinality,
        });
    }

    let mut seen_names: HashSet<&str> = HashSet::new();
    for port in &derived_ports {
        if !seen_names.insert(port.name.as_str()) {
            return Err(format!("exposed port name '{}' is used more than once", port.name));
        }
    }

    if !has_exposed_output {
        return Err("a composite must expose at least one output port".to_string());
    }

    Ok(derived_ports)
}

#[cfg(test)]
#[path = "composite_validator_tests.rs"]
mod tests;

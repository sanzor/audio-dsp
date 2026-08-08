//! Validates a composite transform's wiring graph and derives its exposed
//! `transform_port` rows. Pure Rust, no DB/wasm access — mirrors
//! `ticket_worker::processor::metadata_introspector::validate_metadata`'s
//! style (a single validation entry point returning `Result<_, String>`,
//! unit-tested directly).
//!
//! A composite has no source code and no compiled binary; "compiling" one is
//! just this validation, run synchronously by the explicit validate action
//! (see `TransformsProviderService::validate_composite_draft`) against the
//! currently-persisted graph, and independently again at publish time by
//! `TransformsProviderService::publish_transform`'s composite branch — there's
//! no cargo build or wasmtime introspection step to make async, so this
//! deliberately does not go through the ticket_worker/ticket-polling
//! machinery primitives use. As of
//! `agents/decisions/0007-composite-draft-validation-gate.md`, `save_composite_draft`
//! itself no longer calls this at all — Save persists the graph structurally
//! with no validation, so incremental wiring-in-progress can be preserved.
//!
//! As of the Input/Output node model, a composite's own externally-visible
//! ports are no longer a separate `exposed_ports` list — they're derived
//! from literal Input/Output nodes wired into the graph like any other node
//! (see `CompositeNode::Input`/`CompositeNode::Output` in
//! `transform_snapshot.rs`, and the TS mirror in
//! `frontend/src/domain/Transform/CompositeGraphDefinition.ts`). This
//! collapses what used to be two mechanisms into one: previously
//! `validate_no_dangling_program_inputs` had to check "touched OR exposed"
//! because there was no way to mark externality via a real edge. Now a
//! leaf's Program input fed by an edge from an Input node is simply
//! "touched" by a normal edge — confirmed while implementing this file, the
//! `exposed_set` union is gone with no replacement needed.

use std::collections::{HashMap, HashSet};

use domain::db::{
    db_transform::{DbTransformPort, TransformId},
    transform_snapshot::{CompositeEdge, CompositeTransformDefinition, CompositeNode},
};

use super::data_provider::transforms_data_provider::NewTransformPort;

/// What the validator needs to know about each transform referenced as a
/// leaf node in the composite's graph.
pub struct LeafTransformInfo {
    pub kind: String,
    pub published: bool,
    pub ports: Vec<DbTransformPort>,
}

/// Fixed pseudo-port name used on the single implicit handle every
/// Input/Output node exposes (an Input node's one output handle, an Output
/// node's one input handle) — the `CompositeEdge.from_port`/`.to_port`
/// literal whenever that side of the edge is an Input/Output node. Must
/// match the TS mirror's `IO_NODE_PORT_NAME` constant in
/// `frontend/src/domain/Transform/CompositeGraphDefinition.ts` exactly —
/// both sides hand-authored, not shared code across the Rust/TS boundary.
const IO_PORT_NAME: &str = "signal";

/// Resolves a node_id to what edge/port validation needs: a leaf's real
/// ports (borrowed from `leaf_defs`), or a marker that it's an Input/Output
/// node with exactly one implicit pseudo-port.
enum NodeRef<'a> {
    Leaf(&'a Vec<DbTransformPort>),
    Input,
    Output,
}

/// node_id -> what it resolves to. Borrowed from `leaf_defs` for the
/// lifetime of one validation call.
type NodePorts<'a> = HashMap<i64, NodeRef<'a>>;

/// Every (node_id, port_name) touched by at least one edge, on either side.
type TouchedPorts = HashSet<(i64, String)>;

pub fn validate_composite_graph(
    graph: &CompositeTransformDefinition,
    leaf_defs: &HashMap<TransformId, LeafTransformInfo>,
) -> Result<Vec<NewTransformPort>, String> {
    if graph.nodes.is_empty() {
        return Err("a composite must contain at least one node".to_string());
    }

    let node_ports = build_node_ports(&graph.nodes, leaf_defs)?;
    let touched = validate_edges(&graph.edges, &node_ports)?;
    validate_no_dangling_program_inputs(&graph.nodes, &node_ports, &touched)?;
    derive_io_ports(&graph.nodes, &graph.edges, &node_ports)
}

/// Checks every leaf node references a transform that exists, is a
/// primitive, and is published — composites can only wire in other
/// published primitives in this version (no composite-of-composite yet,
/// that needs recursive graph resolution). Input/Output nodes have no
/// transform to resolve; they get a bare `NodeRef` marker instead. On
/// success, returns node_id -> what the rest of validation needs to look up
/// ports by name.
fn build_node_ports<'a>(
    nodes: &[CompositeNode],
    leaf_defs: &'a HashMap<TransformId, LeafTransformInfo>,
) -> Result<NodePorts<'a>, String> {
    let mut node_ports = NodePorts::new();

    for node in nodes {
        match node {
            CompositeNode::Leaf { node_id, transform_id, .. } => {
                let Some(leaf) = leaf_defs.get(transform_id) else {
                    return Err(format!(
                        "node {node_id} references transform {transform_id} which does not exist"
                    ));
                };
                if leaf.kind != "primitive" {
                    return Err(format!(
                        "node {node_id} references transform {transform_id} which is not a primitive transform (composites can only wire in primitives in this version)"
                    ));
                }
                if !leaf.published {
                    return Err(format!(
                        "node {node_id} references transform {transform_id} which has never been published"
                    ));
                }
                node_ports.insert(*node_id, NodeRef::Leaf(&leaf.ports));
            }
            CompositeNode::Input { node_id, .. } => {
                node_ports.insert(*node_id, NodeRef::Input);
            }
            CompositeNode::Output { node_id, .. } => {
                node_ports.insert(*node_id, NodeRef::Output);
            }
        }
    }

    Ok(node_ports)
}

/// A pseudo `DbTransformPort` standing in for an Input/Output node's single
/// implicit handle — not a real DB row (`port_id`/`transform_id` are unused
/// placeholders), just enough shape for `validate_edges`/`derive_io_ports`
/// to treat it like any other port. An Input node's implicit port is
/// `Many`-cardinality output (unlimited fan-out to multiple leaf inputs, no
/// cardinality constraint, consistent with any other output port); an
/// Output node's is `Single`-cardinality input — this reuses the exact same
/// "more than one edge into a single-cardinality input" rejection
/// `validate_edges` already applies to real ports, so no bespoke "an Output
/// node can have at most one incoming edge" check is needed elsewhere.
fn io_pseudo_port(direction: &str) -> DbTransformPort {
    DbTransformPort {
        port_id: 0,
        transform_id: 0,
        name: IO_PORT_NAME.to_string(),
        direction: direction.to_string(),
        port_order: 0,
        description: None,
        kind: "program".to_string(),
        cardinality: if direction == "input" { "single" } else { "many" }.to_string(),
    }
}

/// Looks up a real (or pseudo) port by name and direction on a node already
/// known to `node_ports` (i.e. already validated by `build_node_ports`).
fn find_port(node_ports: &NodePorts, node_id: i64, port_name: &str, want_direction: &str) -> Result<DbTransformPort, String> {
    match node_ports.get(&node_id) {
        None => Err(format!("edge references unknown node {node_id}")),
        Some(NodeRef::Leaf(ports)) => ports
            .iter()
            .find(|p| p.name == port_name && p.direction == want_direction)
            .cloned()
            .ok_or_else(|| format!("node {node_id} has no {want_direction} port named '{port_name}'")),
        Some(NodeRef::Input) => {
            if want_direction == "output" && port_name == IO_PORT_NAME {
                Ok(io_pseudo_port("output"))
            } else {
                Err(format!(
                    "node {node_id} is an Input node and has no {want_direction} port named '{port_name}' (its only port is the implicit '{IO_PORT_NAME}' output)"
                ))
            }
        }
        Some(NodeRef::Output) => {
            if want_direction == "input" && port_name == IO_PORT_NAME {
                Ok(io_pseudo_port("input"))
            } else {
                Err(format!(
                    "node {node_id} is an Output node and has no {want_direction} port named '{port_name}' (its only port is the implicit '{IO_PORT_NAME}' input)"
                ))
            }
        }
    }
}

/// Checks every edge connects a real (or pseudo) output to a real (or
/// pseudo) input, and that no `single`-cardinality input receives more than
/// one edge — this is also what rejects a second edge into an Output
/// node's implicit input, since that pseudo-port is `single`-cardinality.
/// Returns every (node_id, port_name) touched by any edge, so the
/// dangling-input check can tell "genuinely unconnected" from "wired".
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

/// Every unconnected Program-kind input on a leaf node is a hard error —
/// mirrors `PortKind::Program`'s "fails closed" contract for a single
/// transform's own unwired ports. Sidechain inputs may stay unconnected
/// (they resolve to silence), so only Program-kind is checked here.
///
/// Only leaf nodes are checked here — an Input/Output node's own implicit
/// port is checked for connectivity by `derive_io_ports` instead, which
/// needs a stronger rule ("at least one edge" for Input, "exactly one" for
/// Output) than the generic "touched at all" this function applies.
/// Previously this also had to union in a separate `exposed_ports` set (a
/// leaf's Program input could be marked exposed without ever having an
/// edge) — that's gone now: an Input/Output node participates in `edges`
/// like any other node, so a leaf port fed by one is already `touched`, no
/// special case needed. See this module's top-level doc comment.
fn validate_no_dangling_program_inputs(nodes: &[CompositeNode], node_ports: &NodePorts, touched: &TouchedPorts) -> Result<(), String> {
    for node in nodes {
        let CompositeNode::Leaf { node_id, .. } = node else { continue };
        let Some(NodeRef::Leaf(ports)) = node_ports.get(node_id) else {
            unreachable!("build_node_ports always inserts a NodeRef::Leaf for every CompositeNode::Leaf")
        };
        for port in ports.iter().filter(|p| p.direction == "input") {
            let key = (*node_id, port.name.clone());
            if touched.contains(&key) {
                continue;
            }
            if port.kind == "program" {
                return Err(format!(
                    "node {node_id} input port '{}' is a Program-kind input left unconnected",
                    port.name
                ));
            }
        }
    }

    Ok(())
}

/// Walks Input/Output nodes and turns their wiring into the composite's own
/// `NewTransformPort` list — the same struct primitives' publish path
/// already produces, so the rest of the write path (publish, port-shape-
/// diff) needs no changes to consume it. Replaces the old
/// `derive_exposed_ports`, re-sourced from nodes instead of a separate
/// `exposed_ports` list (Input/Output nodes are the only source of a
/// composite's external ports now).
///
/// An Input node may fan out to multiple leaf inputs (no cardinality
/// constraint on that side) — its own derived port's `kind`/`cardinality`
/// is taken from the *first* connected edge found (graph order), since the
/// composite-level port describes one external signal regardless of how
/// many internal leaf ports it feeds; if those fanned-out leaf ports ever
/// disagree in kind/cardinality that's a modeling question this function
/// doesn't referee. An Output node is constrained to exactly one incoming
/// edge (enforced above by `validate_edges`, since its implicit input is
/// `single`-cardinality), so there's no such ambiguity on that side.
///
/// Rejects a blank or duplicate Input/Output node name, a dangling Input
/// node (zero outgoing edges) or dangling Output node (zero incoming
/// edges), and requires at least one Output node with a valid connection —
/// same "a composite with no output would be unusable" reasoning as before.
fn derive_io_ports(nodes: &[CompositeNode], edges: &[CompositeEdge], node_ports: &NodePorts) -> Result<Vec<NewTransformPort>, String> {
    let mut derived_ports = Vec::new();
    let mut has_exposed_output = false;
    let mut order = 0i32;

    for node in nodes {
        match node {
            CompositeNode::Leaf { .. } => continue,
            CompositeNode::Input { node_id, name, .. } => {
                if name.trim().is_empty() {
                    return Err(format!("Input node {node_id} cannot have an empty name"));
                }
                let edge = edges
                    .iter()
                    .find(|e| e.from_node_id == *node_id)
                    .ok_or_else(|| format!("Input node {node_id} has no outgoing edge and is unusable"))?;
                let leaf_port = find_port(node_ports, edge.to_node_id, &edge.to_port, "input")?;
                derived_ports.push(NewTransformPort {
                    name: name.clone(),
                    direction: "input".to_string(),
                    order,
                    description: None,
                    kind: leaf_port.kind,
                    cardinality: leaf_port.cardinality,
                });
                order += 1;
            }
            CompositeNode::Output { node_id, name, .. } => {
                if name.trim().is_empty() {
                    return Err(format!("Output node {node_id} cannot have an empty name"));
                }
                let edge = edges
                    .iter()
                    .find(|e| e.to_node_id == *node_id)
                    .ok_or_else(|| format!("Output node {node_id} has no incoming edge and is unusable"))?;
                let leaf_port = find_port(node_ports, edge.from_node_id, &edge.from_port, "output")?;
                has_exposed_output = true;
                derived_ports.push(NewTransformPort {
                    name: name.clone(),
                    direction: "output".to_string(),
                    order,
                    description: None,
                    kind: leaf_port.kind,
                    cardinality: leaf_port.cardinality,
                });
                order += 1;
            }
        }
    }

    let mut seen_names: HashSet<&str> = HashSet::new();
    for port in &derived_ports {
        if !seen_names.insert(port.name.as_str()) {
            return Err(format!("Input/Output node name '{}' is used more than once", port.name));
        }
    }

    if !has_exposed_output {
        return Err("a composite must have at least one Output node with a valid connection".to_string());
    }

    Ok(derived_ports)
}

#[cfg(test)]
#[path = "composite_validator_tests.rs"]
mod tests;

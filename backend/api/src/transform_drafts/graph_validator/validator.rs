//! Pure compiler for a composite transform's `metadata` JSON — the wiring
//! graph — into its derived list of externally-visible ports. No DB/wasm
//! access, no side effects: given the raw JSON string and already-fetched
//! info about every other transform it references (`LeafTransformInfo`),
//! `Validator::validate` parses, validates, and derives — the caller owns
//! fetching that info (via `ValidatorInput`) and deciding what (if
//! anything) to persist with the result.
//!
//! Mirrors `ticket_worker::processor::metadata_introspector`'s role for
//! primitives: `Validator::validate` (parse JSON, then validate) is the
//! composite-side analog of `introspect_metadata` (call the wasm export,
//! then validate).
//!
//! A composite has no source code and no compiled binary; "compiling" one is
//! just this. There's no cargo build or wasmtime introspection step to make
//! async, so this deliberately doesn't go through the ticket_worker/
//! ticket-polling machinery primitives use.
//!
//! A composite's own externally-visible ports aren't a separate list —
//! they're derived from literal `Node::Input`/`Node::Output` nodes wired
//! into the graph like any other node. A `Node::Primitive`/`Node::Composite`'s
//! Program input fed by an edge from an Input node is simply "touched" by a
//! normal edge — no separate "exposed" concept needed.

use std::collections::{HashMap, HashSet};

use domain::db::db_transform::TransformId;

use crate::ticket_worker::processor::transform_metadata::{DirectionJson, PortCardinalityJson, PortKindJson, PortMetadataJson};

use super::{edge::Edge, graph_definition::GraphDefinition, transform_info::TransformInfo, node::Node};

/// Fixed pseudo-port name used on the single implicit handle every
/// Input/Output node exposes (an Input node's one output handle, an Output
/// node's one input handle) — the `Edge.from_port`/`.to_port` literal
/// whenever that side of the edge is an Input/Output node. Must match the
/// TS mirror's `IO_NODE_PORT_NAME` constant exactly.
const IO_PORT_NAME: &str = "signal";

/// Resolves a node_id to what edge/port validation needs: a referenced
/// transform's real ports (borrowed from `ValidatorInput::leaf_defs`, for
/// either a `Node::Primitive` or `Node::Composite`), or a marker that it's
/// an Input/Output node with exactly one implicit pseudo-port.
enum NodeRef<'a> {
    Reference(&'a Vec<PortMetadataJson>),
    Input,
    Output,
}

/// node_id -> what it resolves to. Borrowed from `ValidatorInput::leaf_defs`
/// for the lifetime of one `validate` call.
type NodePorts<'a> = HashMap<i64, NodeRef<'a>>;

/// Every (node_id, port_name) touched by at least one edge, on either side.
type TouchedPorts = HashSet<(i64, String)>;

/// Everything `Validator::validate` needs: the raw composite draft
/// `metadata` JSON, and already-fetched info about every transform the
/// graph references. Fetching `leaf_defs` is a DB read — the caller's job,
/// not `Validator`'s, which is why this stays a plain data bundle rather
/// than something `Validator` fetches itself.
pub struct ValidatorInput {
    pub metadata_json: String,
    pub leaf_defs: HashMap<TransformId, TransformInfo>,
}

/// Pure, stateless compiler for a composite's wiring graph — see this
/// module's top-level doc comment.
#[derive(Debug, Default, Clone, Copy)]
pub struct Validator;

impl Validator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses `input.metadata_json` as a `GraphDefinition` and validates
    /// it, returning the composite's derived port list on success.
    pub fn validate(&self, input: ValidatorInput) -> Result<Vec<PortMetadataJson>, String> {
        let graph: GraphDefinition =
            serde_json::from_str(&input.metadata_json)
            .map_err(|e| format!("composite graph metadata is malformed JSON: {e}"))?;

        if graph.nodes.is_empty() {
            return Err("a composite must contain at least one node".to_string());
        }

        let node_ports = build_node_ports(&graph.nodes, &input.leaf_defs)?;
        let touched = validate_edges(&graph.edges, &node_ports)?;
        validate_no_dangling_program_inputs(&graph.nodes, &node_ports, &touched)?;
        derive_io_ports(&graph.nodes, &graph.edges, &node_ports)
    }
}

/// Checks every `Node::Primitive`/`Node::Composite` references a transform
/// that exists, is published, and actually is the kind that variant
/// declares. `Node::Input`/`Node::Output` have no transform to resolve;
/// they get a bare `NodeRef` marker instead. On success, returns node_id ->
/// what the rest of validation needs to look up ports by name.
fn build_node_ports<'a>(nodes: &[Node], leaf_defs: &'a HashMap<TransformId, TransformInfo>) -> Result<NodePorts<'a>, String> {
    let mut node_ports = NodePorts::new();

    for node in nodes {
        match node {
            Node::Primitive(p) => {
                let Some(leaf) = leaf_defs.get(&p.transform_id) else {
                    return Err(format!(
                        "node {} references transform {} which does not exist or has never been published",
                        p.node_id, p.transform_id
                    ));
                };
                if leaf.kind != "primitive" {
                    return Err(format!(
                        "node {} declares transform {} as a primitive, but it is actually a '{}' transform",
                        p.node_id, p.transform_id, leaf.kind
                    ));
                }
                node_ports.insert(p.node_id, NodeRef::Reference(&leaf.ports));
            }
            Node::Composite(c) => {
                let Some(leaf) = leaf_defs.get(&c.transform_id) else {
                    return Err(format!(
                        "node {} references transform {} which does not exist or has never been published",
                        c.node_id, c.transform_id
                    ));
                };
                if leaf.kind != "composite" {
                    return Err(format!(
                        "node {} declares transform {} as a composite, but it is actually a '{}' transform",
                        c.node_id, c.transform_id, leaf.kind
                    ));
                }
                node_ports.insert(c.node_id, NodeRef::Reference(&leaf.ports));
            }
            Node::Input(i) => {
                node_ports.insert(i.node_id, NodeRef::Input);
            }
            Node::Output(o) => {
                node_ports.insert(o.node_id, NodeRef::Output);
            }
        }
    }

    Ok(node_ports)
}

fn io_pseudo_port(direction: DirectionJson) -> PortMetadataJson {
    PortMetadataJson {
        name: IO_PORT_NAME.to_string(),
        direction,
        order: 0,
        description: None,
        kind: PortKindJson::Program,
        cardinality: if direction == DirectionJson::Input { PortCardinalityJson::Single } else { PortCardinalityJson::Many },
    }
}

/// Looks up a real (or pseudo) port by name and direction on a node already
/// known to `node_ports` (i.e. already validated by `build_node_ports`).
fn find_port(node_ports: &NodePorts, node_id: i64, port_name: &str, want_direction: DirectionJson) -> Result<PortMetadataJson, String> {
    match node_ports.get(&node_id) {
        None => Err(format!("edge references unknown node {node_id}")),
        Some(NodeRef::Reference(ports)) => ports
            .iter()
            .find(|p| p.name == port_name && p.direction == want_direction)
            .cloned()
            .ok_or_else(|| format!("node {node_id} has no {} port named '{port_name}'", want_direction.as_db_str())),
        Some(NodeRef::Input) => {
            if want_direction == DirectionJson::Output && port_name == IO_PORT_NAME {
                Ok(io_pseudo_port(DirectionJson::Output))
            } else {
                Err(format!(
                    "node {node_id} is an Input node and has no {} port named '{port_name}' (its only port is the implicit '{IO_PORT_NAME}' output)",
                    want_direction.as_db_str()
                ))
            }
        }
        Some(NodeRef::Output) => {
            if want_direction == DirectionJson::Input && port_name == IO_PORT_NAME {
                Ok(io_pseudo_port(DirectionJson::Input))
            } else {
                Err(format!(
                    "node {node_id} is an Output node and has no {} port named '{port_name}' (its only port is the implicit '{IO_PORT_NAME}' input)",
                    want_direction.as_db_str()
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
fn validate_edges(edges: &[Edge], node_ports: &NodePorts) -> Result<TouchedPorts, String> {
    let mut incoming_count: HashMap<(i64, String), usize> = HashMap::new();
    let mut touched = TouchedPorts::new();

    for edge in edges {
        find_port(node_ports, edge.from_node_id, &edge.from_port, DirectionJson::Output)?;
        let to_port = find_port(node_ports, edge.to_node_id, &edge.to_port, DirectionJson::Input)?;

        touched.insert((edge.from_node_id, edge.from_port.clone()));
        touched.insert((edge.to_node_id, edge.to_port.clone()));

        let count = incoming_count.entry((edge.to_node_id, edge.to_port.clone())).or_insert(0);
        *count += 1;
        if *count > 1 && to_port.cardinality == PortCardinalityJson::Single {
            return Err(format!(
                "node {} port '{}' is a single-cardinality input but has more than one incoming edge",
                edge.to_node_id, edge.to_port
            ));
        }
    }

    Ok(touched)
}

/// Every unconnected Program-kind input on a `Node::Primitive`/
/// `Node::Composite` is a hard error — mirrors `PortKindJson::Program`'s
/// "fails closed" contract for a single transform's own unwired ports.
/// Sidechain inputs may stay unconnected (they resolve to silence), so only
/// Program-kind is checked here.
///
/// Only reference nodes are checked here — an Input/Output node's own
/// implicit port is checked for connectivity by `derive_io_ports` instead,
/// which needs a stronger rule ("at least one edge" for Input, "exactly
/// one" for Output) than the generic "touched at all" this function
/// applies.
fn validate_no_dangling_program_inputs(nodes: &[Node], node_ports: &NodePorts, touched: &TouchedPorts) -> Result<(), String> {
    for node in nodes {
        let node_id = match node {
            Node::Primitive(p) => p.node_id,
            Node::Composite(c) => c.node_id,
            Node::Input(_) | Node::Output(_) => continue,
        };
        let Some(NodeRef::Reference(ports)) = node_ports.get(&node_id) else {
            unreachable!("build_node_ports always inserts a NodeRef::Reference for every Node::Primitive/Node::Composite")
        };
        for port in ports.iter().filter(|p| p.direction == DirectionJson::Input) {
            let key = (node_id, port.name.clone());
            if touched.contains(&key) {
                continue;
            }
            if port.kind == PortKindJson::Program {
                return Err(format!("node {node_id} input port '{}' is a Program-kind input left unconnected", port.name));
            }
        }
    }

    Ok(())
}

/// Walks Input/Output nodes and turns their wiring into the composite's own
/// derived port list — the same `PortMetadataJson` shape a primitive's
/// introspected metadata already uses, so any future reader of a
/// transform's ports doesn't need to special-case composites.
///
/// An Input node may fan out to multiple downstream inputs (no cardinality
/// constraint on that side) — its own derived port's `kind`/`cardinality`
/// is taken from the *first* connected edge found (graph order), since the
/// composite-level port describes one external signal regardless of how
/// many internal ports it feeds; if those fanned-out ports ever disagree in
/// kind/cardinality that's a modeling question this function doesn't
/// referee. An Output node is constrained to exactly one incoming edge
/// (enforced above by `validate_edges`, since its implicit input is
/// `single`-cardinality), so there's no such ambiguity on that side.
///
/// Rejects a blank or duplicate Input/Output node name, a dangling Input
/// node (zero outgoing edges) or dangling Output node (zero incoming
/// edges), and requires at least one Output node with a valid connection —
/// a composite with no output would be unusable.
fn derive_io_ports(nodes: &[Node], edges: &[Edge], node_ports: &NodePorts) -> Result<Vec<PortMetadataJson>, String> {
    let mut derived_ports = Vec::new();
    let mut has_exposed_output = false;
    let mut order = 0i32;

    for node in nodes {
        match node {
            Node::Primitive(_) | Node::Composite(_) => continue,
            Node::Input(i) => {
                if i.name.trim().is_empty() {
                    return Err(format!("Input node {} cannot have an empty name", i.node_id));
                }
                let edge = edges
                    .iter()
                    .find(|e| e.from_node_id == i.node_id)
                    .ok_or_else(|| format!("Input node {} has no outgoing edge and is unusable", i.node_id))?;
                let downstream_port = find_port(node_ports, edge.to_node_id, &edge.to_port, DirectionJson::Input)?;
                derived_ports.push(PortMetadataJson {
                    name: i.name.clone(),
                    direction: DirectionJson::Input,
                    order,
                    description: None,
                    kind: downstream_port.kind,
                    cardinality: downstream_port.cardinality,
                });
                order += 1;
            }
            Node::Output(o) => {
                if o.name.trim().is_empty() {
                    return Err(format!("Output node {} cannot have an empty name", o.node_id));
                }
                let edge = edges
                    .iter()
                    .find(|e| e.to_node_id == o.node_id)
                    .ok_or_else(|| format!("Output node {} has no incoming edge and is unusable", o.node_id))?;
                let upstream_port = find_port(node_ports, edge.from_node_id, &edge.from_port, DirectionJson::Output)?;
                has_exposed_output = true;
                derived_ports.push(PortMetadataJson {
                    name: o.name.clone(),
                    direction: DirectionJson::Output,
                    order,
                    description: None,
                    kind: upstream_port.kind,
                    cardinality: upstream_port.cardinality,
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
#[path = "tests.rs"]
mod tests;

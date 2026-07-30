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
/// of source code. References other transforms by transform_id and their
/// ports by name (never port_id, which is reassigned on every republish —
/// see composite_validator.rs). v1 has no per-node param overrides and no
/// exposed params; only ports are exposed outward.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeGraphDefinition {
    pub nodes: Vec<CompositeNode>,
    pub edges: Vec<CompositeEdge>,
    pub exposed_ports: Vec<CompositeExposedPort>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeNode {
    /// Canvas-local instance id — distinct from transform_id since one leaf
    /// transform can be placed as multiple instances in the same composite.
    pub node_id: i64,
    pub transform_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeEdge {
    pub from_node_id: i64,
    pub from_port: String,
    pub to_node_id: i64,
    pub to_port: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeExposedPort {
    pub node_id: i64,
    pub port_name: String,
    pub exposed_name: String,
}

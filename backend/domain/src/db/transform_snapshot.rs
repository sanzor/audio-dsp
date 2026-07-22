use serde::{Deserialize, Serialize};

/// A port/param shape shared by the two places a compiled-but-not-yet-live
/// artifact snapshot is stored: a ticket's `transform_resources` row (bucket 1,
/// compile check) and `transform_saved_state` (bucket 2, save). Neither has a
/// persisted port_id/param_id — those only exist once published into
/// `transform_ports`/`transform_params` (bucket 3).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortSnapshot {
    pub name: String,
    pub direction: String,
    pub port_order: i32,
    pub description: Option<String>,
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

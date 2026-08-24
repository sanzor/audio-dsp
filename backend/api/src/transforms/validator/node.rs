use serde::{Deserialize, Serialize};

use super::{composite::Composite, input::Input, output::Output, primitive::Primitive};

/// One node in a composite's wiring graph. Mirrors the frontend's node
/// model in `frontend/src/domain/Transform/CompositeGraphDefinition.ts`
/// (same `node_kind` tag field) — both sides hand-authored, not shared code
/// across the Rust/TS boundary, so they must be kept in lockstep by hand.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "node_kind", rename_all = "lowercase")]
pub enum Node {
    Primitive(Primitive),
    Composite(Composite),
    Input(Input),
    Output(Output),
}

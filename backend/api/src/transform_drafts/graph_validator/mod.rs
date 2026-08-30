//! DTOs for a composite transform's wiring graph, plus `Validator` — the
//! pure compiler that parses and validates that graph. See `validator.rs`
//! for the actual logic.

pub mod composite;
pub mod edge;
pub mod graph_definition;
pub mod input;
pub mod transform_info;
pub mod node;
pub mod node_position;
pub mod output;
pub mod primitive;
pub mod validator;

//! SDK for authoring audio transforms that compile to the wasm32-unknown-unknown
//! ABI expected by the editor's audio worklet (frontend/src/audio/worklet/graph-worklet.js):
//! zero-import instantiation, and exported `alloc`/`process`/`memory`.
//!
//! A transform implements [`Transform`] and calls [`export_transform!`] once to
//! generate the extern "C" surface the worklet and the backend compiler both rely on.

mod export;

#[doc(hidden)]
pub use export::__private;

use serde::Serialize;
use std::ops::Index;

/// Bumped whenever the wasm-level calling convention (`process`'s
/// signature/semantics) changes in a way callers must detect. Exported via
/// `transform_abi_version()` by [`export_transform!`]. Its *absence* on a
/// module signals "legacy" — compiled against an SDK before this export
/// existed, using the old in-place single-buffer `process(ptr, len,
/// params_ptr, params_len)` signature. Version 2 is the first array-shaped,
/// named-ports ABI (multi-input `samples: &[&[f32]]`, return-value output).
pub const TRANSFORM_ABI_VERSION: u32 = 2;

/// Contract a creator's transform struct implements. `process` runs once per
/// audio quantum on the wasm side; `metadata` is called only by the backend's
/// post-compile introspection step (and, lazily, once on the wasm side to
/// resolve named param lookups — see [`Params`]), never per-quantum.
pub trait Transform: Default {
    /// `samples` has exactly one entry per declared `Direction::Input` port,
    /// in declared order — a single-input transform (the entire pre-existing
    /// catalog) gets a slice of length 1 and reads `samples[0]`, same as
    /// before. `params` holds the transform's current parameter values, in
    /// the order declared by `metadata().params`, with named access on top
    /// (see [`Params`]).
    ///
    /// Returns this quantum's output: always exactly one array, matching the
    /// "exactly one output port" rule enforced by the backend's metadata
    /// introspection. Replaces the previous in-place `&mut [f32]` mutation —
    /// see `agents/decisions/` for why (a dedicated `output: &mut [f32]`
    /// out-parameter was judged unnecessary complexity once the wasm side
    /// already does a fresh arena `alloc()` per call).
    fn process(&mut self, samples: &[&[f32]], params: &Params<'_>) -> Vec<f32>;

    /// Static description of this transform's ports and params, used by the
    /// backend to populate the transform's catalog entry after a successful
    /// compile. Must be stable for a given build (called once per compile,
    /// and — for named param lookups — lazily once more on the wasm side;
    /// see [`export_transform!`]'s generated `process`).
    fn metadata() -> TransformMetadata;
}

/// Wraps a `process()` call's raw positional `&[f32]` params with the names
/// already declared in `metadata().params`, so a transform can look values up
/// by name without the wasm-level wire format changing at all (`callWasm`
/// still hands a flat positional `Float32Array`). `Index<usize>` is
/// implemented so existing `params[0]`-style call sites keep compiling
/// unchanged even though the trait signature moved from `&[f32]` to
/// `&Params<'_>`.
pub struct Params<'a> {
    values: &'a [f32],
    names: &'a [String],
}

impl<'a> Params<'a> {
    #[doc(hidden)]
    pub fn new(values: &'a [f32], names: &'a [String]) -> Self {
        Self { values, names }
    }

    /// Looks up a param's current value by the name declared for it in
    /// `metadata().params`. `None` if no param with that name was declared.
    pub fn named(&self, name: &str) -> Option<f32> {
        self.names
            .iter()
            .position(|n| n == name)
            .and_then(|i| self.values.get(i).copied())
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<'a> Index<usize> for Params<'a> {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.values[index]
    }
}

#[derive(Debug, Serialize)]
pub struct TransformMetadata {
    pub name: String,
    pub description: Option<String>,
    pub ports: Vec<PortMetadata>,
    pub params: Vec<ParamMetadata>,
}

#[derive(Debug, Serialize)]
pub struct PortMetadata {
    pub name: String,
    pub direction: Direction,
    pub order: i32,
    pub description: Option<String>,
    pub kind: PortKind,
    pub cardinality: PortCardinality,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Input,
    Output,
}

/// What role a port plays. Determines what an *unwired* port at that port
/// resolves to — a routing/graph-pipeline concern, not something this SDK
/// enforces itself, but the semantics are load-bearing enough to document
/// here so a transform author's choice of kind is meaningful:
///
/// - `Program`: main signal. Unwired fails closed — the graph pipeline
///   rejects the graph before `process()` ever runs, rather than silently
///   reviving the old cross-port summing behavior or passing another input
///   through in its place. See `agents/invariants.md`'s fail-closed rule.
/// - `Sidechain`: control/detector signal (e.g. a compressor's key input).
///   Unwired always resolves to a silence buffer, never an error — it's
///   expected to sometimes be empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PortKind {
    Program,
    Sidechain,
}

/// How many edges may target a port. `Many` is an explicit opt-in for the
/// narrow, legitimate case where summing multiple sources into one port is
/// actually wanted (e.g. several mic feeds into one "mix" port) — no longer
/// the implicit whole-node default every port got before this port model
/// existed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PortCardinality {
    Single,
    Many,
}

#[derive(Debug, Serialize)]
pub struct ParamMetadata {
    pub name: String,
    pub order: i32,
    pub default: f32,
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_serializes_to_the_shape_the_backend_expects() {
        let metadata = TransformMetadata {
            name: "RMS Detector".to_string(),
            description: None,
            ports: vec![
                PortMetadata {
                    name: "in".to_string(),
                    direction: Direction::Input,
                    order: 0,
                    description: None,
                    kind: PortKind::Program,
                    cardinality: PortCardinality::Single,
                },
                PortMetadata {
                    name: "out".to_string(),
                    direction: Direction::Output,
                    order: 0,
                    description: None,
                    kind: PortKind::Program,
                    cardinality: PortCardinality::Single,
                },
            ],
            params: vec![ParamMetadata {
                name: "window".to_string(),
                order: 0,
                default: 0.5,
                min: Some(0.0),
                max: Some(1.0),
                description: None,
            }],
        };

        let json = serde_json::to_value(&metadata).unwrap();
        assert_eq!(json["name"], "RMS Detector");
        assert_eq!(json["ports"][0]["direction"], "input");
        assert_eq!(json["ports"][0]["kind"], "program");
        assert_eq!(json["ports"][0]["cardinality"], "single");
        assert_eq!(json["params"][0]["default"], 0.5);
    }

    #[test]
    fn params_index_and_named_agree() {
        let names = vec!["threshold".to_string(), "ratio".to_string()];
        let values = [0.5f32, 4.0f32];
        let params = Params::new(&values, &names);

        assert_eq!(params[0], 0.5);
        assert_eq!(params[1], 4.0);
        assert_eq!(params.named("threshold"), Some(0.5));
        assert_eq!(params.named("ratio"), Some(4.0));
        assert_eq!(params.named("missing"), None);
    }
}

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

/// Contract a creator's transform struct implements. `process` runs once per
/// audio quantum on the wasm side; `metadata` is called only by the backend's
/// post-compile introspection step, never at audio time.
pub trait Transform: Default {
    /// Mutates `samples` in place. `params` holds the transform's current
    /// parameter values, in the order declared by `metadata().params`.
    fn process(&mut self, samples: &mut [f32], params: &[f32]);

    /// Static description of this transform's ports and params, used by the
    /// backend to populate the transform's catalog entry after a successful
    /// compile. Must be stable for a given build (called once per compile).
    fn metadata() -> TransformMetadata;
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
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Input,
    Output,
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
            ports: vec![PortMetadata {
                name: "in".to_string(),
                direction: Direction::Input,
                order: 0,
                description: None,
            }],
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
        assert_eq!(json["params"][0]["default"], 0.5);
    }
}

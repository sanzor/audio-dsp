use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use wasmtime::{Config, Engine, Linker, Module, Store};

pub struct TransformMetadata{
    pub ports: Vec<PortMetadataJson>,
    pub params: Vec<ParamMetadataJson>,
    pub graph: Option
}
/// Backend-side mirror of `transform_sdk::TransformMetadata`. Kept separate
/// from the SDK crate's serialize-side types since they compile for
/// different targets (native here, wasm32 there).
#[derive(Debug, Deserialize,Serialize)]
pub struct TransformMetadataJson {
    pub name: String,
    pub description: Option<String>,
    pub ports: Vec<PortMetadataJson>,
    pub params: Vec<ParamMetadataJson>,
}

#[derive(Debug, Deserialize)]
pub struct PortMetadataJson {
    pub name: String,
    pub direction: DirectionJson,
    pub order: i32,
    pub description: Option<String>,
    pub kind: PortKindJson,
    pub cardinality: PortCardinalityJson,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DirectionJson {
    Input,
    Output,
}

impl DirectionJson {
    pub fn as_db_str(&self) -> &'static str {
        match self {
            DirectionJson::Input => "input",
            DirectionJson::Output => "output",
        }
    }
}

/// Mirrors `transform_sdk::PortKind`. See that type's doc comment for the
/// unwired-port semantics (`Program` fails closed, `Sidechain` resolves to
/// silence) — this side only needs to deserialize and validate the shape.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum PortKindJson {
    Program,
    Sidechain,
}

impl PortKindJson {
    pub fn as_db_str(&self) -> &'static str {
        match self {
            PortKindJson::Program => "program",
            PortKindJson::Sidechain => "sidechain",
        }
    }
}

/// Mirrors `transform_sdk::PortCardinality`.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum PortCardinalityJson {
    Single,
    Many,
}

impl PortCardinalityJson {
    pub fn as_db_str(&self) -> &'static str {
        match self {
            PortCardinalityJson::Single => "single",
            PortCardinalityJson::Many => "many",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ParamMetadataJson {
    pub name: String,
    pub order: i32,
    pub default: f32,
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub description: Option<String>,
}

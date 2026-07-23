//! Exercises the real Rust->wasm32 compile pipeline end to end: an actual
//! `cargo build` subprocess plus wasmtime-based metadata introspection.
//! Slow (invokes the real Rust compiler) — run explicitly with
//! `cargo test -p api --test transform_compile_pipeline -- --ignored`.

use std::path::PathBuf;
use std::time::Duration;

use api::ticket_worker::processor::build_job::{compile_transform_source, BuildJobConfig};
use api::ticket_worker::processor::metadata_introspector::introspect_metadata;

const KNOWN_GOOD_SOURCE: &str = r#"
use transform_sdk::{Transform, TransformMetadata, PortMetadata, ParamMetadata, Direction, PortKind, PortCardinality, Params};

#[derive(Default)]
pub struct RmsDetector;

impl Transform for RmsDetector {
    fn process(&mut self, samples: &[&[f32]], _params: &Params<'_>) -> Vec<f32> {
        samples[0].iter().map(|s| s * 0.5).collect()
    }

    fn metadata() -> TransformMetadata {
        TransformMetadata {
            name: "RMS Detector".to_string(),
            description: None,
            ports: vec![
                PortMetadata { name: "in".to_string(), direction: Direction::Input, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
                PortMetadata { name: "out".to_string(), direction: Direction::Output, order: 0, description: None, kind: PortKind::Program, cardinality: PortCardinality::Single },
            ],
            params: vec![ParamMetadata {
                name: "window".to_string(),
                order: 0,
                default: 0.5,
                min: Some(0.0),
                max: Some(1.0),
                description: None,
            }],
        }
    }
}

transform_sdk::export_transform!(RmsDetector);
"#;

const SYNTAX_ERROR_SOURCE: &str = r#"
use transform_sdk::Transform

pub struct Broken {
"#;

const MISSING_EXPORT_MACRO_SOURCE: &str = r#"
use transform_sdk::{Transform, TransformMetadata, Params};

#[derive(Default)]
pub struct Silent;

impl Transform for Silent {
    fn process(&mut self, _samples: &[&[f32]], _params: &Params<'_>) -> Vec<f32> { vec![] }
    fn metadata() -> TransformMetadata {
        TransformMetadata { name: "Silent".to_string(), description: None, ports: vec![], params: vec![] }
    }
}
"#;

fn test_config() -> BuildJobConfig {
    let sdk_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../transform-sdk");
    let cargo_home = std::env::var("CARGO_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(std::env::var("HOME").unwrap()).join(".cargo"));

    BuildJobConfig {
        sdk_path,
        build_workdir: std::env::temp_dir().join("audio-dsp-test-builds"),
        cargo_target_dir: std::env::temp_dir().join("audio-dsp-test-cargo-target"),
        cargo_home,
        compile_timeout: Duration::from_secs(120),
        max_wasm_bytes: 5 * 1024 * 1024,
    }
}

#[tokio::test]
#[ignore]
async fn known_good_source_compiles_and_exposes_declared_metadata() {
    let config = test_config();
    let bytes = compile_transform_source(&config, 900_000_001, KNOWN_GOOD_SOURCE)
        .await
        .expect("known-good source should compile");

    let metadata = introspect_metadata(&bytes, 10_000_000).expect("metadata should introspect cleanly");

    assert_eq!(metadata.name, "RMS Detector");
    assert_eq!(metadata.ports.len(), 2);
    assert_eq!(metadata.params.len(), 1);
    assert_eq!(metadata.params[0].name, "window");
}

#[tokio::test]
#[ignore]
async fn syntax_error_fails_with_a_useful_message() {
    let config = test_config();
    let result = compile_transform_source(&config, 900_000_002, SYNTAX_ERROR_SOURCE).await;

    let err = result.expect_err("syntax error should not compile");
    assert!(!err.is_empty());
}

#[tokio::test]
#[ignore]
async fn valid_rust_missing_export_macro_fails_introspection_not_silently() {
    let config = test_config();
    let bytes = compile_transform_source(&config, 900_000_003, MISSING_EXPORT_MACRO_SOURCE)
        .await
        .expect("this source is valid Rust and should compile");

    let result = introspect_metadata(&bytes, 10_000_000);
    let err = result.expect_err("compiled module without export_transform! must fail introspection, not silently succeed");
    assert!(err.contains("transform_metadata_ptr"));
}

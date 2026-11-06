//! Exercises the real Rust->wasm32 compile pipeline end to end: an actual
//! `cargo build` subprocess plus wasmtime-based metadata introspection.
//! Slow (invokes the real Rust compiler) — run explicitly with
//! `cargo test -p api --test transform_compile_pipeline -- --ignored`.

use std::path::PathBuf;
use std::time::Duration;

use api::transform_drafts::processor::build_job::compile_transform_source;
use api::transform_drafts::processor::build_job_config::BuildJobConfig;
use api::transform_drafts::processor::wasm::wasm_parser::{parse_wasm, WasmInput};

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
    let bytes = compile_transform_source(&config, KNOWN_GOOD_SOURCE)
        .await
        .expect("known-good source should compile");

    let parsed = parse_wasm(WasmInput {
        wasm_bytes: &bytes,
        fuel_limit: 10_000_000,
    })
    .expect("metadata should introspect cleanly");

    assert_eq!(parsed.metadata.name, "RMS Detector");
    assert_eq!(parsed.metadata.ports.len(), 2);
    assert_eq!(parsed.metadata.params.len(), 1);
    assert_eq!(parsed.metadata.params[0].name, "window");
}

#[tokio::test]
#[ignore]
async fn syntax_error_fails_with_a_useful_message() {
    let config = test_config();
    let result = compile_transform_source(&config, SYNTAX_ERROR_SOURCE).await;

    let err = result.expect_err("syntax error should not compile");
    assert!(!err.is_empty());
}

#[tokio::test]
#[ignore]
async fn valid_rust_missing_export_macro_fails_introspection_not_silently() {
    let config = test_config();
    let bytes = compile_transform_source(&config, MISSING_EXPORT_MACRO_SOURCE)
        .await
        .expect("this source is valid Rust and should compile");

    let result = parse_wasm(WasmInput {
        wasm_bytes: &bytes,
        fuel_limit: 10_000_000,
    });
    let err = result.expect_err(
        "compiled module without export_transform! must fail introspection, not silently succeed",
    );
    assert!(err.contains("transform_metadata_ptr"));
}

/// Server secrets live in the API's environment. Creator source must not be
/// able to read them at compile time through `env!`/`option_env!` — the
/// compile subprocess has to start from a cleared environment.
const LEAK_PROBE_VAR: &str = "AUDIO_DSP_LEAK_PROBE";
const LEAK_PROBE_VALUE: &str = "leak-probe-sentinel-7f3a";

const ENV_ECHO_SOURCE: &str = r#"
use transform_sdk::{Transform, TransformMetadata, Params};

#[derive(Default)]
pub struct EnvEcho;

impl Transform for EnvEcho {
    fn process(&mut self, samples: &[&[f32]], _params: &Params<'_>) -> Vec<f32> {
        samples[0].to_vec()
    }
    fn metadata() -> TransformMetadata {
        TransformMetadata {
            name: option_env!("AUDIO_DSP_LEAK_PROBE").unwrap_or("absent").to_string(),
            description: None,
            ports: vec![],
            params: vec![],
        }
    }
}

transform_sdk::export_transform!(EnvEcho);
"#;

const ENV_IN_COMPILE_ERROR_SOURCE: &str = r#"
compile_error!(env!("AUDIO_DSP_LEAK_PROBE"));
"#;

#[tokio::test]
#[ignore]
async fn server_env_is_not_visible_to_option_env_in_compiled_source() {
    std::env::set_var(LEAK_PROBE_VAR, LEAK_PROBE_VALUE);
    let config = test_config();
    let bytes = compile_transform_source(&config, ENV_ECHO_SOURCE)
        .await
        .expect("env-echo source should compile");

    let parsed = parse_wasm(WasmInput {
        wasm_bytes: &bytes,
        fuel_limit: 10_000_000,
    })
    .expect("metadata should introspect cleanly");

    assert_eq!(parsed.metadata.name, "absent");
}

#[tokio::test]
#[ignore]
async fn server_env_does_not_leak_through_compile_error_output() {
    std::env::set_var(LEAK_PROBE_VAR, LEAK_PROBE_VALUE);
    let config = test_config();
    let err = compile_transform_source(&config, ENV_IN_COMPILE_ERROR_SOURCE)
        .await
        .expect_err("source using env! on an unset variable must not compile");

    assert!(
        !err.contains(LEAK_PROBE_VALUE),
        "compiler output leaked env: {err}"
    );
    assert!(
        err.contains(LEAK_PROBE_VAR),
        "expected an env!-not-defined error, got: {err}"
    );
}

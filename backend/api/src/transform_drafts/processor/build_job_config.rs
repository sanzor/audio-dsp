use std::{path::PathBuf, time::Duration};

#[derive(Clone)]
pub struct BuildJobConfig {
    /// Path to the transform-sdk crate the generated Cargo.toml depends on.
    pub sdk_path: PathBuf,
    /// Per-job scratch directories are created under here and removed after
    /// each build.
    pub build_workdir: PathBuf,
    /// Shared, persistent CARGO_TARGET_DIR so transform-sdk and its deps
    /// compile once and are reused across jobs; only the tiny per-job crate
    /// rebuilds each time.
    pub cargo_target_dir: PathBuf,
    /// Shared, pre-warmed CARGO_HOME so `--offline` never needs network.
    pub cargo_home: PathBuf,
    pub compile_timeout: Duration,
    pub max_wasm_bytes: u64,
}

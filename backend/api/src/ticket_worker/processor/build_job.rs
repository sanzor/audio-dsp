use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;

use domain::db::ticket::db_ticket::TicketId;
use tokio::process::Command;

/// Compiler output (stdout+stderr combined) is capped before being stored as
/// a ticket's failure message — rustc diagnostics can be enormous, and this
/// is meant to be genuinely readable, not a raw dump.
const MAX_ERROR_MESSAGE_BYTES: usize = 64 * 1024;

/// Config for the subprocess Rust->wasm32 build. All paths/limits are backend
/// operator settings, never user-controlled.
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

/// Compiles `source_code` as a creator transform's `src/lib.rs` against the
/// pinned transform-sdk contract, targeting wasm32-unknown-unknown.
///
/// User source is written byte-for-byte (no wrapping) so compiler error line
/// numbers match what the user actually wrote. The generated Cargo.toml is
/// entirely backend-authored — the user has no way to add dependencies.
pub async fn compile_transform_source(
    config: &BuildJobConfig,
    ticket_id: TicketId,
    source_code: &str,
) -> Result<Vec<u8>, String> {
    let job_dir = config.build_workdir.join(ticket_id.to_string());
    let src_dir = job_dir.join("src");

    tokio::fs::create_dir_all(&src_dir)
        .await
        .map_err(|e| format!("failed to create build directory: {e}"))?;

    let result = run_build(config, &job_dir, source_code).await;

    if let Err(e) = tokio::fs::remove_dir_all(&job_dir).await {
        tracing::warn!(ticket_id, error = %e, "failed to clean up build job directory");
    }

    result
}

async fn run_build(config: &BuildJobConfig, job_dir: &std::path::Path, source_code: &str) -> Result<Vec<u8>, String> {
    let cargo_toml = generate_cargo_toml(&config.sdk_path);
    tokio::fs::write(job_dir.join("Cargo.toml"), cargo_toml)
        .await
        .map_err(|e| format!("failed to write Cargo.toml: {e}"))?;
    tokio::fs::write(job_dir.join("src").join("lib.rs"), source_code)
        .await
        .map_err(|e| format!("failed to write src/lib.rs: {e}"))?;

    let mut command = Command::new("cargo");
    command
        .args([
            "build",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
            "--offline",
        ])
        .current_dir(job_dir)
        .env("CARGO_TARGET_DIR", &config.cargo_target_dir)
        .env("CARGO_HOME", &config.cargo_home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);

    let child = command
        .spawn()
        .map_err(|e| format!("failed to spawn cargo: {e}"))?;

    let output = match tokio::time::timeout(config.compile_timeout, child.wait_with_output()).await {
        Ok(Ok(output)) => output,
        Ok(Err(e)) => return Err(format!("failed to run cargo: {e}")),
        Err(_) => {
            return Err(format!(
                "compile timed out after {}s",
                config.compile_timeout.as_secs()
            ))
        }
    };

    if !output.status.success() {
        let mut combined = output.stdout;
        combined.extend_from_slice(&output.stderr);
        return Err(truncate_output(&combined));
    }

    let wasm_path = config
        .cargo_target_dir
        .join("wasm32-unknown-unknown")
        .join("release")
        .join("transform_build.wasm");

    let bytes = tokio::fs::read(&wasm_path)
        .await
        .map_err(|e| format!("build succeeded but produced no wasm artifact: {e}"))?;

    if bytes.len() as u64 > config.max_wasm_bytes {
        return Err(format!(
            "compiled wasm exceeds size limit ({} bytes > {} byte limit)",
            bytes.len(),
            config.max_wasm_bytes
        ));
    }

    Ok(bytes)
}

fn generate_cargo_toml(sdk_path: &std::path::Path) -> String {
    format!(
        r#"[package]
name = "transform_build"
version = "0.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
transform-sdk = {{ path = {sdk_path:?} }}

[profile.release]
opt-level = "s"
lto = true
panic = "abort"

[workspace]
"#
    )
}

fn truncate_output(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    if text.len() <= MAX_ERROR_MESSAGE_BYTES {
        return text.into_owned();
    }
    let mut truncated = text[..MAX_ERROR_MESSAGE_BYTES].to_string();
    truncated.push_str(&format!(
        "\n...[truncated, {} more bytes]",
        text.len() - MAX_ERROR_MESSAGE_BYTES
    ));
    truncated
}

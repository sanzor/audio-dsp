use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};

use domain::db::ticket::db_ticket::TicketId;
use tokio::process::Command;

use crate::ticket_worker::processor::build_job_config::BuildJobConfig;

/// Compiler output (stdout+stderr combined) is capped before being stored as
/// a ticket's failure message — rustc diagnostics can be enormous, and this
/// is meant to be genuinely readable, not a raw dump.
const MAX_ERROR_MESSAGE_BYTES: usize = 64 * 1024;

/// Config for the subprocess Rust->wasm32 build. All paths/limits are backend
/// operator settings, never user-controlled.

/// Writes `source_code` as a creator transform's `src/lib.rs` plus a
/// backend-authored `Cargo.toml` (the user has no way to add dependencies)
/// into a fresh job directory, and removes that directory again once `run`
/// finishes — shared setup/teardown for both a real build
/// (`compile_transform_source`) and a fast syntax/type check
/// (`check_transform_source`); only the cargo subcommand `run` invokes
/// differs between the two.
async fn with_job_dir<T>(
    config: &BuildJobConfig,
    job_id: &str,
    source_code: &str,
    run: impl AsyncFnOnce(&BuildJobConfig, &std::path::Path) -> Result<T, String>,
) -> Result<T, String> {
    let job_dir = config.build_workdir.join(job_id);
    let src_dir = job_dir.join("src");

    tokio::fs::create_dir_all(&src_dir)
        .await
        .map_err(|e| format!("failed to create build directory: {e}"))?;

    let cargo_toml = generate_cargo_toml(&config.sdk_path);
    let write_result = async {
        tokio::fs::write(job_dir.join("Cargo.toml"), cargo_toml).await?;
        tokio::fs::write(job_dir.join("src").join("lib.rs"), source_code).await
    }
    .await;

    let result = match write_result {
        Ok(()) => run(config, &job_dir).await,
        Err(e) => Err(format!("failed to write build job source: {e}")),
    };

    if let Err(e) = tokio::fs::remove_dir_all(&job_dir).await {
        tracing::warn!(job_id, error = %e, "failed to clean up build job directory");
    }

    result
}

/// Compiles `source_code` as a creator transform's `src/lib.rs` against the
/// pinned transform-sdk contract, targeting wasm32-unknown-unknown.
///
/// User source is written byte-for-byte (no wrapping) so compiler error line
/// numbers match what the user actually wrote.
pub async fn compile_transform_source(
    config: &BuildJobConfig,
    ticket_id: TicketId,
    source_code: &str,
) -> Result<Vec<u8>, String> {
    with_job_dir(
        config,
        &ticket_id.to_string(),
        source_code,
        async |config, job_dir| run_build(config, job_dir).await,
    )
    .await
}

/// A fast `cargo check` — type/borrow-checks `source_code` without codegen,
/// so it's meaningfully cheaper than `compile_transform_source` and meant to
/// be called synchronously for quick editor feedback, not through a ticket.
/// `Ok(())` means it compiles cleanly; no wasm artifact is produced or kept.
pub async fn check_transform_source(
    config: &BuildJobConfig,
    source_code: &str,
) -> Result<(), String> {
    static NEXT_JOB_ID: AtomicU64 = AtomicU64::new(0);
    let job_id = format!(
        "check-{}-{}",
        std::process::id(),
        NEXT_JOB_ID.fetch_add(1, Ordering::Relaxed)
    );

    with_job_dir(config, &job_id, source_code, async |config, job_dir| {
        run_check(config, job_dir).await
    })
    .await
}

async fn run_build(config: &BuildJobConfig, job_dir: &std::path::Path) -> Result<Vec<u8>, String> {
    run_cargo(
        config,
        job_dir,
        &[
            "build",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
            "--offline",
        ],
    )
    .await?;

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

async fn run_check(config: &BuildJobConfig, job_dir: &std::path::Path) -> Result<(), String> {
    run_cargo(
        config,
        job_dir,
        &["check", "--target", "wasm32-unknown-unknown", "--offline"],
    )
    .await
}

async fn run_cargo(
    config: &BuildJobConfig,
    job_dir: &std::path::Path,
    args: &[&str],
) -> Result<(), String> {
    let mut command = Command::new("cargo");
    command
        .args(args)
        .current_dir(job_dir)
        .env("CARGO_TARGET_DIR", &config.cargo_target_dir)
        .env("CARGO_HOME", &config.cargo_home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);

    let child = command
        .spawn()
        .map_err(|e| format!("failed to spawn cargo: {e}"))?;

    let output = match tokio::time::timeout(config.compile_timeout, child.wait_with_output()).await
    {
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

    Ok(())
}

const CARGO_TOML_TEMPLATE: &str = include_str!("templates/transform_build_cargo.toml.tpl");

fn generate_cargo_toml(sdk_path: &std::path::Path) -> String {
    CARGO_TOML_TEMPLATE.replace("__SDK_PATH__", &format!("{sdk_path:?}"))
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

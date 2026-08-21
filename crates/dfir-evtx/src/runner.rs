use anyhow::{Context, Result};
use shared_types::OutputEvent;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub async fn run_evtx_dump(binary_path: &str, evtx_path: &str, critical_only: bool) -> Result<()> {
    let mut cmd = Command::new(binary_path);
    cmd.arg("-f").arg(evtx_path).arg("-o").arg("json");

    if critical_only {
        cmd.arg("--level").arg("critical");
    }

    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to spawn EVTX parser binary at: {}", binary_path))?;

    let stdout = child.stdout.take().context("Failed to open stdout pipe")?;
    let mut reader = BufReader::new(stdout).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        if line.trim().is_empty() {
            continue;
        }

        let payload = match serde_json::from_str::<serde_json::Value>(&line) {
            Ok(parsed) => parsed,
            Err(_) => serde_json::json!({ "raw_record": line }),
        };

        let event = OutputEvent::new("dfir-evtx", "RECORD", payload);
        event.print_ndjson();
    }

    let status = child.wait().await?;
    if !status.success() {
        anyhow::bail!("EVTX parser exited with non-zero code: {}", status);
    }

    Ok(())
}

use anyhow::{Context, Result};
use shared_types::OutputEvent;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub async fn run_evtx_dump(binary_path: &str, evtx_path: &str) -> Result<()> {
    let mut child = Command::new(binary_path)
        .arg("-f")
        .arg(evtx_path)
        .arg("-o")
        .arg("json")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to spawn process: {}", binary_path))?;

    let stdout = child.stdout.take().context("Failed to capture stdout")?;
    let mut reader = BufReader::new(stdout).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        let event = OutputEvent::new("dfir-evtx", "INFO", serde_json::json!({ "raw_log": line }));
        event.print_ndjson();
    }

    let status = child.wait().await?;
    if !status.success() {
        anyhow::bail!("Process exited with status: {}", status);
    }

    Ok(())
}

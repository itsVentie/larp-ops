use anyhow::{Context, Result};
use shared_types::OutputEvent;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub async fn run_port_scan(binary_path: &str, target: &str, ports: &str) -> Result<()> {
    let mut cmd = Command::new(binary_path);
    cmd.arg("-p").arg(ports).arg(target);

    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to spawn scanner binary at: {}", binary_path))?;

    let stdout = child.stdout.take().context("Failed to open stdout pipe")?;
    let mut reader = BufReader::new(stdout).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        if line.trim().is_empty() {
            continue;
        }

        let payload = match serde_json::from_str::<serde_json::Value>(&line) {
            Ok(parsed) => parsed,
            Err(_) => serde_json::json!({
                "target": target,
                "raw_output": line
            }),
        };

        let event = OutputEvent::new("recon-net", "DISCOVERY", payload);
        event.print_ndjson();
    }

    let status = child.wait().await?;
    if !status.success() {
        anyhow::bail!("Network scanner exited with status: {}", status);
    }

    Ok(())
}

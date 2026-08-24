use anyhow::{Context, Result};
use axum::{
    body::Body,
    extract::Json,
    http::StatusCode,
    response::Response,
    routing::{get, post},
    Router,
};
use shared_types::OutputEvent;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub async fn run_agent_server(bind_addr: &str) -> Result<()> {
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/execute", post(handle_execute));

    let addr: SocketAddr = bind_addr
        .parse()
        .with_context(|| format!("Invalid bind address: {}", bind_addr))?;

    println!("[*] RAEM Agent listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_execute(Json(playbook_yaml): Json<serde_json::Value>) -> Response {
    let (tx, rx) = mpsc::channel::<Result<String, axum::Error>>(32);

    tokio::spawn(async move {
        let start_event = OutputEvent::new(
            "remote-agent",
            "INFO",
            serde_json::json!({
                "status": "received_playbook",
                "payload": playbook_yaml
            }),
        );

        let mut line = serde_json::to_string(&start_event).unwrap_or_default();
        line.push('\n');
        let _ = tx.send(Ok(line)).await;

        let finished_event = OutputEvent::new(
            "remote-agent",
            "INFO",
            serde_json::json!({ "status": "completed" }),
        );
        let mut line_end = serde_json::to_string(&finished_event).unwrap_or_default();
        line_end.push('\n');
        let _ = tx.send(Ok(line_end)).await;
    });

    let stream = ReceiverStream::new(rx);
    Response::builder()
        .header("Content-Type", "application/x-ndjson")
        .body(Body::from_stream(stream))
        .unwrap_or_else(|_| Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap())
}

pub async fn run_remote_client(agent_url: &str, playbook_file: &str) -> Result<()> {
    let yaml_content = std::fs::read_to_string(playbook_file)
        .with_context(|| format!("Failed to read playbook file: {}", playbook_file))?;

    let parsed_yaml: serde_json::Value = serde_yaml::from_str(&yaml_content)
        .with_context(|| "Failed to parse YAML playbook")?;

    let client = reqwest::Client::new();
    let url = format!("{}/execute", agent_url.trim_end_matches('/'));

    let mut response = client
        .post(&url)
        .json(&parsed_yaml)
        .send()
        .await
        .with_context(|| format!("Failed to connect to agent at {}", url))?;

    while let Some(chunk) = response.chunk().await? {
        let text = String::from_utf8_lossy(&chunk);
        print!("{}", text);
    }

    Ok(())
}

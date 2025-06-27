use axum::{
    Json, Router,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use serde_json::Value;
use tokio::net::TcpListener;
use tracing::{info, trace, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let port = std::env::var("BEES_WEBHOOK_PORT")?;

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/webhook", post(webhook));

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    info!("Server starting on port {port}");

    Ok(axum::serve(listener, app).await?)
}

async fn health_check() -> StatusCode {
    trace!("Health check OK");
    StatusCode::OK
}

async fn webhook(headers: HeaderMap, Json(_raw): Json<Value>) -> StatusCode {
    trace!("Received webhook request");

    let event = headers
        .get("X-GitHub-Event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    match event {
        "issue_comment" => todo!(),
        "pull_request_review" => todo!(),
        invalid => {
            warn!("Ignoring GitHub event: `{invalid}`");
            StatusCode::NO_CONTENT
        }
    }
}

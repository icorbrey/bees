use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use octokit_rs::webhook::IssueCommentCreated;
use tokio::net::TcpListener;
use tracing::trace;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let port = std::env::var("BEES_WEBHOOK_PORT")?;

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/new-work", post(ingest_new_work));

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    info!("Server starting on port {port}");

    Ok(axum::serve(listener, app).await?)
}

async fn health_check() -> StatusCode {
    trace!("Health check OK");
    StatusCode::OK
}

async fn ingest_new_work(Json(_payload): Json<IssueCommentCreated>) -> StatusCode {
    trace!("Ingesting new work");

    // TODO: Validate webhook delivery
    // TODO: Handle webhook delivery

    StatusCode::OK
}

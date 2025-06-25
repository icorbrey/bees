use axum::{
    Router,
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use github::GitHubClient;
use github_webhook::payload_types::{IssueCommentCreatedEvent, IssueCommentEvent};
use tokio::net::TcpListener;
use tracing::{info, trace, warn};

mod github;
mod health;
mod webhook;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // TODO: Inject port value via env variable
    let port = 3000;

    let state = AppState {
        github: GitHubClient::init()?,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/webhook", post(webhook))
        .with_state(state);

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    info!("Server starting on port {port}");

    Ok(axum::serve(listener, app).await?)
}

async fn health_check() -> StatusCode {
    trace!("Health check OK");
    StatusCode::OK
}

#[axum::debug_handler]
async fn webhook(State(state): State<AppState>, headers: HeaderMap, body: Bytes) -> StatusCode {
    trace!("Received webhook request");

    let event = headers
        .get("X-GitHub-Event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    match event {
        "ping" => {
            info!("Received ping");
            StatusCode::OK
        }
        "issue_comment" => {
            let Ok(event) = serde_json::from_slice::<IssueCommentEvent>(&body) else {
                warn!("Received bad issue comment event");
                return StatusCode::BAD_REQUEST;
            };

            let Some(action) = event.parse_action() else {
                return StatusCode::NO_CONTENT;
            };

            let result = match action {
                Action::Ping(event) => state.pong(event).await,
            };

            match result {
                Ok(status) => status,
                Err(_) => todo!(),
            }
        }
        invalid => {
            warn!("Ignoring GitHub event: `{invalid}`");
            StatusCode::NO_CONTENT
        }
    }
}

enum Action<'a> {
    Ping(&'a IssueCommentCreatedEvent<'a>),
}

const PING_SUMMON: &str = "!ping";

#[derive(Clone)]
struct AppState {
    pub github: GitHubClient,
}

impl AppState {
    async fn pong(&self, event: &IssueCommentCreatedEvent<'_>) -> anyhow::Result<StatusCode> {
        trace!("Responding to ping action");

        let owner = &event.repository.owner.login;
        let repo = &event.repository.name;
        let issue = event.issue.issue.number;

        let client = self.github.installation(owner, repo).await?;

        if let Err(e) = client
            .issues(*owner, *repo)
            .create_comment(issue as u64, "Pong!")
            .await
        {
            warn!("Failed to reply to ping action: {e:#?}");
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }

        Ok(StatusCode::OK)
    }
}

trait IssueCommentEventExt {
    fn parse_action(&self) -> Option<Action>;
}

impl IssueCommentEventExt for IssueCommentEvent<'_> {
    fn parse_action(&self) -> Option<Action<'_>> {
        match self {
            Self::Created(event) => {
                if event.comment.body.trim_start().starts_with(PING_SUMMON) {
                    return Some(Action::Ping(event));
                }

                None
            }
            _ => None,
        }
    }
}

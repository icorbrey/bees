use std::{
    collections::HashMap,
    env, fs,
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};

use anyhow::Context;
use jsonwebtoken::EncodingKey;
use octocrab::{Octocrab, OctocrabBuilder, models::AppId};
use serde::Deserialize;
use tokio::sync::Mutex;
use tracing::info;

pub struct GitHubConfig {
    /// The app ID to authenticate to GitHub with. This ID should never be
    /// logged or checked into source control.
    pub app_id: AppId,

    /// The private key to authenticate to GitHub with. This key should never
    /// be logged or checked into source control.
    pub private_key: EncodingKey,
}

impl GitHubConfig {
    const PEM_PATH: &str = "/secrets/github-private-key/key.pem";
    const APP_ID_VAR: &str = "BEES_GITHUB_APP_ID";

    /// Attempt to load the information we need to authenticate to GitHub from
    /// the environment. This will fail if any information is missing or cannot
    /// be parsed or loaded correctly.
    pub fn try_from_env() -> anyhow::Result<Self> {
        let app_id = env::var(Self::APP_ID_VAR)
            .context("GitHub app ID not defined.")
            .and_then(|id| {
                id.parse::<u64>()
                    .context("GitHub app ID must be a valid u64.")
            })
            .map(|id| AppId(id))?;

        let private_key = fs::read(Self::PEM_PATH)
            .context("Could not load GitHub private key file.")
            .and_then(|bytes| {
                EncodingKey::from_rsa_pem(bytes.as_ref())
                    .context("Could not parse GitHub private key file as RSA pem.")
            })?;

        Ok(Self {
            private_key,
            app_id,
        })
    }
}

/// Manages connections to GitHub.
///
/// Since the way you have to authenticate to GitHub as an app is really weird,
/// we authenticate globally with our app ID and private key, then maintain a
/// cache of per-installation authenticated clients. These clients expire after
/// a certain period of time, but this allows us to avoid having to
/// reauthenticate for every request (which could potentially run us up against
/// a rate limit).
#[derive(Clone)]
pub struct GitHubClient {
    /// Authenticated to GitHub as the app itself. This cannot directly
    /// interact with installations, but can be used to authenticate to them.
    app: Octocrab,

    /// A cache of clients authenticated to GitHub scoped to a specific
    /// installation. These clients are what get used to do real work, but
    /// expire after some time and have to be reauthenticated.
    installations: Arc<Mutex<HashMap<u64, Installation>>>,
}

struct Installation {
    /// Authenticated to GitHub scoped to a specific installation.
    client: Octocrab,

    /// The instant at which the scoped client expires and must be
    /// reauthenticated.
    expires_at: Instant,
}

impl GitHubClient {
    pub fn init() -> anyhow::Result<Self> {
        let config = GitHubConfig::try_from_env()?;

        let app = OctocrabBuilder::new()
            .app(config.app_id, config.private_key)
            .build()?;

        let installations = Arc::new(Mutex::new(HashMap::new()));

        Ok(GitHubClient { app, installations })
    }

    /// Returns a GitHub client scoped to an installation at `owner/repo`.
    ///
    /// If there already exists an unexpired client for the given installation,
    /// returns that client. Otherwise, (re)authenticates to GitHub and caches
    /// the client, then returns it.
    pub async fn installation(&self, owner: &str, repo: &str) -> anyhow::Result<Octocrab> {
        let inst: InstallationResponse = (self.app)
            .get(&format!("/repos/{owner}/{repo}/installation"), None::<&()>)
            .await
            .context("Could not get installation ID for `{owner}/{repo}`")?;
        let inst = inst.id;

        let mut installations = self.installations.lock().await;

        // Do we already have an unexpired client for this installation?
        if let Some(i) = installations.get(&inst) {
            if Instant::now() < i.expires_at {
                info!("Using cached installation client for `{owner}/{repo}`");
                return Ok(i.client.clone());
            }
        }

        // If not, get one.
        let token: TokenResponse = (self.app)
            .post(
                &format!("/app/installations/{inst}/access_tokens"),
                None::<&()>,
            )
            .await
            .context("Could not acquire auth token for `{owner}/{repo}`")?;

        let now_systime = SystemTime::now();
        let now_inst = Instant::now();

        let expires_at = now_inst
            + humantime::parse_rfc3339(&token.expires_at)
                .context("Could not parse auth token expiration time for `{owner/repo}`")?
                .duration_since(now_systime)
                .unwrap_or(Duration::ZERO);

        let client = OctocrabBuilder::new().personal_token(token.token).build()?;

        installations.insert(
            inst,
            Installation {
                client: client.clone(),
                expires_at,
            },
        );

        info!("Using new installation client for `{owner}/{repo}`");
        Ok(client)
    }
}

#[derive(Deserialize)]
struct InstallationResponse {
    id: u64,
}

#[derive(Deserialize)]
struct TokenResponse {
    token: String,
    expires_at: String,
}

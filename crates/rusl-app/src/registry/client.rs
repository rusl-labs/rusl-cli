use crate::config::Config;
use crate::config::credentials::Credentials;
use anyhow::{Context, Result, anyhow};
use rusl_api_client::{ApiError, RuslApiClient, SessionTokens, models};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryVersion {
    pub version: String,
    #[serde(default)]
    pub schemas: HashMap<String, String>,
    #[serde(default)]
    pub bundles: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryMetadataResponse {
    pub name: String,
    pub versions: Vec<RegistryVersion>,
}

pub struct RegistryClient {
    api: RuslApiClient,
}

impl RegistryClient {
    pub fn new(config: Config) -> Self {
        Self {
            api: RuslApiClient::new(config.api_base_url),
        }
    }

    pub async fn fetch_schema_meta(
        &self,
        account: &str,
        slug: &str,
    ) -> Result<RegistryMetadataResponse> {
        let (mut credentials, mut session) = self.load_session()?;
        let metadata = self
            .api
            .fetch_schema_metadata(&mut session, account, slug)
            .await
            .map_err(map_api_error)
            .with_context(|| format!("Failed to fetch schema metadata for {account}/{slug}"))?;

        self.persist_session(&mut credentials, &session)?;
        Ok(map_metadata_response(metadata.name, metadata.versions))
    }

    pub async fn fetch_bundle_meta(
        &self,
        account: &str,
        slug: &str,
    ) -> Result<RegistryMetadataResponse> {
        let (mut credentials, mut session) = self.load_session()?;
        let metadata = self
            .api
            .fetch_bundle_metadata(&mut session, account, slug)
            .await
            .map_err(map_api_error)
            .with_context(|| format!("Failed to fetch bundle metadata for {account}/{slug}"))?;

        self.persist_session(&mut credentials, &session)?;
        Ok(map_metadata_response(metadata.name, metadata.versions))
    }

    pub async fn download_schema_blob(
        &self,
        account: &str,
        slug: &str,
        version: &str,
    ) -> Result<Vec<u8>> {
        let (mut credentials, mut session) = self.load_session()?;
        let schema_slug_and_version = format!("{slug}@v{version}");
        let document = self
            .api
            .fetch_schema_document(&mut session, account, &schema_slug_and_version)
            .await
            .map_err(map_api_error)
            .with_context(|| format!("Failed to download schema {account}/{slug}@v{version}"))?;

        self.persist_session(&mut credentials, &session)?;
        serde_json::to_vec(&document).context("Failed to serialize raw schema document")
    }

    pub async fn fetch_me(&self) -> Result<models::MeResponse> {
        let (mut credentials, mut session) = self.load_session()?;
        let me = self
            .api
            .fetch_session_me(&mut session)
            .await
            .map_err(map_api_error)
            .context("Failed to fetch current session")?;

        self.persist_session(&mut credentials, &session)?;
        Ok(me)
    }

    fn load_session(&self) -> Result<(Option<Credentials>, SessionTokens)> {
        let credentials = Credentials::load();
        let session = session_tokens(&credentials);
        Ok((credentials, session))
    }

    fn persist_session(
        &self,
        credentials: &mut Option<Credentials>,
        session: &SessionTokens,
    ) -> Result<()> {
        if !session_has_tokens(session) {
            if credentials.is_some() {
                Credentials::clear().context("Failed to clear stored session")?;
                *credentials = None;
            }
            return Ok(());
        }

        let Some(stored) = credentials.as_mut() else {
            return Ok(());
        };

        let next_access_token = session.access_token.as_deref().unwrap_or_default();
        let next_refresh_token = session.refresh_token.as_deref().unwrap_or_default();
        if stored.access_token == next_access_token && stored.refresh_token == next_refresh_token {
            return Ok(());
        }

        stored.access_token = next_access_token.to_string();
        stored.refresh_token = next_refresh_token.to_string();
        stored.save().context("Failed to persist refreshed session")
    }
}

fn session_tokens(credentials: &Option<Credentials>) -> SessionTokens {
    match credentials {
        Some(credentials) => SessionTokens::new(
            Some(credentials.access_token.clone()),
            Some(credentials.refresh_token.clone()),
        ),
        None => SessionTokens::default(),
    }
}

fn map_metadata_response(
    name: Option<String>,
    versions: Option<Vec<models::RuslWebRawSchemaMetadataControllerShow200ResponseVersionsInner>>,
) -> RegistryMetadataResponse {
    RegistryMetadataResponse {
        name: name.unwrap_or_default(),
        versions: versions
            .unwrap_or_default()
            .into_iter()
            .map(map_version)
            .collect(),
    }
}

fn map_version(
    version: models::RuslWebRawSchemaMetadataControllerShow200ResponseVersionsInner,
) -> RegistryVersion {
    RegistryVersion {
        version: version.version.unwrap_or_default(),
        schemas: version.schemas.unwrap_or_default(),
        bundles: version.bundles.unwrap_or_default(),
    }
}

fn map_api_error(error: ApiError) -> anyhow::Error {
    anyhow!(error)
}

fn session_has_tokens(session: &SessionTokens) -> bool {
    session
        .access_token
        .as_deref()
        .is_some_and(|token| !token.trim().is_empty())
        || session
            .refresh_token
            .as_deref()
            .is_some_and(|token| !token.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::RegistryClient;
    use crate::config::{Config, credentials::Credentials};
    use axum::{
        Json, Router,
        extract::State,
        http::{HeaderMap, StatusCode},
        routing::{get, post},
    };
    use rusl_api_client::models::MeResponse;
    use serde_json::{Value, json};
    use serial_test::serial;
    use std::{collections::VecDeque, ffi::OsString, sync::Arc};
    use tempfile::TempDir;
    use tokio::{
        net::TcpListener,
        sync::{Mutex, oneshot},
        task::JoinHandle,
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct RecordedRequest {
        method: &'static str,
        path: &'static str,
        authorization: Option<String>,
    }

    #[derive(Debug, Clone)]
    struct ResponseSpec {
        status: StatusCode,
        body: Value,
    }

    impl ResponseSpec {
        fn ok(body: Value) -> Self {
            Self {
                status: StatusCode::OK,
                body,
            }
        }

        fn unauthorized() -> Self {
            Self {
                status: StatusCode::UNAUTHORIZED,
                body: json!({ "error": "unauthorized" }),
            }
        }
    }

    #[derive(Clone)]
    struct TestState {
        requests: Arc<Mutex<Vec<RecordedRequest>>>,
        me_responses: Arc<Mutex<VecDeque<ResponseSpec>>>,
        exchange_responses: Arc<Mutex<VecDeque<ResponseSpec>>>,
    }

    struct TestServer {
        base_url: String,
        requests: Arc<Mutex<Vec<RecordedRequest>>>,
        shutdown: Option<oneshot::Sender<()>>,
        task: JoinHandle<()>,
    }

    impl TestServer {
        async fn start(
            me_responses: Vec<ResponseSpec>,
            exchange_responses: Vec<ResponseSpec>,
        ) -> Self {
            let requests = Arc::new(Mutex::new(Vec::new()));
            let state = TestState {
                requests: requests.clone(),
                me_responses: Arc::new(Mutex::new(VecDeque::from(me_responses))),
                exchange_responses: Arc::new(Mutex::new(VecDeque::from(exchange_responses))),
            };

            let app = Router::new()
                .route("/api/auth/sessions/me", get(me_handler))
                .route("/api/tokens/exchange", post(exchange_handler))
                .with_state(state);

            let listener = TcpListener::bind("127.0.0.1:0")
                .await
                .expect("bind test server");
            let address = listener.local_addr().expect("read test server address");
            let (shutdown_tx, shutdown_rx) = oneshot::channel();
            let task = tokio::spawn(async move {
                axum::serve(listener, app)
                    .with_graceful_shutdown(async {
                        let _ = shutdown_rx.await;
                    })
                    .await
                    .expect("run test server");
            });

            Self {
                base_url: format!("http://{address}"),
                requests,
                shutdown: Some(shutdown_tx),
                task,
            }
        }

        async fn recorded_requests(&self) -> Vec<RecordedRequest> {
            self.requests.lock().await.clone()
        }
    }

    impl Drop for TestServer {
        fn drop(&mut self) {
            if let Some(shutdown) = self.shutdown.take() {
                let _ = shutdown.send(());
            }
            self.task.abort();
        }
    }

    struct HomeGuard {
        previous_home: Option<OsString>,
    }

    impl HomeGuard {
        fn new(home_dir: &std::path::Path) -> Self {
            let previous_home = std::env::var_os(home_var_name());
            set_env_var(home_var_name(), home_dir.as_os_str());
            Self { previous_home }
        }
    }

    impl Drop for HomeGuard {
        fn drop(&mut self) {
            restore_env_var(home_var_name(), self.previous_home.as_ref());
        }
    }

    #[tokio::test]
    #[serial]
    async fn persists_refreshed_access_token_after_fetching_me() {
        let server = TestServer::start(
            vec![
                ResponseSpec::unauthorized(),
                ResponseSpec::ok(json!({ "authenticated": false })),
            ],
            vec![ResponseSpec::ok(json!({ "access_token": "fresh-access" }))],
        )
        .await;
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = HomeGuard::new(temp_dir.path());
        Credentials {
            access_token: "stale-access".to_string(),
            refresh_token: "refresh-token".to_string(),
        }
        .save()
        .expect("save credentials");

        let client = RegistryClient::new(Config {
            api_base_url: server.base_url.clone(),
            website_url: "https://example.test".to_string(),
            schema_dir: None,
            generators: Default::default(),
        });

        let me = client.fetch_me().await.expect("fetch me");

        assert!(matches!(
            me,
            MeResponse::MeResponseUnauthenticated1(ref body) if !body.authenticated
        ));
        let stored = Credentials::load().expect("reloaded credentials");
        assert_eq!(stored.access_token, "fresh-access");
        assert_eq!(stored.refresh_token, "refresh-token");
        assert_eq!(
            server.recorded_requests().await,
            vec![
                RecordedRequest {
                    method: "GET",
                    path: "/api/auth/sessions/me",
                    authorization: Some("Bearer stale-access".to_string()),
                },
                RecordedRequest {
                    method: "POST",
                    path: "/api/tokens/exchange",
                    authorization: Some("Bearer refresh-token".to_string()),
                },
                RecordedRequest {
                    method: "GET",
                    path: "/api/auth/sessions/me",
                    authorization: Some("Bearer fresh-access".to_string()),
                },
            ]
        );
    }

    #[tokio::test]
    #[serial]
    async fn clears_stored_credentials_when_refresh_fails_before_fetching_me() {
        let server = TestServer::start(
            vec![ResponseSpec::ok(json!({ "authenticated": false }))],
            vec![ResponseSpec::unauthorized()],
        )
        .await;
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = HomeGuard::new(temp_dir.path());
        Credentials {
            access_token: "".to_string(),
            refresh_token: "expired-refresh".to_string(),
        }
        .save()
        .expect("save credentials");

        let client = RegistryClient::new(Config {
            api_base_url: server.base_url.clone(),
            website_url: "https://example.test".to_string(),
            schema_dir: None,
            generators: Default::default(),
        });

        let me = client.fetch_me().await.expect("fetch me");

        assert!(matches!(
            me,
            MeResponse::MeResponseUnauthenticated1(ref body) if !body.authenticated
        ));
        assert!(Credentials::load().is_none());
        assert_eq!(
            server.recorded_requests().await,
            vec![
                RecordedRequest {
                    method: "POST",
                    path: "/api/tokens/exchange",
                    authorization: Some("Bearer expired-refresh".to_string()),
                },
                RecordedRequest {
                    method: "GET",
                    path: "/api/auth/sessions/me",
                    authorization: None,
                },
            ]
        );
    }

    async fn me_handler(
        State(state): State<TestState>,
        headers: HeaderMap,
    ) -> (StatusCode, Json<Value>) {
        record_request(&state, "GET", "/api/auth/sessions/me", &headers).await;
        let response = next_response(&state.me_responses).await;
        (response.status, Json(response.body))
    }

    async fn exchange_handler(
        State(state): State<TestState>,
        headers: HeaderMap,
    ) -> (StatusCode, Json<Value>) {
        record_request(&state, "POST", "/api/tokens/exchange", &headers).await;
        let response = next_response(&state.exchange_responses).await;
        (response.status, Json(response.body))
    }

    async fn record_request(
        state: &TestState,
        method: &'static str,
        path: &'static str,
        headers: &HeaderMap,
    ) {
        state.requests.lock().await.push(RecordedRequest {
            method,
            path,
            authorization: headers
                .get("authorization")
                .and_then(|value| value.to_str().ok())
                .map(ToOwned::to_owned),
        });
    }

    async fn next_response(queue: &Arc<Mutex<VecDeque<ResponseSpec>>>) -> ResponseSpec {
        queue
            .lock()
            .await
            .pop_front()
            .expect("test server received more requests than expected")
    }

    #[cfg(windows)]
    fn home_var_name() -> &'static str {
        "USERPROFILE"
    }

    #[cfg(not(windows))]
    fn home_var_name() -> &'static str {
        "HOME"
    }

    fn set_env_var<K, V>(key: K, value: V)
    where
        K: AsRef<std::ffi::OsStr>,
        V: AsRef<std::ffi::OsStr>,
    {
        unsafe { std::env::set_var(key, value) }
    }

    fn restore_env_var(key: &str, value: Option<&OsString>) {
        match value {
            Some(value) => unsafe { std::env::set_var(key, value) },
            None => unsafe { std::env::remove_var(key) },
        }
    }
}

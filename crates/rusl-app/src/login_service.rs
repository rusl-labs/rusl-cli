use crate::config::{self, credentials::Credentials};
use anyhow::{Context, Result, bail};
use base64::prelude::*;
use rand::Rng;
use rusl_api_client::{ApiError, RuslApiClient, SessionTokens, models};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginSession {
    pub browser_url: String,
    pub logo_url: String,
    pub code_verifier: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginCallbackOutcome {
    Success,
    MissingCode,
    ExchangeFailed,
}

impl LoginSession {
    pub fn callback_page_html(&self, outcome: LoginCallbackOutcome) -> String {
        let title = match outcome {
            LoginCallbackOutcome::Success => "Login successful",
            LoginCallbackOutcome::MissingCode | LoginCallbackOutcome::ExchangeFailed => {
                "Login failed"
            }
        };
        let message = match outcome {
            LoginCallbackOutcome::Success => {
                "You can safely close this browser window and return to your terminal."
            }
            LoginCallbackOutcome::MissingCode => {
                "We could not read the authorization code. Please return to your terminal and try again."
            }
            LoginCallbackOutcome::ExchangeFailed => {
                "The CLI could not verify your credentials. Please return to your terminal for details."
            }
        };

        format!(
            "<html><body style='margin:0;background:#0f172a;color:#e2e8f0;font-family:-apple-system,BlinkMacSystemFont,\"Segoe UI\",sans-serif'><div style='min-height:100vh;display:flex;align-items:center;justify-content:center;padding:24px'><div style='max-width:480px;width:100%;text-align:center;background:#111827;border:1px solid rgba(148,163,184,0.2);border-radius:20px;padding:32px 24px;box-shadow:0 20px 45px rgba(15,23,42,0.35)'><img src='{logo_url}' alt='Rusl' style='width:200px;height:200px;max-width:200px;max-height:200px;object-fit:contain;margin:0 auto 16px;display:block' /><h1 style='font-size:28px;line-height:1.2;margin:0 0 12px;color:#f8fafc'>{title}</h1><p style='margin:0;color:#94a3b8;font-size:16px;line-height:1.5'>{message}</p></div></div><script>setTimeout(()=>window.close(), 3000)</script></body></html>",
            logo_url = self.logo_url,
            title = title,
            message = message,
        )
    }
}

pub fn begin_login(callback_url: &str) -> Result<LoginSession> {
    let config = config::load().context("Failed to load network configuration")?;
    let website_url = config.website_url.trim_end_matches('/').to_string();

    let mut random_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut random_bytes);
    let code_verifier = BASE64_URL_SAFE_NO_PAD.encode(random_bytes);

    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let code_challenge = BASE64_URL_SAFE_NO_PAD.encode(hasher.finalize());

    let encoded_callback = callback_url.replace(":", "%3A").replace("/", "%2F");
    let browser_url = format!(
        "{website_url}/login/cli?callback_url={encoded_callback}&code_challenge={code_challenge}&code_challenge_method=S256"
    );

    Ok(LoginSession {
        browser_url,
        logo_url: format!("{website_url}/logo.png"),
        code_verifier,
    })
}

pub async fn complete_login(code: String, code_verifier: String) -> Result<()> {
    if code.trim().is_empty() {
        bail!("Login failed: no authorization code was received from the browser.");
    }

    let config = config::load().context("Failed to load network configuration")?;
    let client = RuslApiClient::new(config.api_base_url);
    let payload = client
        .exchange_cli_token(code, code_verifier)
        .await
        .context("Failed to exchange the browser code for CLI credentials")?;
    let access_token = match payload
        .access_token
        .filter(|token| !token.trim().is_empty())
    {
        Some(access_token) => access_token,
        None => client
            .exchange_refresh_token(&payload.refresh_token)
            .await
            .context("Failed to exchange the refresh token for an access token")?,
    };

    Credentials {
        access_token,
        refresh_token: payload.refresh_token,
    }
    .save()
    .context("Failed to save credentials")
}

/// Authenticate with a long-lived service account bearer token.
///
/// The token is validated against the registry before it is stored. Service
/// account tokens do not use refresh exchange, so `refresh_token` is left empty.
pub async fn login_with_token(token: impl Into<String>) -> Result<()> {
    let token = token.into().trim().to_string();
    if token.is_empty() {
        bail!("Login failed: token is empty.");
    }

    let config = config::load().context("Failed to load network configuration")?;
    let client = RuslApiClient::new(config.api_base_url);
    let mut session = SessionTokens::new(Some(token.clone()), None);
    let me = match client.fetch_session_me(&mut session).await {
        Ok(me) => me,
        Err(ApiError::Unauthorized) => {
            bail!("Login failed: the token was rejected by the registry.");
        }
        Err(error) => {
            return Err(error).context("Failed to verify the service account token");
        }
    };

    match me {
        models::MeResponse::MeResponseAuthenticated1(_) => {}
        models::MeResponse::MeResponseUnauthenticated1(_) => {
            bail!("Login failed: the token was rejected by the registry.");
        }
    }

    Credentials {
        access_token: token,
        refresh_token: String::new(),
    }
    .save()
    .context("Failed to save credentials")
}

#[cfg(test)]
mod tests {
    use super::{begin_login, login_with_token};
    use crate::config::credentials::Credentials;
    use axum::{
        Json, Router,
        extract::State,
        http::{HeaderMap, StatusCode},
        routing::get,
    };
    use serde_json::{Value, json};
    use serial_test::serial;
    use std::{
        collections::VecDeque,
        ffi::OsString,
        path::{Path, PathBuf},
        sync::Arc,
    };
    use tempfile::TempDir;
    use tokio::{
        net::TcpListener,
        sync::{Mutex as AsyncMutex, oneshot},
        task::JoinHandle,
    };

    #[test]
    fn begin_login_embeds_callback_and_pkce_parameters() {
        let session = begin_login("http://127.0.0.1:4123/callback").expect("build login session");

        assert!(
            session
                .browser_url
                .contains("callback_url=http%3A%2F%2F127.0.0.1%3A4123%2Fcallback")
        );
        assert!(session.browser_url.contains("code_challenge="));
        assert!(session.browser_url.contains("code_challenge_method=S256"));
        assert!(session.logo_url.ends_with("/logo.png"));
        assert!(!session.code_verifier.is_empty());
    }

    #[tokio::test]
    #[serial]
    async fn login_with_token_saves_access_token_when_me_is_authenticated() {
        let server = TestMeServer::start(vec![ResponseSpec::ok(json!({
            "authenticated": true,
            "authentication_type": "api_key",
            "invitations": [],
            "service_account": {
                "api_key_id": "c236847e-a84a-42a0-88bc-09a271bb3c24"
            },
            "user": {
                "__typename": "users",
                "name": "Local Agent",
                "guid": "users.user_bot",
                "id": "user_bot",
                "inserted_at": "2026-04-05T00:00:00Z",
                "owning_account_slug": "dan",
                "principal_type": "service",
                "updated_at": "2026-04-05T00:00:00Z",
                "user_type": "unknown"
            },
            "accounts": {}
        }))])
        .await;
        let temp_dir = TempDir::new().expect("create temp dir");
        let _env = TestEnvGuard::new(temp_dir.path(), &server.base_url);

        login_with_token("sa-long-lived-token")
            .await
            .expect("login with token");

        let credentials = Credentials::load().expect("saved credentials");
        assert_eq!(credentials.access_token, "sa-long-lived-token");
        assert_eq!(credentials.refresh_token, "");

        let requests = server.recorded_requests().await;
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].path, "/api/v1/auth/sessions/me");
        assert_eq!(
            requests[0].authorization.as_deref(),
            Some("Bearer sa-long-lived-token")
        );
    }

    #[tokio::test]
    #[serial]
    async fn login_with_token_rejects_unauthenticated_session() {
        let server =
            TestMeServer::start(vec![ResponseSpec::ok(json!({ "authenticated": false }))]).await;
        let temp_dir = TempDir::new().expect("create temp dir");
        let _env = TestEnvGuard::new(temp_dir.path(), &server.base_url);

        let error = login_with_token("bad-token")
            .await
            .expect_err("login should fail");

        assert!(error.to_string().contains("token was rejected"));
        assert!(Credentials::load().is_none());
    }

    #[tokio::test]
    #[serial]
    async fn login_with_token_rejects_empty_token() {
        let error = login_with_token("   ")
            .await
            .expect_err("empty token should fail");
        assert!(error.to_string().contains("token is empty"));
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
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct RecordedRequest {
        path: &'static str,
        authorization: Option<String>,
    }

    #[derive(Clone)]
    struct TestState {
        requests: Arc<AsyncMutex<Vec<RecordedRequest>>>,
        me_responses: Arc<AsyncMutex<VecDeque<ResponseSpec>>>,
    }

    struct TestMeServer {
        base_url: String,
        requests: Arc<AsyncMutex<Vec<RecordedRequest>>>,
        shutdown: Option<oneshot::Sender<()>>,
        task: JoinHandle<()>,
    }

    impl TestMeServer {
        async fn start(me_responses: Vec<ResponseSpec>) -> Self {
            let requests = Arc::new(AsyncMutex::new(Vec::new()));
            let state = TestState {
                requests: requests.clone(),
                me_responses: Arc::new(AsyncMutex::new(VecDeque::from(me_responses))),
            };

            let app = Router::new()
                .route("/api/v1/auth/sessions/me", get(me_handler))
                .with_state(state);

            let listener = TcpListener::bind("127.0.0.1:0")
                .await
                .expect("bind me test server");
            let address = listener.local_addr().expect("read me test server address");
            let (shutdown_tx, shutdown_rx) = oneshot::channel();
            let task = tokio::spawn(async move {
                axum::serve(listener, app)
                    .with_graceful_shutdown(async {
                        let _ = shutdown_rx.await;
                    })
                    .await
                    .expect("run me test server");
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

    impl Drop for TestMeServer {
        fn drop(&mut self) {
            if let Some(shutdown) = self.shutdown.take() {
                let _ = shutdown.send(());
            }
            self.task.abort();
        }
    }

    struct TestEnvGuard {
        previous_home: Option<OsString>,
        previous_xdg_config_home: Option<OsString>,
        previous_xdg_data_home: Option<OsString>,
        previous_api_url: Option<OsString>,
        previous_website_url: Option<OsString>,
        previous_dir: PathBuf,
    }

    impl TestEnvGuard {
        fn new(home_dir: &Path, api_base_url: &str) -> Self {
            let previous_home = std::env::var_os(home_var_name());
            let previous_xdg_config_home = std::env::var_os("XDG_CONFIG_HOME");
            let previous_xdg_data_home = std::env::var_os("XDG_DATA_HOME");
            let previous_api_url = std::env::var_os("RUSL_API_URL");
            let previous_website_url = std::env::var_os("RUSL_WEBSITE_URL");
            let previous_dir = std::env::current_dir().expect("current dir");

            set_env_var(home_var_name(), home_dir.as_os_str());
            set_env_var("XDG_CONFIG_HOME", home_dir.join(".config"));
            set_env_var("XDG_DATA_HOME", home_dir.join(".local").join("share"));
            set_env_var("RUSL_API_URL", api_base_url);
            set_env_var("RUSL_WEBSITE_URL", api_base_url);
            std::env::set_current_dir(home_dir).expect("set workspace dir");

            Self {
                previous_home,
                previous_xdg_config_home,
                previous_xdg_data_home,
                previous_api_url,
                previous_website_url,
                previous_dir,
            }
        }
    }

    impl Drop for TestEnvGuard {
        fn drop(&mut self) {
            restore_env_var(home_var_name(), self.previous_home.as_ref());
            restore_env_var("XDG_CONFIG_HOME", self.previous_xdg_config_home.as_ref());
            restore_env_var("XDG_DATA_HOME", self.previous_xdg_data_home.as_ref());
            restore_env_var("RUSL_API_URL", self.previous_api_url.as_ref());
            restore_env_var("RUSL_WEBSITE_URL", self.previous_website_url.as_ref());
            std::env::set_current_dir(&self.previous_dir).expect("restore current dir");
        }
    }

    async fn me_handler(
        State(state): State<TestState>,
        headers: HeaderMap,
    ) -> (StatusCode, Json<Value>) {
        state.requests.lock().await.push(RecordedRequest {
            path: "/api/v1/auth/sessions/me",
            authorization: headers
                .get("authorization")
                .and_then(|value| value.to_str().ok())
                .map(ToOwned::to_owned),
        });
        let response = state
            .me_responses
            .lock()
            .await
            .pop_front()
            .expect("unexpected me request");
        (response.status, Json(response.body))
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

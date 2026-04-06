use crate::cli::LoginArgs;
use anyhow::{Context, Result};
use colored::Colorize;
use rusl_app::login_service;
use tokio::net::TcpListener;

pub async fn run(_args: LoginArgs) -> Result<()> {
    run_with_browser_opener(_args, webbrowser::open).await
}

async fn run_with_browser_opener<F>(_args: LoginArgs, open_browser: F) -> Result<()>
where
    F: Fn(&str) -> std::io::Result<()>,
{
    let pb = crate::ui::spinner("Starting login...");
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("Failed to bind a local callback port")?;

    let local_port = listener.local_addr()?.port();
    let callback_url = format!("http://127.0.0.1:{}/callback", local_port);
    let session = login_service::begin_login(&callback_url)?;

    pb.set_message("Opening your browser to authenticate...");

    if open_browser(&session.browser_url).is_err() {
        pb.println(format!(
            "{} Failed to open your browser automatically. Open this URL instead: {}",
            "Warning:".yellow().bold(),
            session.browser_url
        ));
    }

    pb.set_message(format!(
        "Waiting for browser redirect on port {}...",
        local_port
    ));

    let (stream, _) = listener
        .accept()
        .await
        .context("Browser callback listener closed unexpectedly")?;

    stream.readable().await?;
    let mut buffer = [0; 4096];
    let mut code = String::new();

    match stream.try_read(&mut buffer) {
        Ok(0) => pb.println(format!(
            "{} Timed out waiting for callback.",
            "Warning:".yellow().bold()
        )),
        Ok(n) => {
            let request_string = String::from_utf8_lossy(&buffer[..n]);

            if let Some(first_line) = request_string.lines().next()
                && let Some(query_start) = first_line.find("?code=")
            {
                let block = &first_line[query_start + 6..];
                let end_idx = block
                    .find('&')
                    .or_else(|| block.find(' '))
                    .unwrap_or(block.len());
                code = block[..end_idx].to_string();
            }
        }
        Err(e) => pb.println(format!(
            "{} Connection error: {}",
            "Warning:".yellow().bold(),
            e
        )),
    }

    let response = format!(
        "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: text/html\r\n\r\n{}",
        session.callback_page_html(!code.is_empty())
    );
    let _ = stream.try_write(response.as_bytes());

    drop(stream);
    drop(listener);

    pb.set_message("Verifying credentials...");
    login_service::complete_login(code, session.code_verifier).await?;

    pb.finish_with_message(format!(
        "{} Logged in successfully!",
        "Success:".green().bold()
    ));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run_with_browser_opener;
    use crate::cli::LoginArgs;
    use axum::{
        Json, Router,
        body::Bytes,
        extract::State,
        http::{HeaderMap, StatusCode},
        routing::post,
    };
    use rusl_app::config::credentials::Credentials;
    use serial_test::serial;
    use std::{
        collections::VecDeque,
        ffi::OsString,
        path::{Path, PathBuf},
        sync::{Arc, Mutex},
        time::Duration,
    };
    use tempfile::TempDir;
    use tokio::{
        fs,
        net::TcpListener,
        sync::{Mutex as AsyncMutex, oneshot},
        task::JoinHandle,
        time::{Instant, sleep},
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct RecordedRequest {
        method: &'static str,
        path: &'static str,
        authorization: Option<String>,
        body: String,
    }

    #[derive(Debug, Clone)]
    struct ResponseSpec {
        status: StatusCode,
        body: &'static str,
    }

    impl ResponseSpec {
        fn ok(body: &'static str) -> Self {
            Self {
                status: StatusCode::OK,
                body,
            }
        }
    }

    #[derive(Clone)]
    struct TestState {
        requests: Arc<AsyncMutex<Vec<RecordedRequest>>>,
        token_responses: Arc<AsyncMutex<VecDeque<ResponseSpec>>>,
        exchange_responses: Arc<AsyncMutex<VecDeque<ResponseSpec>>>,
    }

    struct TestApiServer {
        base_url: String,
        requests: Arc<AsyncMutex<Vec<RecordedRequest>>>,
        shutdown: Option<oneshot::Sender<()>>,
        task: JoinHandle<()>,
    }

    impl TestApiServer {
        async fn start(
            token_responses: Vec<ResponseSpec>,
            exchange_responses: Vec<ResponseSpec>,
        ) -> Self {
            let requests = Arc::new(AsyncMutex::new(Vec::new()));
            let state = TestState {
                requests: requests.clone(),
                token_responses: Arc::new(AsyncMutex::new(VecDeque::from(token_responses))),
                exchange_responses: Arc::new(AsyncMutex::new(VecDeque::from(exchange_responses))),
            };

            let app = Router::new()
                .route("/api/auth/cli/token", post(token_handler))
                .route("/api/tokens/exchange", post(exchange_handler))
                .with_state(state);

            let listener = TcpListener::bind("127.0.0.1:0")
                .await
                .expect("bind api test server");
            let address = listener.local_addr().expect("read api test server address");
            let (shutdown_tx, shutdown_rx) = oneshot::channel();
            let task = tokio::spawn(async move {
                axum::serve(listener, app)
                    .with_graceful_shutdown(async {
                        let _ = shutdown_rx.await;
                    })
                    .await
                    .expect("run api test server");
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

    impl Drop for TestApiServer {
        fn drop(&mut self) {
            if let Some(shutdown) = self.shutdown.take() {
                let _ = shutdown.send(());
            }
            self.task.abort();
        }
    }

    struct TestEnvGuard {
        previous_home: Option<OsString>,
        previous_api_url: Option<OsString>,
        previous_website_url: Option<OsString>,
        previous_dir: PathBuf,
    }

    impl TestEnvGuard {
        fn new(
            home_dir: &Path,
            workspace_dir: &Path,
            api_base_url: &str,
            website_url: &str,
        ) -> Self {
            let previous_home = std::env::var_os(home_var_name());
            let previous_api_url = std::env::var_os("RUSL_API_URL");
            let previous_website_url = std::env::var_os("RUSL_WEBSITE_URL");
            let previous_dir = std::env::current_dir().expect("current dir");

            set_env_var(home_var_name(), home_dir.as_os_str());
            set_env_var("RUSL_API_URL", api_base_url);
            set_env_var("RUSL_WEBSITE_URL", website_url);
            std::env::set_current_dir(workspace_dir).expect("set workspace dir");

            Self {
                previous_home,
                previous_api_url,
                previous_website_url,
                previous_dir,
            }
        }
    }

    impl Drop for TestEnvGuard {
        fn drop(&mut self) {
            restore_env_var(home_var_name(), self.previous_home.as_ref());
            restore_env_var("RUSL_API_URL", self.previous_api_url.as_ref());
            restore_env_var("RUSL_WEBSITE_URL", self.previous_website_url.as_ref());
            std::env::set_current_dir(&self.previous_dir).expect("restore current dir");
        }
    }

    #[tokio::test]
    #[serial]
    async fn saves_credentials_after_callback_code_is_received() {
        let server = TestApiServer::start(
            vec![ResponseSpec::ok(r#"{"refresh_token":"refresh-token"}"#)],
            vec![ResponseSpec::ok(r#"{"access_token":"access-token"}"#)],
        )
        .await;
        let temp_dir = TempDir::new().expect("create temp dir");
        let home_dir = temp_dir.path().join("home");
        let workspace_dir = temp_dir.path().join("workspace");
        fs::create_dir_all(&home_dir)
            .await
            .expect("create home dir");
        fs::create_dir_all(&workspace_dir)
            .await
            .expect("create workspace dir");
        let _env = TestEnvGuard::new(
            &home_dir,
            &workspace_dir,
            &server.base_url,
            &server.base_url,
        );
        let opened_url = Arc::new(Mutex::new(None::<String>));

        let task = tokio::spawn({
            let opened_url = opened_url.clone();
            async move {
                run_with_browser_opener(LoginArgs {}, move |url| {
                    *opened_url.lock().expect("lock opened url") = Some(url.to_string());
                    Ok(())
                })
                .await
            }
        });

        let browser_url = wait_for_browser_url(&opened_url).await;
        assert!(browser_url.contains("/login/cli?callback_url="));
        assert!(browser_url.contains("code_challenge="));

        let callback_url = extract_callback_url(&browser_url);
        reqwest::get(format!("{callback_url}?code=browser-code"))
            .await
            .expect("deliver callback");

        task.await
            .expect("join login task")
            .expect("login succeeds");

        let credentials = Credentials::load().expect("saved credentials");
        assert_eq!(credentials.access_token, "access-token");
        assert_eq!(credentials.refresh_token, "refresh-token");

        let requests = server.recorded_requests().await;
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].method, "POST");
        assert_eq!(requests[0].path, "/api/auth/cli/token");
        assert_eq!(requests[0].authorization, None);
        assert!(requests[0].body.contains("\"code\":\"browser-code\""));
        assert!(requests[0].body.contains("\"code_verifier\":\""));
        assert_eq!(requests[1].method, "POST");
        assert_eq!(requests[1].path, "/api/tokens/exchange");
        assert_eq!(
            requests[1].authorization.as_deref(),
            Some("Bearer refresh-token")
        );
    }

    #[tokio::test]
    #[serial]
    async fn fails_when_callback_does_not_include_a_code() {
        let server = TestApiServer::start(Vec::new(), Vec::new()).await;
        let temp_dir = TempDir::new().expect("create temp dir");
        let home_dir = temp_dir.path().join("home");
        let workspace_dir = temp_dir.path().join("workspace");
        fs::create_dir_all(&home_dir)
            .await
            .expect("create home dir");
        fs::create_dir_all(&workspace_dir)
            .await
            .expect("create workspace dir");
        let _env = TestEnvGuard::new(
            &home_dir,
            &workspace_dir,
            &server.base_url,
            &server.base_url,
        );
        let opened_url = Arc::new(Mutex::new(None::<String>));

        let task = tokio::spawn({
            let opened_url = opened_url.clone();
            async move {
                run_with_browser_opener(LoginArgs {}, move |url| {
                    *opened_url.lock().expect("lock opened url") = Some(url.to_string());
                    Ok(())
                })
                .await
            }
        });

        let callback_url = extract_callback_url(&wait_for_browser_url(&opened_url).await);
        reqwest::get(callback_url).await.expect("deliver callback");

        let error = task
            .await
            .expect("join login task")
            .expect_err("login should fail");

        assert!(
            error
                .to_string()
                .contains("no authorization code was received from the browser")
        );
        assert!(Credentials::load().is_none());
        assert!(server.recorded_requests().await.is_empty());
    }

    async fn token_handler(
        State(state): State<TestState>,
        headers: HeaderMap,
        body: Bytes,
    ) -> (StatusCode, Json<serde_json::Value>) {
        record_request(
            &state,
            "POST",
            "/api/auth/cli/token",
            &headers,
            String::from_utf8(body.to_vec()).expect("utf8 request body"),
        )
        .await;
        let response = next_response(&state.token_responses).await;
        (
            response.status,
            Json(serde_json::from_str(response.body).expect("valid token json")),
        )
    }

    async fn exchange_handler(
        State(state): State<TestState>,
        headers: HeaderMap,
    ) -> (StatusCode, Json<serde_json::Value>) {
        record_request(
            &state,
            "POST",
            "/api/tokens/exchange",
            &headers,
            String::new(),
        )
        .await;
        let response = next_response(&state.exchange_responses).await;
        (
            response.status,
            Json(serde_json::from_str(response.body).expect("valid exchange json")),
        )
    }

    async fn record_request(
        state: &TestState,
        method: &'static str,
        path: &'static str,
        headers: &HeaderMap,
        body: String,
    ) {
        state.requests.lock().await.push(RecordedRequest {
            method,
            path,
            authorization: headers
                .get("authorization")
                .and_then(|value| value.to_str().ok())
                .map(ToOwned::to_owned),
            body,
        });
    }

    async fn next_response(queue: &Arc<AsyncMutex<VecDeque<ResponseSpec>>>) -> ResponseSpec {
        queue
            .lock()
            .await
            .pop_front()
            .expect("test server received more requests than expected")
    }

    async fn wait_for_browser_url(opened_url: &Arc<Mutex<Option<String>>>) -> String {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(url) = opened_url.lock().expect("lock opened url").clone() {
                return url;
            }

            if Instant::now() >= deadline {
                panic!("timed out waiting for browser URL");
            }

            sleep(Duration::from_millis(20)).await;
        }
    }

    fn extract_callback_url(browser_url: &str) -> String {
        let encoded = browser_url
            .split("callback_url=")
            .nth(1)
            .expect("callback_url query parameter")
            .split('&')
            .next()
            .expect("callback_url value");

        encoded.replace("%3A", ":").replace("%2F", "/")
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

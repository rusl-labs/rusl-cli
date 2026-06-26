use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use rusl_api_client::{RUSL_AGENT_HEADER, RuslApiClient, SessionTokens, models::MeResponse};
use serde_json::{Value, json};
use std::{collections::VecDeque, sync::Arc};
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
    user_agent: Option<String>,
    rusl_agent: Option<String>,
}

impl RecordedRequest {
    fn new(
        method: &'static str,
        path: &'static str,
        authorization: Option<&str>,
        user_agent: Option<&str>,
    ) -> Self {
        Self {
            method,
            path,
            authorization: authorization.map(ToOwned::to_owned),
            user_agent: user_agent.map(ToOwned::to_owned),
            rusl_agent: None,
        }
    }

    fn new_with_agent(
        method: &'static str,
        path: &'static str,
        authorization: Option<&str>,
        user_agent: Option<&str>,
        rusl_agent: Option<&str>,
    ) -> Self {
        Self {
            method,
            path,
            authorization: authorization.map(ToOwned::to_owned),
            user_agent: user_agent.map(ToOwned::to_owned),
            rusl_agent: rusl_agent.map(ToOwned::to_owned),
        }
    }
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
    exchange_responses: Arc<Mutex<VecDeque<ResponseSpec>>>,
    me_responses: Arc<Mutex<VecDeque<ResponseSpec>>>,
}

struct TestServer {
    base_url: String,
    requests: Arc<Mutex<Vec<RecordedRequest>>>,
    shutdown: Option<oneshot::Sender<()>>,
    task: JoinHandle<()>,
}

impl TestServer {
    async fn start(exchange_responses: Vec<ResponseSpec>, me_responses: Vec<ResponseSpec>) -> Self {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let state = TestState {
            requests: requests.clone(),
            exchange_responses: Arc::new(Mutex::new(VecDeque::from(exchange_responses))),
            me_responses: Arc::new(Mutex::new(VecDeque::from(me_responses))),
        };

        let app = Router::new()
            .route("/api/v1/tokens/exchange", post(exchange_handler))
            .route("/api/v1/auth/sessions/me", get(me_handler))
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

async fn exchange_handler(
    State(state): State<TestState>,
    headers: HeaderMap,
) -> (StatusCode, Json<Value>) {
    record_request(&state, "POST", "/api/v1/tokens/exchange", &headers).await;
    let response = next_response(&state.exchange_responses).await;
    (response.status, Json(response.body))
}

async fn me_handler(
    State(state): State<TestState>,
    headers: HeaderMap,
) -> (StatusCode, Json<Value>) {
    record_request(&state, "GET", "/api/v1/auth/sessions/me", &headers).await;
    let response = next_response(&state.me_responses).await;
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
        authorization: header_value(headers, "authorization"),
        user_agent: header_value(headers, "user-agent"),
        rusl_agent: header_value(headers, RUSL_AGENT_HEADER),
    });
}

async fn next_response(queue: &Arc<Mutex<VecDeque<ResponseSpec>>>) -> ResponseSpec {
    queue
        .lock()
        .await
        .pop_front()
        .expect("test server received more requests than expected")
}

fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
}

fn unauthenticated_me() -> Value {
    json!({ "authenticated": false })
}

#[tokio::test]
async fn exchanges_refresh_token_before_request_when_access_token_is_missing() {
    let server = TestServer::start(
        vec![ResponseSpec::ok(json!({ "access_token": "fresh-access" }))],
        vec![ResponseSpec::ok(unauthenticated_me())],
    )
    .await;
    let client = RuslApiClient::new(server.base_url.clone()).with_user_agent("rusl-test");
    let mut session = SessionTokens::new(None, Some("refresh-token".to_string()));

    let response = client
        .fetch_session_me(&mut session)
        .await
        .expect("fetch session me");

    assert!(matches!(
        response,
        MeResponse::MeResponseUnauthenticated1(ref body) if !body.authenticated
    ));
    assert_eq!(session.access_token.as_deref(), Some("fresh-access"));
    assert_eq!(session.refresh_token.as_deref(), Some("refresh-token"));
    assert_eq!(
        server.recorded_requests().await,
        vec![
            RecordedRequest::new(
                "POST",
                "/api/v1/tokens/exchange",
                Some("Bearer refresh-token"),
                Some("rusl-test")
            ),
            RecordedRequest::new(
                "GET",
                "/api/v1/auth/sessions/me",
                Some("Bearer fresh-access"),
                Some("rusl-test")
            ),
        ]
    );
}

#[tokio::test]
async fn sends_rusl_agent_header_on_session_and_refresh_requests() {
    let server = TestServer::start(
        vec![ResponseSpec::ok(json!({ "access_token": "fresh-access" }))],
        vec![ResponseSpec::ok(unauthenticated_me())],
    )
    .await;
    let client = RuslApiClient::new(server.base_url.clone())
        .with_user_agent("rusl-test")
        .with_rusl_agent("mcp");
    let mut session = SessionTokens::new(None, Some("refresh-token".to_string()));

    client
        .fetch_session_me(&mut session)
        .await
        .expect("fetch session me");

    assert_eq!(
        server.recorded_requests().await,
        vec![
            RecordedRequest::new_with_agent(
                "POST",
                "/api/v1/tokens/exchange",
                Some("Bearer refresh-token"),
                Some("rusl-test"),
                Some("mcp")
            ),
            RecordedRequest::new_with_agent(
                "GET",
                "/api/v1/auth/sessions/me",
                Some("Bearer fresh-access"),
                Some("rusl-test"),
                Some("mcp")
            ),
        ]
    );
}

#[tokio::test]
async fn refreshes_and_retries_once_after_an_unauthorized_response() {
    let server = TestServer::start(
        vec![ResponseSpec::ok(json!({ "access_token": "fresh-access" }))],
        vec![
            ResponseSpec::unauthorized(),
            ResponseSpec::ok(unauthenticated_me()),
        ],
    )
    .await;
    let client = RuslApiClient::new(server.base_url.clone()).with_user_agent("rusl-test");
    let mut session = SessionTokens::new(
        Some("stale-access".to_string()),
        Some("refresh-token".to_string()),
    );

    let response = client
        .fetch_session_me(&mut session)
        .await
        .expect("fetch session me");

    assert!(matches!(
        response,
        MeResponse::MeResponseUnauthenticated1(ref body) if !body.authenticated
    ));
    assert_eq!(session.access_token.as_deref(), Some("fresh-access"));
    assert_eq!(
        server.recorded_requests().await,
        vec![
            RecordedRequest::new(
                "GET",
                "/api/v1/auth/sessions/me",
                Some("Bearer stale-access"),
                Some("rusl-test")
            ),
            RecordedRequest::new(
                "POST",
                "/api/v1/tokens/exchange",
                Some("Bearer refresh-token"),
                Some("rusl-test")
            ),
            RecordedRequest::new(
                "GET",
                "/api/v1/auth/sessions/me",
                Some("Bearer fresh-access"),
                Some("rusl-test")
            ),
        ]
    );
}

#[tokio::test]
async fn clears_tokens_and_continues_unauthenticated_when_preflight_refresh_fails() {
    let server = TestServer::start(
        vec![ResponseSpec::unauthorized()],
        vec![ResponseSpec::ok(unauthenticated_me())],
    )
    .await;
    let client = RuslApiClient::new(server.base_url.clone()).with_user_agent("rusl-test");
    let mut session = SessionTokens::new(None, Some("expired-refresh".to_string()));

    let response = client
        .fetch_session_me(&mut session)
        .await
        .expect("fetch session me");

    assert!(matches!(
        response,
        MeResponse::MeResponseUnauthenticated1(ref body) if !body.authenticated
    ));
    assert_eq!(session.access_token, None);
    assert_eq!(session.refresh_token, None);
    assert_eq!(
        server.recorded_requests().await,
        vec![
            RecordedRequest::new(
                "POST",
                "/api/v1/tokens/exchange",
                Some("Bearer expired-refresh"),
                Some("rusl-test")
            ),
            RecordedRequest::new("GET", "/api/v1/auth/sessions/me", None, Some("rusl-test")),
        ]
    );
}

#[tokio::test]
async fn clears_tokens_and_retries_unauthenticated_when_refresh_after_401_fails() {
    let server = TestServer::start(
        vec![ResponseSpec::unauthorized()],
        vec![
            ResponseSpec::unauthorized(),
            ResponseSpec::ok(unauthenticated_me()),
        ],
    )
    .await;
    let client = RuslApiClient::new(server.base_url.clone()).with_user_agent("rusl-test");
    let mut session = SessionTokens::new(
        Some("stale-access".to_string()),
        Some("expired-refresh".to_string()),
    );

    let response = client
        .fetch_session_me(&mut session)
        .await
        .expect("fetch session me");

    assert!(matches!(
        response,
        MeResponse::MeResponseUnauthenticated1(ref body) if !body.authenticated
    ));
    assert_eq!(session.access_token, None);
    assert_eq!(session.refresh_token, None);
    assert_eq!(
        server.recorded_requests().await,
        vec![
            RecordedRequest::new(
                "GET",
                "/api/v1/auth/sessions/me",
                Some("Bearer stale-access"),
                Some("rusl-test")
            ),
            RecordedRequest::new(
                "POST",
                "/api/v1/tokens/exchange",
                Some("Bearer expired-refresh"),
                Some("rusl-test")
            ),
            RecordedRequest::new("GET", "/api/v1/auth/sessions/me", None, Some("rusl-test")),
        ]
    );
}

#[tokio::test]
async fn retries_only_once_after_refreshing_the_access_token() {
    let server = TestServer::start(
        vec![ResponseSpec::ok(json!({ "access_token": "fresh-access" }))],
        vec![ResponseSpec::unauthorized(), ResponseSpec::unauthorized()],
    )
    .await;
    let client = RuslApiClient::new(server.base_url.clone()).with_user_agent("rusl-test");
    let mut session = SessionTokens::new(
        Some("stale-access".to_string()),
        Some("refresh-token".to_string()),
    );

    let error = client
        .fetch_session_me(&mut session)
        .await
        .expect_err("expected unauthorized result");

    assert!(matches!(error, rusl_api_client::ApiError::Unauthorized));
    assert_eq!(session.access_token.as_deref(), Some("fresh-access"));
    assert_eq!(
        server.recorded_requests().await,
        vec![
            RecordedRequest::new(
                "GET",
                "/api/v1/auth/sessions/me",
                Some("Bearer stale-access"),
                Some("rusl-test")
            ),
            RecordedRequest::new(
                "POST",
                "/api/v1/tokens/exchange",
                Some("Bearer refresh-token"),
                Some("rusl-test")
            ),
            RecordedRequest::new(
                "GET",
                "/api/v1/auth/sessions/me",
                Some("Bearer fresh-access"),
                Some("rusl-test")
            ),
        ]
    );
}

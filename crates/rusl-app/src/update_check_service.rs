use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const DEFAULT_LATEST_RELEASE_API_URL: &str =
    "https://api.github.com/repos/rusl-labs/rusl-cli/releases/latest";
const DEFAULT_LATEST_RELEASE_WEB_URL: &str =
    "https://github.com/rusl-labs/rusl-cli/releases/latest";
const DISABLE_UPDATE_CHECK_ENV: &str = "RUSL_NO_UPDATE_CHECK";
const UPDATE_CHECK_FILE: &str = "update-check.json";
const UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);
const UPDATE_CHECK_TIMEOUT: Duration = Duration::from_millis(800);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateHint {
    pub current_version: String,
    pub latest_version: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateCheckState {
    last_checked_at_unix_seconds: u64,
}

#[derive(Debug, Deserialize)]
struct LatestReleaseResponse {
    tag_name: String,
}

pub async fn check_for_update(current_version: &str) -> Option<UpdateHint> {
    if update_check_disabled() {
        return None;
    }

    let state_path = update_check_state_path()?;
    check_for_update_with_options(
        current_version,
        DEFAULT_LATEST_RELEASE_API_URL,
        &state_path,
        SystemTime::now(),
        UPDATE_CHECK_INTERVAL,
        UPDATE_CHECK_TIMEOUT,
    )
    .await
}

async fn check_for_update_with_options(
    current_version: &str,
    latest_release_api_url: &str,
    state_path: &Path,
    now: SystemTime,
    interval: Duration,
    timeout: Duration,
) -> Option<UpdateHint> {
    if !check_is_due(state_path, now, interval)? {
        return None;
    }

    record_update_check_attempt(state_path, now)?;

    let release = fetch_latest_release(latest_release_api_url, current_version, timeout)
        .await
        .ok()?;

    hint_for_versions(current_version, &release.tag_name)
}

async fn fetch_latest_release(
    latest_release_api_url: &str,
    current_version: &str,
    timeout: Duration,
) -> Result<LatestReleaseResponse, reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(timeout)
        .user_agent(format!("rusl-cli/{current_version}"))
        .build()?;

    let response = client
        .get(latest_release_api_url)
        .send()
        .await?
        .error_for_status()?;

    response.json::<LatestReleaseResponse>().await
}

fn hint_for_versions(current_version: &str, latest_tag: &str) -> Option<UpdateHint> {
    let current = parse_version(current_version)?;
    let latest = parse_version(latest_tag)?;

    if latest <= current {
        return None;
    }

    let latest_version = latest.to_string();
    let current_version = current.to_string();
    let message = format!(
        "rusl {latest_version} is available. You have {current_version}. Update with Homebrew or rerun the installer: {DEFAULT_LATEST_RELEASE_WEB_URL}"
    );

    Some(UpdateHint {
        current_version,
        latest_version,
        message,
    })
}

fn parse_version(version: &str) -> Option<Version> {
    Version::parse(version.trim().trim_start_matches('v')).ok()
}

fn update_check_disabled() -> bool {
    std::env::var_os(DISABLE_UPDATE_CHECK_ENV).is_some()
}

fn update_check_state_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("", "", "rusl")
        .map(|dirs| dirs.cache_dir().join(UPDATE_CHECK_FILE))
}

fn check_is_due(state_path: &Path, now: SystemTime, interval: Duration) -> Option<bool> {
    if !state_path.exists() {
        return Some(true);
    }

    let contents = fs::read_to_string(state_path).ok()?;
    let state = serde_json::from_str::<UpdateCheckState>(&contents).unwrap_or(UpdateCheckState {
        last_checked_at_unix_seconds: 0,
    });
    let now_seconds = unix_seconds(now)?;

    Some(now_seconds.saturating_sub(state.last_checked_at_unix_seconds) >= interval.as_secs())
}

fn record_update_check_attempt(state_path: &Path, now: SystemTime) -> Option<()> {
    let parent = state_path.parent()?;
    fs::create_dir_all(parent).ok()?;
    let state = UpdateCheckState {
        last_checked_at_unix_seconds: unix_seconds(now)?,
    };
    let contents = serde_json::to_string(&state).ok()?;
    fs::write(state_path, contents).ok()
}

fn unix_seconds(time: SystemTime) -> Option<u64> {
    time.duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Json, Router, http::StatusCode, routing::get};
    use serde_json::json;
    use serial_test::serial;
    use std::{
        ffi::OsString,
        net::SocketAddr,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
    };
    use tempfile::TempDir;
    use tokio::{net::TcpListener, task::JoinHandle};

    #[test]
    fn newer_release_produces_hint() {
        let hint = hint_for_versions("0.1.0", "v0.2.0").expect("newer version hint");

        assert_eq!(hint.current_version, "0.1.0");
        assert_eq!(hint.latest_version, "0.2.0");
        assert!(hint.message.contains("rusl 0.2.0 is available"));
        assert!(hint.message.contains(DEFAULT_LATEST_RELEASE_WEB_URL));
    }

    #[test]
    fn same_or_older_release_produces_no_hint() {
        assert!(hint_for_versions("0.1.0", "v0.1.0").is_none());
        assert!(hint_for_versions("0.1.0", "v0.0.9").is_none());
    }

    #[tokio::test]
    async fn check_fetches_latest_release_and_records_attempt() {
        let server = TestServer::new(StatusCode::OK, json!({ "tag_name": "v0.2.0" })).await;
        let temp_dir = TempDir::new().expect("temp dir");
        let state_path = temp_dir.path().join("update-check.json");

        let hint = check_for_update_with_options(
            "0.1.0",
            &server.url(),
            &state_path,
            unix_time(1_000),
            UPDATE_CHECK_INTERVAL,
            Duration::from_secs(2),
        )
        .await
        .expect("update hint");

        assert_eq!(hint.latest_version, "0.2.0");
        assert_eq!(server.request_count(), 1);
        assert!(state_path.exists());
    }

    #[tokio::test]
    async fn cache_throttles_checks_for_twenty_four_hours() {
        let server = TestServer::new(StatusCode::OK, json!({ "tag_name": "v0.2.0" })).await;
        let temp_dir = TempDir::new().expect("temp dir");
        let state_path = temp_dir.path().join("update-check.json");

        let first = check_for_update_with_options(
            "0.1.0",
            &server.url(),
            &state_path,
            unix_time(1_000),
            UPDATE_CHECK_INTERVAL,
            Duration::from_secs(2),
        )
        .await;
        let second = check_for_update_with_options(
            "0.1.0",
            &server.url(),
            &state_path,
            unix_time(1_000 + 60),
            UPDATE_CHECK_INTERVAL,
            Duration::from_secs(2),
        )
        .await;

        assert!(first.is_some());
        assert!(second.is_none());
        assert_eq!(server.request_count(), 1);
    }

    #[tokio::test]
    async fn failed_request_is_silent_and_updates_cache() {
        let server = TestServer::new(StatusCode::INTERNAL_SERVER_ERROR, json!({})).await;
        let temp_dir = TempDir::new().expect("temp dir");
        let state_path = temp_dir.path().join("update-check.json");

        let first = check_for_update_with_options(
            "0.1.0",
            &server.url(),
            &state_path,
            unix_time(1_000),
            UPDATE_CHECK_INTERVAL,
            Duration::from_secs(2),
        )
        .await;
        let second = check_for_update_with_options(
            "0.1.0",
            &server.url(),
            &state_path,
            unix_time(1_000 + 60),
            UPDATE_CHECK_INTERVAL,
            Duration::from_secs(2),
        )
        .await;

        assert!(first.is_none());
        assert!(second.is_none());
        assert_eq!(server.request_count(), 1);
        assert!(state_path.exists());
    }

    #[test]
    #[serial]
    fn environment_disables_update_checks() {
        let previous = std::env::var_os(DISABLE_UPDATE_CHECK_ENV);
        set_env_var(DISABLE_UPDATE_CHECK_ENV, "1");

        assert!(update_check_disabled());

        restore_env_var(DISABLE_UPDATE_CHECK_ENV, previous.as_ref());
    }

    struct TestServer {
        addr: SocketAddr,
        requests: Arc<AtomicUsize>,
        task: JoinHandle<()>,
    }

    impl TestServer {
        async fn new(status: StatusCode, body: serde_json::Value) -> Self {
            let requests = Arc::new(AtomicUsize::new(0));
            let app = Router::new().route(
                "/latest",
                get({
                    let requests = Arc::clone(&requests);
                    move || {
                        let body = body.clone();
                        let requests = Arc::clone(&requests);
                        async move {
                            requests.fetch_add(1, Ordering::SeqCst);
                            (status, Json(body))
                        }
                    }
                }),
            );
            let listener = TcpListener::bind("127.0.0.1:0")
                .await
                .expect("bind test server");
            let addr = listener.local_addr().expect("test server addr");
            let task = tokio::spawn(async move {
                axum::serve(listener, app).await.expect("test server");
            });

            Self {
                addr,
                requests,
                task,
            }
        }

        fn url(&self) -> String {
            format!("http://{}/latest", self.addr)
        }

        fn request_count(&self) -> usize {
            self.requests.load(Ordering::SeqCst)
        }
    }

    impl Drop for TestServer {
        fn drop(&mut self) {
            self.task.abort();
        }
    }

    fn unix_time(seconds: u64) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(seconds)
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

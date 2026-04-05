use crate::config;
use crate::registry::client::RegistryClient;
use anyhow::{Context, Result};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhoamiProfile {
    pub email: String,
    pub slug: String,
    pub user_id: String,
    pub accounts: Vec<WhoamiAccount>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhoamiAccount {
    pub slug: String,
    pub role: String,
    pub account_type: String,
}

impl WhoamiAccount {
    pub fn is_organization(&self) -> bool {
        self.account_type == "organization"
    }
}

pub async fn load_current_user() -> Result<WhoamiProfile> {
    let config = config::load().context("Failed to load network configurations")?;
    let client = RegistryClient::new(config);
    let session_data = client
        .fetch_me()
        .await
        .context("Failed to authenticate with registry! Are you sure you are logged in?")?;

    Ok(parse_profile(&session_data))
}

fn parse_profile(session_data: &Value) -> WhoamiProfile {
    WhoamiProfile {
        email: session_data["user"]["email"]
            .as_str()
            .unwrap_or("Unknown Email")
            .to_string(),
        slug: session_data["user"]["slug"]
            .as_str()
            .unwrap_or("Unknown Slug")
            .to_string(),
        user_id: session_data["user"]["id"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string(),
        accounts: parse_accounts(session_data),
    }
}

fn parse_accounts(session_data: &Value) -> Vec<WhoamiAccount> {
    let Some(accounts) = session_data["accounts"].as_object() else {
        return Vec::new();
    };

    let mut mapped = accounts
        .iter()
        .map(|(slug, details)| WhoamiAccount {
            slug: slug.clone(),
            role: details["roles"][0].as_str().unwrap_or("MEMBER").to_string(),
            account_type: details["type"].as_str().unwrap_or("unknown").to_string(),
        })
        .collect::<Vec<_>>();

    mapped.sort_by(|left, right| left.slug.cmp(&right.slug));
    mapped
}

#[cfg(test)]
mod tests {
    use super::{WhoamiAccount, parse_profile};
    use serde_json::json;

    #[test]
    fn parses_profile_and_sorts_accounts() {
        let session = json!({
            "user": {
                "email": "dev@rusl.app",
                "slug": "hassox",
                "id": "user_123"
            },
            "accounts": {
                "zeta": {
                    "type": "organization",
                    "roles": ["ADMIN"]
                },
                "alpha": {
                    "type": "user",
                    "roles": ["OWNER"]
                }
            }
        });

        let profile = parse_profile(&session);

        assert_eq!(profile.email, "dev@rusl.app");
        assert_eq!(profile.slug, "hassox");
        assert_eq!(profile.user_id, "user_123");
        assert_eq!(
            profile.accounts,
            vec![
                WhoamiAccount {
                    slug: "alpha".to_string(),
                    role: "OWNER".to_string(),
                    account_type: "user".to_string(),
                },
                WhoamiAccount {
                    slug: "zeta".to_string(),
                    role: "ADMIN".to_string(),
                    account_type: "organization".to_string(),
                },
            ]
        );
        assert!(profile.accounts[1].is_organization());
    }
}

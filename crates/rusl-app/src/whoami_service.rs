use crate::config;
use crate::registry::client::RegistryClient;
use anyhow::{Context, Result, bail};
use rusl_api_client::models;

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
    let session = client
        .fetch_me()
        .await
        .context("Failed to authenticate with registry! Are you sure you are logged in?")?;

    parse_profile(session)
}

fn parse_profile(session: models::MeResponse) -> Result<WhoamiProfile> {
    match session {
        models::MeResponse::MeResponseAuthenticated1(authenticated) => Ok(WhoamiProfile {
            email: authenticated.user.email.clone().unwrap_or_default(),
            slug: authenticated.user.slug.clone(),
            user_id: authenticated.user.id.clone(),
            accounts: parse_accounts(&authenticated.accounts),
        }),
        models::MeResponse::MeResponseUnauthenticated1(_) => {
            bail!("You are not authenticated. Run `rusl login` first.")
        }
    }
}

fn parse_accounts(
    accounts: &std::collections::HashMap<String, models::SessionAccount1>,
) -> Vec<WhoamiAccount> {
    let mut mapped = accounts
        .values()
        .map(|details| WhoamiAccount {
            slug: details.slug.clone(),
            role: details
                .roles
                .first()
                .map(|role| match role {
                    models::session_account_1::Roles::Owner => "OWNER",
                    models::session_account_1::Roles::Contributor => "CONTRIBUTOR",
                })
                .unwrap_or("MEMBER")
                .to_string(),
            account_type: match details.r#type {
                models::session_account_1::Type::User => "user",
                models::session_account_1::Type::Organization => "organization",
            }
            .to_string(),
        })
        .collect::<Vec<_>>();

    mapped.sort_by(|left, right| left.slug.cmp(&right.slug));
    mapped
}

#[cfg(test)]
mod tests {
    use super::{WhoamiAccount, parse_profile};
    use rusl_api_client::models;
    use serde_json::json;

    #[test]
    fn parses_profile_and_sorts_accounts() {
        let session: models::MeResponse = serde_json::from_value(json!({
            "authenticated": true,
            "invitations": [],
            "user": {
                "__typename": "users",
                "email": "dev@rusl.app",
                "guid": "guid_123",
                "id": "user_123",
                "inserted_at": "2026-04-05T00:00:00Z",
                "slug": "hassox",
                "updated_at": "2026-04-05T00:00:00Z",
                "user_type": "human"
            },
            "accounts": {
                "zeta": {
                    "__typename": "accounts",
                    "available_seats": 5,
                    "consumed_seats": 3,
                    "guid": "account_zeta",
                    "owner_user_id": "00000000-0000-0000-0000-000000000000",
                    "permissions": {},
                    "roles": ["CONTRIBUTOR"],
                    "slug": "zeta",
                    "type": "organization"
                },
                "alpha": {
                    "__typename": "accounts",
                    "available_seats": 1,
                    "consumed_seats": 1,
                    "guid": "account_alpha",
                    "owner_user_id": "00000000-0000-0000-0000-000000000000",
                    "permissions": {},
                    "roles": ["OWNER"],
                    "slug": "alpha",
                    "type": "user"
                }
            }
        }))
        .expect("deserialize authenticated session");

        let profile = parse_profile(session).expect("parse authenticated session");

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
                    role: "CONTRIBUTOR".to_string(),
                    account_type: "organization".to_string(),
                },
            ]
        );
        assert!(profile.accounts[1].is_organization());
    }

    #[test]
    fn rejects_unauthenticated_sessions() {
        let session: models::MeResponse = serde_json::from_value(json!({
            "authenticated": false
        }))
        .expect("deserialize unauthenticated session");

        let error = parse_profile(session).expect_err("expected auth error");

        assert!(error.to_string().contains("Run `rusl login` first"));
    }
}

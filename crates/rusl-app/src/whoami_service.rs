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
        models::MeResponse::MeResponseAuthenticated1(authenticated) => {
            let user = &authenticated.user;
            let display_name = user
                .name
                .clone()
                .flatten()
                .filter(|value| !value.trim().is_empty());
            let email = user
                .email
                .clone()
                .filter(|value| !value.trim().is_empty())
                .or_else(|| display_name.clone())
                .unwrap_or_else(|| "service account".to_string());
            let slug = user
                .slug
                .clone()
                .filter(|value| !value.trim().is_empty())
                .or_else(|| {
                    user.owning_account_slug
                        .clone()
                        .flatten()
                        .filter(|value| !value.trim().is_empty())
                })
                .unwrap_or_default();

            Ok(WhoamiProfile {
                email,
                slug,
                user_id: user.id.clone(),
                accounts: parse_accounts(&authenticated.accounts),
            })
        }
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
            "authentication_type": "jwt",
            "invitations": [],
            "user": {
                "__typename": "users",
                "email": "dev@rusl.app",
                "guid": "guid_123",
                "id": "user_123",
                "inserted_at": "2026-04-05T00:00:00Z",
                "principal_type": "regular",
                "slug": "hassox",
                "updated_at": "2026-04-05T00:00:00Z",
                "user_type": "human"
            },
            "accounts": {
                "zeta": {
                    "__typename": "accounts",
                    "available_seats": 5,
                    "consumed_seats": 3,
                    "entitlements": {
                        "feed_subscriptions": true,
                        "max_private_schemas": -1,
                        "max_service_accounts": -1,
                        "private_annotations_on_public": true,
                        "private_team_visibility": true
                    },
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
                    "entitlements": {
                        "feed_subscriptions": false,
                        "max_private_schemas": 0,
                        "max_service_accounts": 0,
                        "private_annotations_on_public": false,
                        "private_team_visibility": false
                    },
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
    fn parses_service_account_session_without_user_slug() {
        let session: models::MeResponse = serde_json::from_value(json!({
            "user": {
                "id": "841d9e69-7e1a-4709-b28f-18f6f01952e3",
                "name": "Local Agent",
                "user_type": "unknown",
                "inserted_at": "2026-07-14T18:12:48Z",
                "updated_at": "2026-07-14T18:12:48Z",
                "guid": "users.841d9e69-7e1a-4709-b28f-18f6f01952e3",
                "disabled_at": null,
                "owning_account_slug": "dan",
                "principal_type": "service",
                "__typename": "users"
            },
            "service_account": {
                "api_key_id": "c236847e-a84a-42a0-88bc-09a271bb3c24"
            },
            "accounts": {
                "dan": {
                    "type": "user",
                    "permissions": {
                        "schemas": {
                            "read": true,
                            "write": true
                        }
                    },
                    "display_name": null,
                    "guid": "accounts.dan",
                    "avatar_asset_id": "9476cf67-dd54-4fc3-b896-45f1060e3fb8",
                    "billing_subscription_id": "448661c2-d38e-4a59-91e6-4bcc93fa7b2a",
                    "owner_user_id": "c6b4e751-9c7e-48d9-8cbd-d912991b76ee",
                    "slug": "dan",
                    "__typename": "accounts",
                    "roles": ["CONTRIBUTOR"],
                    "stripe_subscription_id": "sub_1TlJflGo3rtnwShveaIp2Y9g",
                    "stripe_price_id": "price_1TlGF2Go3rtnwShvDuykmT0U",
                    "plan_slug": "pro",
                    "entitlements": {
                        "private_annotations_on_public": true,
                        "feed_subscriptions": true,
                        "max_private_schemas": -1,
                        "max_service_accounts": -1,
                        "private_team_visibility": true
                    },
                    "consumed_seats": 1,
                    "available_seats": 1
                }
            },
            "invitations": [],
            "authentication_type": "api_key",
            "authenticated": true
        }))
        .expect("deserialize service account session");

        let profile = parse_profile(session).expect("parse service account session");

        assert_eq!(profile.email, "Local Agent");
        assert_eq!(profile.slug, "dan");
        assert_eq!(profile.user_id, "841d9e69-7e1a-4709-b28f-18f6f01952e3");
        assert_eq!(
            profile.accounts,
            vec![WhoamiAccount {
                slug: "dan".to_string(),
                role: "CONTRIBUTOR".to_string(),
                account_type: "user".to_string(),
            }]
        );
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

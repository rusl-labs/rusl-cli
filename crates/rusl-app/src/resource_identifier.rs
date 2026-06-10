#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    Schema,
    Bundle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryResource {
    pub kind: ResourceKind,
    pub account: String,
    pub slug: String,
}

impl RegistryResource {
    pub fn from_identifier(identifier: &str) -> Option<Self> {
        let trimmed = identifier.trim();
        let parts: Vec<&str> = trimmed.split('/').collect();

        match parts.as_slice() {
            [account, "schemas", slug] if valid_part(account) && valid_part(slug) => Some(Self {
                kind: ResourceKind::Schema,
                account: (*account).to_string(),
                slug: (*slug).to_string(),
            }),
            [account, "bundles", slug] if valid_part(account) && valid_part(slug) => Some(Self {
                kind: ResourceKind::Bundle,
                account: (*account).to_string(),
                slug: (*slug).to_string(),
            }),
            [account, slug]
                if valid_part(account)
                    && valid_part(slug)
                    && *slug != "bundles"
                    && *slug != "schemas" =>
            {
                Some(Self {
                    kind: ResourceKind::Schema,
                    account: (*account).to_string(),
                    slug: (*slug).to_string(),
                })
            }
            _ => None,
        }
    }

    pub fn schema(identifier: &str) -> Option<Self> {
        let (account, slug) = parse_schema_identifier(identifier)?;
        Some(Self {
            kind: ResourceKind::Schema,
            account,
            slug,
        })
    }

    pub fn bundle(identifier: &str) -> Option<Self> {
        let trimmed = identifier.trim();
        let parts: Vec<&str> = trimmed.split('/').collect();

        match parts.as_slice() {
            [account, "bundles", slug] if valid_part(account) && valid_part(slug) => Some(Self {
                kind: ResourceKind::Bundle,
                account: (*account).to_string(),
                slug: (*slug).to_string(),
            }),
            [account, slug] if valid_part(account) && valid_part(slug) && *slug != "bundles" => {
                Some(Self {
                    kind: ResourceKind::Bundle,
                    account: (*account).to_string(),
                    slug: (*slug).to_string(),
                })
            }
            _ => None,
        }
    }

    pub fn package_key(&self) -> String {
        let prefix = match self.kind {
            ResourceKind::Schema => "schema",
            ResourceKind::Bundle => "bundle",
        };
        format!("{prefix}:{}", self.identifier())
    }

    pub fn identifier(&self) -> String {
        match self.kind {
            ResourceKind::Schema => format!("{}/schemas/{}", self.account, self.slug),
            ResourceKind::Bundle => format!("{}/bundles/{}", self.account, self.slug),
        }
    }
}

pub fn parse_package_key(package_key: &str) -> Option<RegistryResource> {
    let (kind, identifier) = package_key.split_once(':')?;
    match kind {
        "schema" => RegistryResource::schema(identifier),
        "bundle" => RegistryResource::bundle(identifier),
        _ => None,
    }
}

pub fn display_package_key(package_key: &str) -> String {
    parse_package_key(package_key)
        .map(|resource| resource.identifier())
        .unwrap_or_else(|| package_key.to_string())
}

pub fn package_key_for(kind: ResourceKind, identifier: &str) -> Option<String> {
    let resource = match kind {
        ResourceKind::Schema => RegistryResource::schema(identifier),
        ResourceKind::Bundle => RegistryResource::bundle(identifier),
    }?;
    Some(resource.package_key())
}

pub fn package_key_from_identifier(identifier: &str) -> Option<String> {
    RegistryResource::from_identifier(identifier).map(|resource| resource.package_key())
}

fn parse_schema_identifier(identifier: &str) -> Option<(String, String)> {
    let trimmed = identifier.trim();
    let parts: Vec<&str> = trimmed.split('/').collect();

    match parts.as_slice() {
        [account, "schemas", slug] if valid_part(account) && valid_part(slug) => {
            Some(((*account).to_string(), (*slug).to_string()))
        }
        [account, slug]
            if valid_part(account)
                && valid_part(slug)
                && *slug != "schemas"
                && *slug != "bundles" =>
        {
            Some(((*account).to_string(), (*slug).to_string()))
        }
        _ => None,
    }
}

fn valid_part(part: &str) -> bool {
    !part.trim().is_empty() && part == part.trim()
}

#[cfg(test)]
mod tests {
    use super::{
        RegistryResource, ResourceKind, display_package_key, package_key_for,
        package_key_from_identifier,
    };

    #[test]
    fn parses_schema_identifiers() {
        let resource = RegistryResource::schema("acme/schemas/payment").expect("schema identifier");

        assert_eq!(resource.kind, ResourceKind::Schema);
        assert_eq!(resource.account, "acme");
        assert_eq!(resource.slug, "payment");
        assert_eq!(resource.identifier(), "acme/schemas/payment");
        assert_eq!(resource.package_key(), "schema:acme/schemas/payment");
    }

    #[test]
    fn infers_resource_kind_from_canonical_identifier() {
        let schema =
            RegistryResource::from_identifier("acme/schemas/payment").expect("schema resource");
        let bundle =
            RegistryResource::from_identifier("acme/bundles/billing").expect("bundle resource");

        assert_eq!(schema.kind, ResourceKind::Schema);
        assert_eq!(schema.identifier(), "acme/schemas/payment");
        assert_eq!(bundle.kind, ResourceKind::Bundle);
        assert_eq!(bundle.identifier(), "acme/bundles/billing");
    }

    #[test]
    fn accepts_legacy_schema_identifiers_but_displays_canonical_form() {
        let resource = RegistryResource::schema("acme/payment").expect("legacy schema identifier");

        assert_eq!(resource.identifier(), "acme/schemas/payment");
        assert_eq!(resource.package_key(), "schema:acme/schemas/payment");
    }

    #[test]
    fn parses_canonical_bundle_identifiers() {
        let resource = RegistryResource::bundle("acme/bundles/billing").expect("bundle identifier");

        assert_eq!(resource.kind, ResourceKind::Bundle);
        assert_eq!(resource.account, "acme");
        assert_eq!(resource.slug, "billing");
        assert_eq!(resource.identifier(), "acme/bundles/billing");
        assert_eq!(resource.package_key(), "bundle:acme/bundles/billing");
    }

    #[test]
    fn accepts_legacy_bundle_identifiers_but_displays_canonical_form() {
        let resource = RegistryResource::bundle("acme/billing").expect("legacy bundle identifier");

        assert_eq!(resource.identifier(), "acme/bundles/billing");
        assert_eq!(resource.package_key(), "bundle:acme/bundles/billing");
    }

    #[test]
    fn rejects_incomplete_canonical_bundle_identifiers() {
        assert_eq!(RegistryResource::bundle("acme/bundles"), None);
    }

    #[test]
    fn displays_package_keys_with_canonical_bundle_identifiers() {
        assert_eq!(
            display_package_key("schema:acme/schemas/root"),
            "acme/schemas/root"
        );
        assert_eq!(display_package_key("schema:acme/root"), "acme/schemas/root");
        assert_eq!(
            display_package_key("bundle:acme/common"),
            "acme/bundles/common"
        );
        assert_eq!(
            display_package_key("bundle:acme/bundles/common"),
            "acme/bundles/common"
        );
    }

    #[test]
    fn builds_package_keys_for_identifiers() {
        assert_eq!(
            package_key_for(ResourceKind::Bundle, "acme/common"),
            Some("bundle:acme/bundles/common".to_string())
        );
        assert_eq!(
            package_key_for(ResourceKind::Schema, "acme/common"),
            Some("schema:acme/schemas/common".to_string())
        );
        assert_eq!(
            package_key_from_identifier("acme/bundles/common"),
            Some("bundle:acme/bundles/common".to_string())
        );
        assert_eq!(
            package_key_from_identifier("acme/common"),
            Some("schema:acme/schemas/common".to_string())
        );
        assert_eq!(
            package_key_from_identifier("acme/schemas/common"),
            Some("schema:acme/schemas/common".to_string())
        );
    }
}

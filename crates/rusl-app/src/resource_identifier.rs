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
    /// Parse a canonical resource identifier (`account/schemas/name` or `account/bundles/name`),
    /// inferring the kind from the middle segment.
    pub fn from_identifier(identifier: &str) -> Option<Self> {
        let parts: Vec<&str> = identifier.trim().split('/').collect();

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
            _ => None,
        }
    }

    pub fn schema(identifier: &str) -> Option<Self> {
        Self::from_identifier(identifier).filter(|resource| resource.kind == ResourceKind::Schema)
    }

    pub fn bundle(identifier: &str) -> Option<Self> {
        Self::from_identifier(identifier).filter(|resource| resource.kind == ResourceKind::Bundle)
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
    fn rejects_two_part_identifiers() {
        assert_eq!(RegistryResource::from_identifier("acme/payment"), None);
        assert_eq!(RegistryResource::schema("acme/payment"), None);
        assert_eq!(RegistryResource::bundle("acme/billing"), None);
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
    fn rejects_incomplete_canonical_bundle_identifiers() {
        assert_eq!(RegistryResource::bundle("acme/bundles"), None);
    }

    #[test]
    fn displays_canonical_package_keys() {
        assert_eq!(
            display_package_key("schema:acme/schemas/root"),
            "acme/schemas/root"
        );
        assert_eq!(
            display_package_key("bundle:acme/bundles/common"),
            "acme/bundles/common"
        );
    }

    #[test]
    fn builds_package_keys_for_canonical_identifiers() {
        assert_eq!(
            package_key_for(ResourceKind::Bundle, "acme/bundles/common"),
            Some("bundle:acme/bundles/common".to_string())
        );
        assert_eq!(
            package_key_for(ResourceKind::Schema, "acme/schemas/common"),
            Some("schema:acme/schemas/common".to_string())
        );
        assert_eq!(
            package_key_from_identifier("acme/bundles/common"),
            Some("bundle:acme/bundles/common".to_string())
        );
        assert_eq!(
            package_key_from_identifier("acme/schemas/common"),
            Some("schema:acme/schemas/common".to_string())
        );
        assert_eq!(package_key_from_identifier("acme/common"), None);
    }
}

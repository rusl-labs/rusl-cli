pub fn embedded_feedback_schema_json(slug: &str) -> Option<&'static str> {
    Some(match slug {
        "context-loading-hint" => {
            include_str!("../assets/feedback-schemas/context-loading-hint.schema.json")
        }
        "usage-report" => include_str!("../assets/feedback-schemas/usage-report.schema.json"),
        "domain-interpretation" => {
            include_str!("../assets/feedback-schemas/domain-interpretation.schema.json")
        }
        "semantic-link" => include_str!("../assets/feedback-schemas/semantic-link.schema.json"),
        "trust-signal" => include_str!("../assets/feedback-schemas/trust-signal.schema.json"),
        "context-request" => include_str!("../assets/feedback-schemas/context-request.schema.json"),
        "source-attestation" => {
            include_str!("../assets/feedback-schemas/source-attestation.schema.json")
        }
        "migration-guide" => include_str!("../assets/feedback-schemas/migration-guide.schema.json"),
        _ => return None,
    })
}

use rmcp::ErrorData;
use serde_json::json;

pub(super) fn to_mcp_error(error: anyhow::Error) -> ErrorData {
    let causes = error.chain().map(ToString::to_string).collect::<Vec<_>>();
    let message = causes.join(": ");

    ErrorData::internal_error(message, Some(json!({ "causes": causes })))
}

#[cfg(test)]
mod tests {
    use super::to_mcp_error;

    #[test]
    fn mcp_errors_include_the_full_cause_chain() {
        let error = anyhow::anyhow!("socket closed")
            .context("Rusl API request failed before receiving a response")
            .context("Failed to search registry at https://example.test/api/search");

        let mcp_error = to_mcp_error(error);

        assert_eq!(
            mcp_error.message,
            "Failed to search registry at https://example.test/api/search: Rusl API request failed before receiving a response: socket closed"
        );
        assert_eq!(
            mcp_error.data.expect("error data")["causes"][0],
            "Failed to search registry at https://example.test/api/search"
        );
    }
}

use turbomcp::prelude::McpError;

pub(super) fn to_mcp_error(error: anyhow::Error) -> McpError {
    McpError::internal(mcp_error_message(&error))
}

pub(super) fn mcp_error_message(error: &anyhow::Error) -> String {
    let causes = error.chain().map(ToString::to_string).collect::<Vec<_>>();
    causes.join(": ")
}

#[cfg(test)]
mod tests {
    use super::mcp_error_message;

    #[test]
    fn mcp_errors_include_the_full_cause_chain() {
        let error = anyhow::anyhow!("socket closed")
            .context("Rusl API request failed before receiving a response")
            .context("Failed to search registry at https://example.test/api/search");

        let message = mcp_error_message(&error);

        assert_eq!(
            message,
            "Failed to search registry at https://example.test/api/search: Rusl API request failed before receiving a response: socket closed"
        );
    }
}

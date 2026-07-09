use crate::{config, registry::client::RegistryClient};
use anyhow::{Context, Result};
use rusl_api_client::{models, rusl_user_agent_with_context};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSchemaExamplesRequest {
    pub account_slug: String,
    pub schema_slug: String,
    pub version: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaExamplesOutput {
    pub examples: Vec<SchemaExampleOutput>,
    pub page_info: Option<SchemaExamplesPageInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaExampleOutput {
    pub id: String,
    pub schema_version_id: String,
    pub version: String,
    pub position: i32,
    pub title: Option<String>,
    pub data: Value,
    pub inserted_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaExamplesPageInfo {
    pub current_page: Option<i32>,
    pub page_size: Option<i32>,
    pub total_count: Option<i32>,
    pub total_pages: Option<i32>,
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub next_page: Option<i32>,
    pub previous_page: Option<i32>,
}

pub async fn list_schema_examples_with_user_agent_context(
    request: ListSchemaExamplesRequest,
    context: &str,
) -> Result<SchemaExamplesOutput> {
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let response = client
        .list_schema_examples(
            &request.account_slug,
            &request.schema_slug,
            request.version,
            request.page,
            request.per_page,
        )
        .await?;

    Ok(map_schema_examples_response(response))
}

fn map_schema_examples_response(
    response: models::RuslWebApiSchemaVersionControllerExampleDataIndex200Response,
) -> SchemaExamplesOutput {
    SchemaExamplesOutput {
        examples: response
            .data
            .unwrap_or_default()
            .into_iter()
            .map(map_schema_example)
            .collect(),
        page_info: response
            .page_info
            .map(|page_info| map_page_info(*page_info)),
    }
}

fn map_schema_example(example: models::SchemaExampleData1) -> SchemaExampleOutput {
    SchemaExampleOutput {
        id: example.id,
        schema_version_id: example.schema_version_id,
        version: example.version,
        position: example.position,
        title: example.title.flatten(),
        data: example.data.unwrap_or(Value::Null),
        inserted_at: example.inserted_at,
        updated_at: example.updated_at,
    }
}

fn map_page_info(
    page_info: models::RuslWebApiSchemaVersionControllerExampleDataIndex200ResponsePageInfo,
) -> SchemaExamplesPageInfo {
    SchemaExamplesPageInfo {
        current_page: page_info.current_page.flatten(),
        page_size: page_info.page_size.flatten(),
        total_count: page_info.total_count.flatten(),
        total_pages: page_info.total_pages.flatten(),
        has_next_page: page_info.has_next_page,
        has_previous_page: page_info.has_previous_page,
        next_page: page_info.next_page.flatten(),
        previous_page: page_info.previous_page.flatten(),
    }
}

#[cfg(test)]
mod tests {
    use super::map_schema_examples_response;
    use rusl_api_client::models;
    use serde_json::json;

    #[test]
    fn maps_schema_examples_and_pagination_metadata() {
        let mut example = models::SchemaExampleData1::new(
            Some(json!({ "name": "Example" })),
            "example-1".to_string(),
            "2026-05-20T00:00:00Z".to_string(),
            1,
            "version-1".to_string(),
            "2026-05-20T00:00:00Z".to_string(),
            "1.2.0".to_string(),
        );
        example.title = Some(Some("Minimal object".to_string()));

        let mut page_info =
            models::RuslWebApiSchemaVersionControllerExampleDataIndex200ResponsePageInfo::new(
                true, false,
            );
        page_info.current_page = Some(Some(1));
        page_info.page_size = Some(Some(20));
        page_info.total_count = Some(Some(21));
        page_info.total_pages = Some(Some(2));
        page_info.next_page = Some(Some(2));

        let output = map_schema_examples_response(
            models::RuslWebApiSchemaVersionControllerExampleDataIndex200Response {
                data: Some(vec![example]),
                page_info: Some(Box::new(page_info)),
            },
        );

        assert_eq!(output.examples.len(), 1);
        assert_eq!(output.examples[0].version, "1.2.0");
        assert_eq!(output.examples[0].title, Some("Minimal object".to_string()));
        let page_info = output.page_info.expect("page info");
        assert_eq!(page_info.current_page, Some(1));
        assert_eq!(page_info.page_size, Some(20));
        assert_eq!(page_info.total_count, Some(21));
        assert!(page_info.has_next_page);
    }
}

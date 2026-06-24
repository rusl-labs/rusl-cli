use crate::{config, config::credentials::Credentials, registry::client::RegistryClient};
use anyhow::{Context, Result, bail};
use rusl_api_client::{models, rusl_user_agent_with_context};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaVisibility {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateSchemaRequest {
    pub account_slug: String,
    pub schema_slug: String,
    pub description: Option<String>,
    pub visibility: SchemaVisibility,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateSchemaProposalRequest {
    pub account_slug: String,
    pub schema_slug: String,
    pub content: Value,
    pub description: Option<String>,
    pub valid_data: Vec<ExampleDataInput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSchemaProposalRequest {
    pub account_slug: String,
    pub schema_slug: String,
    pub proposal_number: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateSchemaProposalRequest {
    pub account_slug: String,
    pub schema_slug: String,
    pub proposal_number: i32,
    pub content: Value,
    pub description: Option<String>,
    pub valid_data: Vec<ExampleDataInput>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExampleDataInput {
    pub data: Value,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProposalReviewThreadsRequest {
    pub account_slug: String,
    pub schema_slug: String,
    pub proposal_number: i32,
    pub include_comments: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetProposalReviewThreadRequest {
    pub account_slug: String,
    pub schema_slug: String,
    pub proposal_number: i32,
    pub thread_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateProposalReviewThreadRequest {
    pub account_slug: String,
    pub schema_slug: String,
    pub proposal_number: i32,
    pub body_text: String,
    pub body_meta: Option<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReplyToProposalReviewThreadRequest {
    pub account_slug: String,
    pub schema_slug: String,
    pub proposal_number: i32,
    pub thread_id: String,
    pub parent_comment_id: Option<String>,
    pub body_text: String,
    pub body_meta: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaOutput {
    pub id: String,
    pub guid: Option<String>,
    pub account_slug: String,
    pub schema_slug: String,
    pub schema_identifier: String,
    pub description: Option<String>,
    pub schema_format: String,
    pub status: String,
    pub visibility: Option<String>,
    pub inserted_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProposalOutput {
    pub id: String,
    pub guid: String,
    pub schema_id: String,
    pub proposal_number: Option<i32>,
    pub status: String,
    pub description: Option<String>,
    pub proposed_version: Option<String>,
    pub based_on_version: Option<String>,
    pub minimum_bump_type: Option<String>,
    pub outdated: bool,
    pub root_instance_types: Vec<String>,
    pub valid_data_count: usize,
    pub inserted_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProposalDetailOutput {
    pub proposal: ProposalOutput,
    pub content: Value,
    pub valid_data: Vec<ExampleDataOutput>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExampleDataOutput {
    pub data: Value,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProposalReviewThreadsOutput {
    pub threads: Vec<ReviewThreadOutput>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewThreadOutput {
    pub id: String,
    pub status: String,
    pub subject_guid: String,
    pub created_by_user_id: String,
    pub inserted_at: String,
    pub updated_at: String,
    pub comment_count: usize,
    pub bookmark: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<ReviewCommentOutput>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewCommentOutput {
    pub id: String,
    pub thread_id: String,
    pub parent_comment_id: Option<String>,
    pub depth: i32,
    pub body_format: String,
    pub body_text: String,
    pub body_meta: Value,
    pub created_by_user_id: String,
    pub inserted_at: String,
    pub updated_at: String,
}

pub async fn create_schema_with_user_agent_context(
    request: CreateSchemaRequest,
    context: &str,
) -> Result<SchemaOutput> {
    require_login()?;
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let account_slug = request.account_slug.clone();
    let response = client
        .create_schema(&account_slug, to_api_create_schema_request(request))
        .await?;
    let Some(schema) = response.data.flatten() else {
        bail!("Rusl API did not return created schema data");
    };

    Ok(map_schema(*schema))
}

pub async fn create_schema_proposal_with_user_agent_context(
    request: CreateSchemaProposalRequest,
    context: &str,
) -> Result<ProposalOutput> {
    require_login()?;
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let account_slug = request.account_slug.clone();
    let schema_slug = request.schema_slug.clone();
    let api_request = to_api_create_schema_proposal_request(request);
    let response = client
        .create_schema_proposal(&account_slug, &schema_slug, api_request)
        .await?;
    let Some(proposal) = response.data else {
        bail!("Rusl API did not return created proposal data");
    };

    Ok(map_proposal(*proposal))
}

pub async fn get_schema_proposal_with_user_agent_context(
    request: GetSchemaProposalRequest,
    context: &str,
) -> Result<ProposalDetailOutput> {
    require_login()?;
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let response = client
        .fetch_schema_proposal(
            &request.account_slug,
            &request.schema_slug,
            request.proposal_number,
        )
        .await?;
    let Some(proposal) = response.data else {
        bail!("Rusl API did not return schema proposal data");
    };

    Ok(map_proposal_detail(*proposal))
}

pub async fn update_schema_proposal_with_user_agent_context(
    request: UpdateSchemaProposalRequest,
    context: &str,
) -> Result<ProposalOutput> {
    require_login()?;
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let account_slug = request.account_slug.clone();
    let schema_slug = request.schema_slug.clone();
    let proposal_number = request.proposal_number;
    let api_request = to_api_update_schema_proposal_request(request);
    let response = client
        .update_schema_proposal(&account_slug, &schema_slug, proposal_number, api_request)
        .await?;
    let Some(proposal) = response.data else {
        bail!("Rusl API did not return updated proposal data");
    };

    Ok(map_proposal(*proposal))
}

pub async fn list_proposal_review_threads_with_user_agent_context(
    request: ProposalReviewThreadsRequest,
    context: &str,
) -> Result<ProposalReviewThreadsOutput> {
    require_login()?;
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let response = client
        .list_proposal_review_threads(
            &request.account_slug,
            &request.schema_slug,
            request.proposal_number,
        )
        .await?;

    Ok(ProposalReviewThreadsOutput {
        threads: response
            .data
            .unwrap_or_default()
            .into_iter()
            .map(|thread| map_review_thread(thread, request.include_comments))
            .collect(),
    })
}

pub async fn get_proposal_review_thread_with_user_agent_context(
    request: GetProposalReviewThreadRequest,
    context: &str,
) -> Result<ReviewThreadOutput> {
    let thread_id = request.thread_id.clone();
    let threads = list_proposal_review_threads_with_user_agent_context(
        ProposalReviewThreadsRequest {
            account_slug: request.account_slug,
            schema_slug: request.schema_slug,
            proposal_number: request.proposal_number,
            include_comments: true,
        },
        context,
    )
    .await?;

    threads
        .threads
        .into_iter()
        .find(|thread| thread.id == thread_id)
        .with_context(|| format!("Proposal review thread {thread_id} was not found"))
}

pub async fn create_proposal_review_thread_with_user_agent_context(
    request: CreateProposalReviewThreadRequest,
    context: &str,
) -> Result<ReviewThreadOutput> {
    require_login()?;
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let account_slug = request.account_slug.clone();
    let schema_slug = request.schema_slug.clone();
    let proposal_number = request.proposal_number;
    let api_request = to_api_create_review_thread_request(request);
    let response = client
        .create_proposal_review_thread(&account_slug, &schema_slug, proposal_number, api_request)
        .await?;
    let Some(thread) = response.data else {
        bail!("Rusl API did not return created review thread data");
    };

    Ok(map_review_thread(*thread, true))
}

pub async fn reply_to_proposal_review_thread_with_user_agent_context(
    request: ReplyToProposalReviewThreadRequest,
    context: &str,
) -> Result<ReviewCommentOutput> {
    require_login()?;
    let parent_comment_id = match request.parent_comment_id.clone() {
        Some(parent_comment_id) => parent_comment_id,
        None => root_comment_id_for_thread(&request, context).await?,
    };

    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let account_slug = request.account_slug.clone();
    let schema_slug = request.schema_slug.clone();
    let proposal_number = request.proposal_number;
    let thread_id = request.thread_id.clone();
    let api_request = to_api_create_review_comment_request(request, parent_comment_id);
    let response = client
        .create_proposal_review_comment(
            &account_slug,
            &schema_slug,
            proposal_number,
            &thread_id,
            api_request,
        )
        .await?;
    let Some(comment) = response.data else {
        bail!("Rusl API did not return created review comment data");
    };

    Ok(map_review_comment(*comment))
}

fn require_login() -> Result<()> {
    let Some(credentials) = Credentials::load() else {
        bail!("This tool requires user authorization. Run `rusl login` first.");
    };

    if credentials.access_token.trim().is_empty() && credentials.refresh_token.trim().is_empty() {
        bail!("This tool requires user authorization. Run `rusl login` first.");
    }

    Ok(())
}

async fn root_comment_id_for_thread(
    request: &ReplyToProposalReviewThreadRequest,
    context: &str,
) -> Result<String> {
    let thread = get_proposal_review_thread_with_user_agent_context(
        GetProposalReviewThreadRequest {
            account_slug: request.account_slug.clone(),
            schema_slug: request.schema_slug.clone(),
            proposal_number: request.proposal_number,
            thread_id: request.thread_id.clone(),
        },
        context,
    )
    .await?;

    let comments = thread.comments.unwrap_or_default();
    comments
        .into_iter()
        .find(|comment| comment.depth == 0 || comment.parent_comment_id.is_none())
        .map(|comment| comment.id)
        .with_context(|| {
            format!(
                "Review thread {} did not include a root comment",
                request.thread_id
            )
        })
}

fn to_api_create_schema_request(request: CreateSchemaRequest) -> models::OpenApiSchema1 {
    let mut api_request = models::OpenApiSchema1::new(request.schema_slug);
    api_request.description = request.description.map(Some);
    api_request.schema_format = Some(models::open_api_schema_1::SchemaFormat::JsonSchema);
    api_request.visibility = Some(match request.visibility {
        SchemaVisibility::Public => models::open_api_schema_1::Visibility::Public,
        SchemaVisibility::Private => models::open_api_schema_1::Visibility::Private,
    });
    api_request
}

fn to_api_create_schema_proposal_request(
    request: CreateSchemaProposalRequest,
) -> models::OpenApiSchema2 {
    let mut api_request = models::OpenApiSchema2::new(
        request.content,
        request.valid_data.into_iter().map(Into::into).collect(),
    );
    api_request.description = request.description;
    api_request
}

fn to_api_update_schema_proposal_request(
    request: UpdateSchemaProposalRequest,
) -> models::OpenApiSchema3 {
    let mut api_request = models::OpenApiSchema3::new(
        request.content,
        request.valid_data.into_iter().map(Into::into).collect(),
    );
    api_request.description = request.description;
    api_request
}

fn to_api_create_review_thread_request(
    request: CreateProposalReviewThreadRequest,
) -> models::CreateReviewThreadRequest1 {
    let mut api_request = models::CreateReviewThreadRequest1::new(request.body_text);
    api_request.body_format = Some(models::create_review_thread_request_1::BodyFormat::Markdown);
    api_request.body_meta = request.body_meta;
    api_request
}

fn to_api_create_review_comment_request(
    request: ReplyToProposalReviewThreadRequest,
    parent_comment_id: String,
) -> models::CreateReviewCommentRequest1 {
    let mut api_request =
        models::CreateReviewCommentRequest1::new(request.body_text, parent_comment_id);
    api_request.body_format = Some(models::create_review_comment_request_1::BodyFormat::Markdown);
    api_request.body_meta = request.body_meta;
    api_request
}

impl From<ExampleDataInput> for models::ExampleData1 {
    fn from(input: ExampleDataInput) -> Self {
        let mut example = models::ExampleData1::new(input.data);
        example.title = input.title;
        example
    }
}

fn map_schema(schema: models::Schema1) -> SchemaOutput {
    SchemaOutput {
        id: schema.id,
        guid: schema.guid,
        schema_identifier: schema.identifier,
        account_slug: schema.account_slug,
        schema_slug: schema.slug,
        description: schema.description.flatten(),
        schema_format: schema_format_label(schema.schema_format).to_string(),
        status: schema_status_label(schema.status).to_string(),
        visibility: schema
            .visibility
            .map(schema_visibility_label)
            .map(str::to_string),
        inserted_at: schema.inserted_at,
        updated_at: schema.updated_at,
    }
}

fn map_proposal(proposal: models::Proposal1) -> ProposalOutput {
    ProposalOutput {
        id: proposal.id,
        guid: proposal.guid,
        schema_id: proposal.schema_id,
        proposal_number: proposal.proposal_number,
        status: proposal_status_label(proposal.status).to_string(),
        description: proposal.description.flatten(),
        proposed_version: proposal.proposed_version.flatten(),
        based_on_version: proposal.based_on_version.flatten(),
        minimum_bump_type: proposal
            .minimum_bump_type
            .flatten()
            .map(proposal_bump_type_label)
            .map(str::to_string),
        outdated: proposal.outdated.unwrap_or(false),
        root_instance_types: proposal.root_instance_types,
        valid_data_count: proposal.valid_data.len(),
        inserted_at: proposal.inserted_at,
        updated_at: proposal.updated_at,
    }
}

fn map_proposal_detail(proposal: models::Proposal1) -> ProposalDetailOutput {
    let content = proposal.content.clone();
    let valid_data = proposal
        .valid_data
        .iter()
        .cloned()
        .map(map_example_data)
        .collect();

    ProposalDetailOutput {
        proposal: map_proposal(proposal),
        content,
        valid_data,
    }
}

fn map_example_data(example: models::ExampleData1) -> ExampleDataOutput {
    ExampleDataOutput {
        data: example.data,
        title: example.title,
    }
}

fn map_review_thread(thread: models::ReviewThread1, include_comments: bool) -> ReviewThreadOutput {
    let comments = thread.comments.flatten().unwrap_or_default();
    let comment_count = comments.len();
    let comments = include_comments.then(|| comments.into_iter().map(map_review_comment).collect());

    ReviewThreadOutput {
        id: thread.id,
        status: review_thread_status_label(thread.status).to_string(),
        subject_guid: thread.subject_guid,
        created_by_user_id: thread.created_by_user_id,
        inserted_at: thread.inserted_at,
        updated_at: thread.updated_at,
        comment_count,
        bookmark: thread
            .bookmark
            .flatten()
            .and_then(|bookmark| serde_json::to_value(bookmark).ok()),
        comments,
    }
}

fn map_review_comment(comment: models::ReviewComment1) -> ReviewCommentOutput {
    ReviewCommentOutput {
        id: comment.id,
        thread_id: comment.thread_id,
        parent_comment_id: comment.parent_comment_id.flatten(),
        depth: review_comment_depth_value(comment.depth),
        body_format: review_comment_body_format_label(comment.body_format).to_string(),
        body_text: comment.body_text,
        body_meta: comment.body_meta,
        created_by_user_id: comment.created_by_user_id,
        inserted_at: comment.inserted_at,
        updated_at: comment.updated_at,
    }
}

fn schema_format_label(format: models::schema_1::SchemaFormat) -> &'static str {
    match format {
        models::schema_1::SchemaFormat::JsonSchema => "JSON_SCHEMA",
    }
}

fn schema_status_label(status: models::schema_1::Status) -> &'static str {
    match status {
        models::schema_1::Status::Active => "ACTIVE",
        models::schema_1::Status::Archived => "ARCHIVED",
    }
}

fn schema_visibility_label(visibility: models::schema_1::Visibility) -> &'static str {
    match visibility {
        models::schema_1::Visibility::Public => "PUBLIC",
        models::schema_1::Visibility::Private => "PRIVATE",
    }
}

fn proposal_status_label(status: models::proposal_1::Status) -> &'static str {
    match status {
        models::proposal_1::Status::Pending => "PENDING",
        models::proposal_1::Status::Accepted => "ACCEPTED",
        models::proposal_1::Status::Rejected => "REJECTED",
        models::proposal_1::Status::Closed => "CLOSED",
    }
}

fn proposal_bump_type_label(bump_type: models::proposal_1::MinimumBumpType) -> &'static str {
    match bump_type {
        models::proposal_1::MinimumBumpType::Major => "major",
        models::proposal_1::MinimumBumpType::Minor => "minor",
        models::proposal_1::MinimumBumpType::Patch => "patch",
    }
}

fn review_thread_status_label(status: models::review_thread_1::Status) -> &'static str {
    match status {
        models::review_thread_1::Status::Open => "open",
        models::review_thread_1::Status::Resolved => "resolved",
    }
}

fn review_comment_body_format_label(format: models::review_comment_1::BodyFormat) -> &'static str {
    match format {
        models::review_comment_1::BodyFormat::Markdown => "markdown",
    }
}

fn review_comment_depth_value(depth: models::review_comment_1::Depth) -> i32 {
    match depth {
        models::review_comment_1::Depth::Variant0 => 0,
        models::review_comment_1::Depth::Variant1 => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CreateSchemaRequest, ExampleDataInput, SchemaVisibility, UpdateSchemaProposalRequest,
        map_review_thread, to_api_create_schema_request, to_api_update_schema_proposal_request,
    };
    use rusl_api_client::models;
    use serde_json::json;

    #[test]
    fn create_schema_request_requires_explicit_visibility() {
        let request = to_api_create_schema_request(CreateSchemaRequest {
            account_slug: "hassox".to_string(),
            schema_slug: "contracts/thing".to_string(),
            description: Some("A thing contract".to_string()),
            visibility: SchemaVisibility::Private,
        });

        assert_eq!(request.slug, "contracts/thing");
        assert_eq!(
            request.description,
            Some(Some("A thing contract".to_string()))
        );
        assert_eq!(
            request.visibility,
            Some(models::open_api_schema_1::Visibility::Private)
        );
    }

    #[test]
    fn review_thread_mapping_strips_comments_by_default_but_keeps_count() {
        let thread = review_thread_with_comments();

        let output = map_review_thread(thread, false);

        assert_eq!(output.comment_count, 2);
        assert!(output.comments.is_none());
    }

    #[test]
    fn review_thread_mapping_includes_comments_when_requested() {
        let thread = review_thread_with_comments();

        let output = map_review_thread(thread, true);

        assert_eq!(output.comment_count, 2);
        let comments = output.comments.expect("comments included");
        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].depth, 0);
        assert_eq!(
            comments[1].parent_comment_id,
            Some("root-comment".to_string())
        );
    }

    #[test]
    fn update_schema_proposal_request_uses_full_replacement_content_and_examples() {
        let request = to_api_update_schema_proposal_request(UpdateSchemaProposalRequest {
            account_slug: "hassox".to_string(),
            schema_slug: "contracts/thing".to_string(),
            proposal_number: 7,
            content: json!({ "type": "object" }),
            description: Some("Revised shape".to_string()),
            valid_data: vec![ExampleDataInput {
                data: json!({}),
                title: Some("empty object".to_string()),
            }],
        });

        assert_eq!(request.content, json!({ "type": "object" }));
        assert_eq!(request.description, Some("Revised shape".to_string()));
        assert_eq!(request.valid_data.len(), 1);
        assert_eq!(
            request.valid_data[0].title,
            Some("empty object".to_string())
        );
    }

    fn review_thread_with_comments() -> models::ReviewThread1 {
        let mut root = models::ReviewComment1::new(
            models::review_comment_1::Typename::ReviewComments,
            models::review_comment_1::BodyFormat::Markdown,
            json!({}),
            "Please check this".to_string(),
            "user-1".to_string(),
            models::review_comment_1::Depth::Variant0,
            "root-comment".to_string(),
            "2026-05-20T00:00:00Z".to_string(),
            "thread-1".to_string(),
            "2026-05-20T00:00:00Z".to_string(),
        );
        root.parent_comment_id = Some(None);

        let mut reply = models::ReviewComment1::new(
            models::review_comment_1::Typename::ReviewComments,
            models::review_comment_1::BodyFormat::Markdown,
            json!({ "source": "mcp" }),
            "Checked".to_string(),
            "user-2".to_string(),
            models::review_comment_1::Depth::Variant1,
            "reply-comment".to_string(),
            "2026-05-20T00:01:00Z".to_string(),
            "thread-1".to_string(),
            "2026-05-20T00:01:00Z".to_string(),
        );
        reply.parent_comment_id = Some(Some("root-comment".to_string()));

        let mut thread = models::ReviewThread1::new(
            models::review_thread_1::Typename::ReviewThreads,
            "user-1".to_string(),
            "thread-1".to_string(),
            "2026-05-20T00:00:00Z".to_string(),
            models::review_thread_1::Status::Open,
            "schema_proposals.proposal-1".to_string(),
            "2026-05-20T00:01:00Z".to_string(),
        );
        thread.comments = Some(Some(vec![root, reply]));
        thread
    }
}

# Rusl MCP Tools

The Rusl MCP server gives agents a structured way to discover schemas, inspect examples, manage proposals, and attach feedback to Rusl resources.

| Tool | Short description |
|---|---|
| `search` | Search visible Rusl schemas, bundles, annotation types, and annotations. |
| `list_schema_examples` | Fetch committed examples for a schema before using it or drafting a proposal. |
| `endorse` | Endorse an annotation as a positive trust signal. |
| `create_schema` | Create a schema namespace before proposing its first content. |
| `create_schema_proposal` | Propose a complete JSON Schema revision for an existing schema. |
| `get_schema_proposal` | Read proposal content, examples, status, and version metadata before editing. |
| `update_schema_proposal` | Replace a pending proposal with revised complete content and examples. |
| `list_proposal_review_threads` | Inspect proposal review threads; comments are omitted unless requested. |
| `get_proposal_review_thread` | Read one proposal review thread with comments. |
| `create_proposal_review_thread` | Start a new review thread on a proposal. |
| `reply_to_proposal_review_thread` | Reply in an existing proposal review thread. |
| `create_context_loading_hint` | Add guidance about what context to load before using a schema or contract. |
| `create_usage_report` | Record how a schema or contract was used in a project, tool, or workflow. |
| `create_domain_interpretation` | Explain what a schema, field, or contract means in a specific domain. |
| `create_semantic_link` | Link related schemas, fields, annotations, docs, or external sources. |
| `create_trust_signal` | Add evidence that helps users judge whether a schema or annotation is reliable. |
| `create_context_request` | Ask for missing context when a schema or contract cannot be used safely. |
| `create_source_attestation` | Connect a schema, contract, or annotation to its source of authority. |
| `create_migration_guide` | Document compatibility, breaking changes, and upgrade guidance between versions. |

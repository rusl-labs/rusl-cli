# Rusl MCP Tools

The Rusl MCP server gives agents a structured way to discover schemas, inspect examples, manage proposals, register bundles and annotation types, and attach feedback to Rusl resources.

Search is a discovery surface, not a content retrieval surface. When an agent needs the full content of a specific schema, bundle, annotation type, or annotation, use `get_schema`, `get_bundle`, `get_annotation_type`, or `get_annotation`; search results intentionally do not include the full object content.

| Tool | Short description |
|---|---|
| `search` | Search visible Rusl schemas, bundles, annotation types, and annotations. |
| `get_schema` | Fetch the full schema show record, or a specific schema version record when `version` is set. |
| `get_bundle` | Fetch the full bundle show record, or a specific bundle version record when `version` is set. |
| `get_annotation_type` | Fetch the full registered annotation type record. |
| `get_annotation` | Fetch the full annotation record by raw ID or `annotations.<id>` GUID. |
| `list_schema_examples` | Fetch committed examples for a schema before using it or drafting a proposal. |
| `endorse` | Endorse an annotation as a positive trust signal. |
| `create_schema` | Create a schema namespace before proposing its first content. Accepts an optional `package` to place it in a package namespace. |
| `create_schema_proposal` | Propose a complete JSON Schema revision for an existing schema. |
| `get_schema_proposal` | Read proposal content, examples, status, and version metadata before editing. |
| `update_schema_proposal` | Replace a pending proposal with revised complete content and examples. |
| `accept_schema_proposal` | Accept a pending proposal and publish its content as a new schema version. |
| `list_proposal_review_threads` | Inspect proposal review threads; comments are omitted unless requested. |
| `get_proposal_review_thread` | Read one proposal review thread with comments. |
| `create_proposal_review_thread` | Start a new review thread on a proposal. |
| `reply_to_proposal_review_thread` | Reply in an existing proposal review thread. |
| `create_bundle` | Create a bundle namespace before publishing bundle content. |
| `create_bundle_version` | Create a draft bundle version with manifest content. |
| `publish_bundle_version` | Publish a draft bundle version after validation. |
| `create_annotation_type` | Register a new annotation type with validation schema linkage. |
| `create_annotation` | Create an annotation of any registered type against a visible subject; content is validated server-side. |
| `create_context_loading_hint` | Add guidance about what context to load before using a schema or contract. |
| `create_usage_report` | Record how a schema or contract was used in a project, tool, or workflow. |
| `create_domain_interpretation` | Explain what a schema, field, or contract means in a specific domain. |
| `create_semantic_link` | Link related schemas, fields, annotations, docs, or external sources. |
| `create_trust_signal` | Add evidence that helps users judge whether a schema or annotation is reliable. |
| `create_context_request` | Ask for missing context when a schema or contract cannot be used safely. |
| `create_source_attestation` | Connect a schema, contract, or annotation to its source of authority. |
| `create_migration_guide` | Document compatibility, breaking changes, and upgrade guidance between versions. |

Use canonical identifiers in tool arguments and results: schemas are `account/schemas/slug`, bundles are `account/bundles/slug`, and annotation types are `account/annotation-types/slug`.

A schema's final identifier segment is an opaque compound: for a packaged schema it folds the dotted package path onto the leaf (`acme/schemas/payments.checkout` is the `checkout` schema in the `payments` package). Do not split it — pass identifiers through verbatim, and route follow-up proposal, version, and example tool calls with the whole compound (`payments.checkout`), never the bare leaf (`checkout`), which resolves to a different schema or none. Create a packaged schema by passing `create_schema` a `slug` (the leaf, no dots) plus an optional `package` (the dotted path, e.g. `payments`); omit `package` to leave the schema not packaged. `package` is immutable after creation. The `create_schema` result reports the compound in `schema_slug` and the full routing handle in `identifier`.

For exact search by returned resource identifier, use the search tool's `identifiers` field. This works for schemas, bundles, and annotation types. To fetch paged annotations for a known schema, bundle, annotation type, or other subject, search with `types: ["annotation"]`, `subject_identifier_prefix`, `page`, and `per_page`. To narrow by annotation type, add `type_identifiers`.

Feedback annotation tools validate content against embedded Rusl feedback schemas, so they work without a local `rusl install` of `rusl/bundles/feedback-schemas`.

Use `create_annotation` when the target annotation type is not one of the feedback kinds (for example, `rusl/annotation-types/storage-policy` consumed by rusl-kv). The typed `create_<kind>` tools remain the preferred surface for feedback annotations because they validate content locally before the API call.

# Schema-Driven Repository

This repository is schema-driven.

`/schemas` is the authoritative home for data shape definitions in this project. A data shape is any structured payload that crosses a boundary: CLI input/output, config files, manifests, lockfiles, MCP tool contracts, API payloads, generated protocol messages, persisted records, or agent-facing context.

## Iron Rule

No code is created, modified, or generated for a data type before its schema has been proposed and approved.

When a task touches a data shape:

1. Read this file first.
2. Check `/schemas` for an existing definition.
3. Reuse an existing schema before defining a new one.
4. If no schema exists, propose one before changing code.
5. After approval, derive implementation artifacts from the schema wherever practical.

## Schema Location

Authoritative project schemas live under:

```text
schemas/
```

Installed or vendored registry snapshots may live below that tree, for example:

```text
schemas/vendor/
```

The configured `output.schema_dir` decides where Rusl installs resolved schema files. The built-in default is `./schemas`.

## Definition Standard

New first-party schemas should be strict JSON Schema using draft 2020-12 unless a task explicitly chooses another schema language.

Each schema should:

1. Declare one concept.
2. Use stable names and explicit required fields.
3. Reject undeclared properties unless extension is intentional.
4. Include enough description to identify shape, not every consumer-specific meaning.
5. Version through review rather than silent mutation.

## Meaning Layer

Schemas define shape. Operational meaning belongs beside the shape, not hidden in code or prompts.

Examples of meaning include masking rules, retention policy, domain interpretation, migration notes, usage reports, trust signals, and context-loading guidance. Capture these as typed annotations or schema-backed context where the tooling supports it.

## Existing Code

Existing implicit shapes are not automatically converted by this file. When touching an existing shape, first decide whether to adopt, reuse, or propose the schema for that shape. Do not add another parallel definition just to complete the immediate code change.

For generated OpenAPI client types, the backend OpenAPI spec remains the boundary source of truth. Do not hand-edit generated client models; update the upstream definition and regenerate.

## Agent Rule

Agents working in this repository must treat schema decisions as architecture decisions. If a requested code change requires inventing or changing a data shape, stop at the schema proposal and ask for approval before implementing the code that depends on it.

# Rusl Agent Directives

These rules are mechanical. Follow them unless the user explicitly overrides a specific item.

## Execution Model

1. **Phase work**
   - Keep changes in small phases, usually 3-4 files at a time.
   - Verify each phase before claiming progress.
   - If a file is large and messy, clean dead code before major structural edits unless the user explicitly says to push through.

2. **No broken windows**
   - Fix compile errors, clippy warnings, and test failures.
   - Do not treat existing failures as acceptable background noise.

3. **Required verification**
   - Run:
     - `cargo check --workspace`
     - `cargo clippy --workspace --all-targets -- -D warnings`
     - `cargo test --workspace`
     - `cargo fmt --check`
     - `cargo build --workspace --release`
   - Prefer `make verify` when working from the repo root.

## Clean Architecture

4. **Keep boundaries explicit**
   - `crates/rusl-cli` is the CLI gateway.
   - `crates/rusl-app` is the shared application/core layer.
   - `crates/rusl-api-client` is the HTTP client boundary.
   - Future gateways, including MCP/STDIO, should call into `crates/rusl-app`, not duplicate logic.

5. **What belongs in `crates/rusl-cli`**
   - clap argument parsing
   - terminal rendering
   - progress spinners
   - user-facing output formatting
   - transport-specific callback handling for local login flow

6. **What belongs in `crates/rusl-app`**
   - use-case orchestration
   - manifest/lockfile handling
   - dependency resolution flow
   - generation flow
   - transport-agnostic login/session logic
   - shared logic that must work for CLI and future non-CLI gateways

7. **What belongs in `crates/rusl-api-client`**
   - generated OpenAPI bindings in `generated/`
   - handwritten wrapper code in `src/`
   - bearer token injection
   - refresh-token exchange
   - one-time retry on `401`
   - HTTP error normalization

8. **What must not happen**
   - do not put business logic in clap handlers
   - do not hand-roll new HTTP calls inside CLI commands
   - do not let app/core code depend on terminal rendering concerns
   - do not bypass the shared auth wrapper when adding authenticated API calls

## API and Auth Rules

9. **Network config**
   - `api_base_url` and `website_url` are independent.
   - Never derive one from the other.
   - Respect config/env resolution already in the app layer.

10. **Token handling**
    - Requests may be public; if there is no refresh token, call anyway.
    - If there is no access token but there is a refresh token, exchange it first.
    - If refresh exchange fails, clear stored auth state and continue unauthenticated.
    - If the first authenticated request gets `401`, exchange refresh token and retry once.
    - Never retry more than once for the same request.

11. **Credential storage**
    - Store credentials only through the shared credentials layer.
    - Keep file permissions secure.
    - Do not invent alternate token storage locations.

## OpenAPI Workflow

12. **Generated client is the source of truth for backend types**
    - Refresh the spec with `scripts/refresh-openapi-spec.sh`
    - Regenerate the client with `scripts/generate-rust-client.sh`
    - Commit generated output intentionally
    - If generator output needs cleanup, encode it in scripts/config, not one-off manual edits

## CLI UX

13. **Output style**
    - User-facing output should be calm, direct, and human.
    - Use the spinner helpers for progress.
    - Use `colored` for status emphasis where helpful.
    - Do not use tracing logs as CLI output.

14. **Naming and messaging**
    - Prefer plain language:
      - good: `Downloading schema...`
      - good: `Login successful.`
      - bad: `Topologically resolved resource graph`

## Editing Discipline

15. **Re-read before edit**
    - Re-read files before editing them.
    - Re-read after editing to confirm the actual final state.

16. **Do not leave architectural drift behind**
    - If you touch a boundary, move the code toward the intended layer instead of adding more leakage.
    - If a new feature needs shared behavior, prefer extending `rusl-app` or `rusl-api-client` rather than bolting it onto the CLI.

17. **Keep MCP tool docs current**
    - When adding, removing, renaming, or materially changing an MCP tool, update `docs/mcp-tools.md` in the same change.
    - Keep tool descriptions terse, documentation-ready, and agent-focused.

# Agent Memory & Learnings

This file contains accumulated architectural context, logic rules, and design philosophies explicitly mapped for the `rusl` CLI project to keep automated assistants aligned with the project's vision.

### 1. Unified CLI UX & Output
- **No verbose tracing for user logs**: Do NOT use `tracing::{info, warn}` out to the terminal for state management or debugging lines. Users find this ugly and robotic. 
- **Use `indicatif` Spinners**: Use `crate::ui::spinner("Plain text message...")` to yield a single-line, dynamic progress bar for CLI steps so that the terminal history isn't spammed with step-by-step texts.
- **Use colored outputs**: Use the `colored` crate (`"Success:".green().bold()`) over standard `println!` for alerts.
- **Human-Readable Text**: Status messages must sound like a normal human CLI tool (e.g. "Downloading schema...", "Installed 3 schemas."). Avoid heavy, nerdy adjectives ("mathematically mapped", "topologically resolved", "physically linked").

### 2. Authentication Architecture (PKCE)
- **Flow**: The CLI performs an OAuth 2.0 PKCE flow by randomly binding to `127.0.0.1:0` natively, generating S256 code verifiers, and explicitly asking the user to log in via the main website.
- **Token Resiliency**: The main HTTP client (`RegistryClient`) physically traps `401 Unauthorized` responses. If a `refresh_token` exists locally, it eagerly rotates the `access_token` by hitting `POST /api/tokens/exchange` and automatically retries the identical request once.
- **Storage**: Sessions are saved securely in `~/.config/rusl/credentials.toml` via `0600` permission boundaries.

### 3. Network Configuration
- **Decoupled URLs**: `api_base_url` (API Backend) and `website_url` (Frontend Interface) are treated as two fundamentally different routing spaces. Never natively derive the API url systematically from the Website url.
- Local mapping is usually `localhost:4000` (API) and `localhost:3000` (Web). Production defaults are `https://api.rusl.app` and `https://rusl.app`.

### 4. Code Generation Rule
- Avoid heavy OpenAPI auto-generators (e.g., `progenitor`). Opt for manually maintained `reqwest` API calls equipped with explicit `serde` payload structs to keep compiler times lightning fast and the binary size ultra-lightweight.

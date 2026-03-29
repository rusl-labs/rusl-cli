# Rusl CLI Authentication Flow (OAuth2 PKCE)

To securely authenticate a distributed CLI application (like `rusl`), the Backend and CLI must implement the **OAuth 2.0 Authorization Code Flow with PKCE** (Proof Key for Code Exchange). 

Because `rusl` runs on public user machines, it is impossible to securely bundle a static "Client Secret" inside the binary. PKCE was designed specifically to prevent malicious local applications from intercepting the authentication loop natively.

---

## 1. The PKCE Ignition (CLI Action)
When the user types `rusl login`:
1. The CLI dynamically spins up a tiny, invisible HTTP server on an ephemeral local port (e.g., `http://127.0.0.1:14321/callback`).
2. The CLI generates a highly secure, one-time cryptographic string called the `code_verifier`. 
3. The CLI hashes that string using SHA-256 to create the `code_challenge`.
4. The CLI executes an OS command to open the user's default browser, passing the challenge in the URL natively:
```text
https://registry.rusl.dev/login/cli?callback_url=http://127.0.0.1:14321/callback&code_challenge=<HASH>&code_challenge_method=S256
```

---

## 2. The Browser Login (Web App Action)
When the browser loads `https://registry.rusl.dev/login/cli`:
1. The Frontend extracts the `callback_url`, `code_challenge`, and `code_challenge_method`. 
> [!CAUTION]
> **Open Redirect Validation:** The Backend/Frontend **MUST** rigidly assert that `callback_url` matches `http://127.0.0.1:*` or `http://localhost:*` before proceeding, strictly preventing Open Redirect hijacking.
2. The user authenticates natively in the browser (SSO, GitHub, Email).
3. The Backend securely stores the `<HASH>` tied to this specific user login session.
4. The Backend generates a short-lived (e.g., 60-second) **Authorization Code**, NOT the sensitive refresh token!
5. The Frontend explicitly redirects the browser back to the CLI:
```javascript
window.location.href = `http://127.0.0.1:14321/callback?code=${authorization_code}`;
```

---

## 3. The Cryptographic Exchange (CLI Action)
When the browser redirects to `http://127.0.0.1:14321/callback`:
1. The CLI's temporary HTTP server intrinsically catches the `GET` request and extracts the `code`.
2. The CLI immediately destroys the local HTTP server and prints "Login Successful" to the browser.
3. The CLI makes a secure **POST** request directly to the Backend API:
```http
POST /api/auth/token
Content-Type: application/json

{
  "code": "123xyz...",
  "code_verifier": "<THE_ORIGINAL_PLAINTEXT_SECRET_FROM_STEP_1>"
}
```

---

## 4. The Verification & Credential Storage (Backend Action)
When the Backend receives the `POST /api/auth/token`:
1. The Backend manually hashes the plaintext `code_verifier` provided by the CLI using `SHA-256`.
2. The Backend compares this hash to the `<HASH>` it saved earlier in Step 2.
3. **If they completely match:** The Backend has cryptographic proof that the process making the HTTP request is the exact same process that initiated the browser session, completely thwarting malware interception!
4. The Backend replies over the secure HTTPS channel with the permanent `refresh_token` and an ephemeral `access_token`.
5. The CLI securely natively persists these tokens to `~/.config/rusl/credentials.toml`.

Later, during `rusl install`, the CLI injects `Authorization: Bearer <access_token>` seamlessly into every `reqwest` HTTP header going out, unlocking all private schemas instinctively!

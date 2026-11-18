# Data access and auth

## The ladder

Choose a source per feature, not one for the whole app. Climb down only when the rung above can't do the job.

1. **Official public API.** It's documented, stable, allowed, and usually has a proper OAuth flow for third-party apps. Costs: the user may have to register a developer app, some data may be missing, and some APIs restrict new apps or require a paid tier. If it covers the user's core tasks, use it, even if it covers less than the website.
2. **The web app's internal API**, the JSON/GraphQL endpoints you saw in recon. It has full parity with the website, because it *is* what the website uses. Costs: it's undocumented, it can change any day (persisted-query hashes rotate on deploys), auth is often awkward to reproduce, and it may violate the service's terms. Use it for features the official API lacks, and tell the user plainly what they're opting into.
3. **HTML scraping** (`scraper` crate) is for server-rendered sites with no usable API. It's the most fragile option: keep selectors in one module so fixing them is a one-file job.

Never use a technique whose purpose is to get around a restriction: DRM, paywalls, rate limits, bot detection or account bans. If the only way to build a feature is one of those, it goes in the *not possible* group at the plan checkpoint.

## Auth patterns

Pick the simplest one that works, and confirm it with the user at the plan checkpoint.

**OAuth 2 with PKCE and a loopback redirect** is the standard for official APIs and native apps:
1. The user registers a developer app on the service and gives you the **client ID**. A native app with PKCE needs no client secret. If the service insists on a secret, it can't really be kept secret in a desktop app, so say so.
2. The redirect URI is `http://127.0.0.1:<port>/callback`. Pick a fixed port and tell the user to register exactly that URI.
3. In Rust: generate a verifier and challenge, open the authorize URL in the system browser (`tauri-plugin-opener`), and accept one connection on a `std::net::TcpListener` bound to `127.0.0.1:<port>`. Read the `code` from the request line, reply with a tiny "You can close this tab" page, then exchange the code for tokens.
4. Store the refresh token, and refresh access tokens on 401 or before they expire.

Put the login flow behind a route like `/login` with a `[log in]` action, and show login state in the status line.

**Session from the web app** applies when you're on the internal API. Options, from best to worst:
- **The service's own token endpoint.** Sometimes the web app gets a bearer token from an endpoint you can call too (recon pass 2 will show it).
- **A login window.** Open a second Tauri webview window on the real login page, and once it lands on a logged-in URL, read the cookies it holds. Check that your Tauri version exposes cookie access before you plan around this.
- **Paste-a-token.** A `/settings` page with a textarea where the user pastes a cookie or token copied from their browser. It's crude and it expires, but it's honest and quick. It's fine for a first version, as long as you say so.

**API keys:** a `/settings` page where the user pastes the key.

## Storing credentials

- In the app data directory (`AppState.data_dir`, which is the Tauri app data dir), as `auth.json`, written with `0600` permissions on Unix. Never put it in the project directory or anywhere it could be committed.
- If the user wants it, use the OS keyring via the `keyring` crate instead.
- Load credentials into `AppState` at startup, behind a `tokio::sync::Mutex` (it's held across `.await` while refreshing).

## Making requests

- Use the shared `reqwest::Client` in `AppState`. Set a truthful user agent (the template sets `<app>/<version>`). Don't impersonate the official client unless the internal API rejects everything else, and if you do, tell the user.
- Write one function per endpoint in `api.rs` (split into `api/<area>.rs` when it grows), returning typed structs. Deserialize only the fields you render, and use `#[serde(default)]` or `Option` generously, because API responses lie about optionality.
- Turn errors into readable messages that include the status, the endpoint and a short body excerpt. The router renders them as an error page with a retry link.
- **Cache** read-only responses in memory with a short TTL (30–120 s) keyed by URL. Back and forward then become instant, and you're gentler on rate limits. A `Mutex<HashMap<String, (Instant, serde_json::Value)>>` is enough. Invalidate the related keys after a mutation.
- **Rate limits:** respect `429` and `Retry-After`. Never fan out dozens of parallel requests to fill one page; if a page needs that, it's two pages.
- **Images:** request the smallest size the CDN offers that fits the style (see `ui-style.md`), and load them straight from the CDN with `<img loading="lazy">`. The CSP already allows `https:` images.

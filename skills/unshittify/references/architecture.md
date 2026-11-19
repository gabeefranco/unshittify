# Architecture of the template

## Request flow

```
click <a href="#/artist/4Z8W">
  → hashchange → app.js show()
  → invoke("render", { path: "/artist/4Z8W" })
  → lib.rs render → router::render → Req::parse → match ["artist", id]
  → pages::artist::show(id, state) → api::artist(...) → maud Markup
  ← HTML fragment → main.innerHTML
  → invoke("status") → #status
```

- **GET forms** (search, filters) don't touch Rust directly. `app.js` turns them into a path with a query string and navigates, so the results are bookmarkable in history and back works.
- **POST forms** (actions) call `invoke("act", {path, form, from})`. The router runs the action and returns the path to show next. On error, Rust sets a flash message and returns `from`, so the user sees the error on the page they were on.
- **External links** (`https:`, `mailto:`) open in the system browser.
- **Back/forward** are the webview's own history: every navigation is a hash change.

The window shows the static `ui/` folder, which is embedded in the binary. No dev server or node is involved.

## Files

| File | Role |
|---|---|
| `ui/index.html` | Header (app name, section links, back/forward), `#status`, `<main>` |
| `ui/app.js` | Router, form handling, external links, keys. Keep logic out of it. |
| `ui/style.css` | All styling. Variables at the top. |
| `src-tauri/src/lib.rs` | Tauri setup and the three commands: `render`, `act`, `status` |
| `src-tauri/src/router.rs` | `Req` (path, segments, query helpers) and the route tables for GET and POST |
| `src-tauri/src/pages/*.rs` | One module per screen type |
| `src-tauri/src/html.rs` | `link`, `action`, `pager`, `duration`, `standalone` |
| `src-tauri/src/api.rs` | Endpoint functions and response structs |
| `src-tauri/src/state.rs` | `AppState`: data dir, HTTP client, flash, and your credentials and caches |
| `SPEC.md` | The recon output and build status |
| `install.sh` | Linux installer: `sudo ./install.sh` builds the app and adds it to the user's app menu |
| `AGENTS.md` | Short orientation for whoever works on the app next |

## Adding a page

1. Write the API function in `api.rs`. Derive `Deserialize` structs from the sample in `docs/samples/`:

   ```rust
   #[derive(Deserialize)]
   pub struct Album { pub id: String, pub name: String, pub release_date: String, pub tracks: Page<Track> }

   pub async fn album(state: &AppState, id: &str) -> anyhow::Result<Album> {
       get_json(state, &format!("https://api.example.com/v1/albums/{id}")).await
   }
   ```
   Add auth headers as needed: make `get_json` take the bearer token from `AppState` once auth exists.

2. Write `pages/album.rs`:

   ```rust
   pub async fn show(id: &str, state: &AppState) -> anyhow::Result<Markup> {
       let album = api::album(state, id).await?;
       Ok(html! {
           h1 { (album.name) " — album" }
           p { (album.release_date) " · " (album.tracks.total) " tracks" }
           table {
               tr { th.num { "#" } th { "title" } th.num { "time" } th {} }
               @for (i, t) in album.tracks.items.iter().enumerate() {
                   tr {
                       td.num { (i + 1) }
                       td { (link(&format!("/track/{}", t.id), &t.name)) }
                       td.num { (duration(t.duration_ms)) }
                       td { (action("/play", "play", &[("uri", &t.uri)])) }
                   }
               }
           }
       })
   }
   ```

3. Register it with `pub mod album;` in `pages/mod.rs`, add `["album", id] => pages::album::show(id, state).await,` in `router::render`, and link to it from wherever albums are mentioned.

4. Check it: `cargo run -- --render /album/6dVIqQ8qmQ5GBnJ9shOYGE`.

## Adding an action

```rust
// router::act
["play"] => pages::player::play(form, state).await,

// pages/player.rs
pub async fn play(form: HashMap<String, String>, state: &AppState) -> anyhow::Result<String> {
    let uri = form.get("uri").context("missing uri")?;
    api::play(state, uri).await?;
    state.flash("Playing.");
    Ok("/queue".into()) // or return the page the user was on: add a hidden "back" field
}
```

To return the user to the page they acted from, include `("back", &req.path)` in the action's fields and return `form["back"]`.

## Checking your work without a window

`cargo run -- --render '<path>'` prints the page as a full HTML document with `style.css` inlined. Pipe it to a file and:
- read it, which is enough to verify content, links and escaping, or
- open it in your browser tool to see the layout. Links won't work there; it's a snapshot.

Routes that need credentials read them from `APP_DATA_DIR`. A debug run of the app prints its data dir on startup; export that path to reuse the app's stored login.

## Running and building

- `cd src-tauri && cargo run` opens the app (debug build).
- `cargo tauri dev` does the same with file watching, if the Tauri CLI is installed (`cargo install tauri-cli --version "^2" --locked`).
- `cargo tauri build` builds installers. Run `cargo tauri icon <png>` first for platform icons.
- `sudo ./install.sh` (Linux) installs the app for the user who ran sudo, without the Tauri CLI. Root installs the system libraries; the release build (`cargo build --release --features tauri/custom-protocol`) runs as the user. The binary goes to `~/.local/bin`, the icon to `~/.local/share/icons/<identifier>.png`, and the desktop entry to `~/.local/share/applications/<identifier>.desktop`. `--no-deps` skips the packages, and `--uninstall` removes the three files but keeps the app's data. The desktop entry uses `productName` as its name, `src-tauri/icons/icon.png` as its icon, and the `COMMENT` default at the top of the script as its description.

## Security model

The webview can call the Rust commands, so any script that runs in it can act as the user. Defenses:
1. **maud escapes everything**, so API content can't inject markup. Don't use `PreEscaped` on external data.
2. **The CSP** in `tauri.conf.json` blocks inline scripts, and scripts and styles from anywhere but the app. Scripts inserted with `innerHTML` never execute anyway.
3. If you must show third-party HTML (an email body, a rich post), put it in a sandboxed `<iframe sandbox srcdoc="…">`, never in `<main>`.

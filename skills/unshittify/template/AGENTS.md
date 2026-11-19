# Working on this app

This is an "unshittified" native client: Rust + Tauri 2, with pages rendered by Rust as plain, text-first HTML. Read `SPEC.md` for what the app covers and where its data comes from.

## Layout

```
install.sh             Linux installer (sudo): builds, installs to ~/.local, adds a menu entry
ui/index.html          header, status line, <main>. Edit header links here
ui/app.js              hash router (~100 lines). Rarely needs changes
ui/style.css           all styling. Variables at the top
src-tauri/src/
  router.rs            every route and action. Start here
  pages/<screen>.rs    one module per screen type, returns maud Markup
  api.rs               service API calls and typed response structs
  state.rs             shared state: HTTP client, credentials, caches, flash
  html.rs              helpers: link(), action(), pager(), duration()
docs/samples/          redacted sample API responses used to write structs
```

## Conventions

- **Add a page:** write `pages/<screen>.rs` → add a route arm in `router.rs` → link to it from an existing page.
- **Add an action:** add an arm in `router::act` that does the work, calls `state.flash("…")` and returns the path to show next. Render it with `html::action(path, "label", &[fields])`.
- Links use `html::link("/route", text)`. Never hand-write `href="#…"`.
- All HTML goes through `maud`, which escapes it. Never use `PreEscaped` on API data.
- The look is a plain document: text links, `[bracketed]` actions, no icons, no animations, images only when they're the content.
- No secrets in the repo. Credentials live in the app data dir.

## Run and check

```sh
cd src-tauri
cargo run                                  # open the app
cargo run -- --render '/notes?page=2'      # print one page as standalone HTML (no window)
cargo clippy && cargo build --release      # or: cargo tauri build, for installers
sudo ../install.sh                         # Linux: install into ~/.local with a menu entry
```

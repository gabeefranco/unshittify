---
name: unshittify
description: Recreate any web app (Spotify, Gmail, YouTube, Reddit, Jira, a bank dashboard, anything that runs in a browser) as a small native desktop client in Rust + Tauri, with a minimalist text-first UI that looks like a plain HTML document, in the style of die.net's Linux man pages (a header, text, links, a few images). Covers getting browser access to inspect the live app, mapping its screens and network APIs, choosing a data source, scaffolding from a bundled Tauri template, and building it page by page. Use this whenever the user wants to rebuild, replace, clone, de-bloat or "unshittify" a website or web app, or wants a native, lightweight, minimal, text-based or keyboard-friendly client for an existing web service, even if they don't mention Tauri or Rust.
compatibility: Any coding agent that can run shell commands. Browser automation tools (Chrome DevTools MCP, Playwright MCP, a built-in browser, etc.) are strongly recommended for the recon phase. Needs Rust and the Tauri 2 system prerequisites.
---

# Unshittify

You are rebuilding a bloated web app as a fast native client that reads like a document. The user keeps the information and the actions they care about. The ads, upsells, spinners, carousels, tracking and the 40 MB of JavaScript go away.

## What you are building (the defaults)

- **A Rust + Tauri 2 desktop app.** Rust does everything that matters: API calls, auth, caching, and rendering each page to HTML with `maud`.
- **A tiny frontend.** One static `index.html`, one `style.css` and about 100 lines of vanilla JS (`app.js`). The JS is a hash router: `#/artist/4Z8W` → `invoke("render", {path})` → Rust returns an HTML fragment → it goes into `<main>`. There's no framework, no bundler and no node.
- **A look like a man page on the web.** A header line, an `h1`, sections, blue links, text instead of widgets, and images only when they are the content. `references/ui-style.md` is the authority on this.

These are defaults, not laws. If the user asks for a different look or stack detail, do that: their instructions beat this skill, and this skill beats your own taste. When you deviate, say so in the project's README so the next person knows.

The bundled `template/` is a working app built on this architecture. Start from it and don't scaffold from scratch. A consistent shape across every unshittified app is what makes them easy to modify later.

## Workflow

Two checkpoints need the user's approval: the **plan** (before you write app code) and any **hard case** (a feature that can't be rebuilt cleanly). Between checkpoints, keep going on your own.

### 1. Scope

Ask the user briefly:
- Which app, and its URL.
- What they actually use it for. Ask for the top 3–5 tasks. These set the build order, and they matter more than feature parity.
- Anything they specifically want gone, or kept.
- **A colorscheme.** Accept any answer:
  - a known scheme ("solarized dark", "gruvbox", "nord")
  - "match the original app"
  - something custom (hex codes, "warm sepia", a screenshot, a terminal theme file)
  - "no preference", which means the default man-page look

  The colors change; the man-page layout stays. `references/ui-style.md` → *Colors* says how to turn any answer into the palette.
- Where to create the project (default: `./<app>-unshittified`).

Don't ask what you can find out by looking at the app yourself.

### 2. Get browser access

You need to see the live app, and ideally its network traffic, while the user is logged in. Read `references/browser-access.md`. In short: check which browser tools you have. If you have none, stop and ask the user to give you some, and offer concrete options. Have the user log in themselves, and never ask for their password.

### 3. Recon: map the app

Read `references/recon.md`. Walk the app and list each *kind* of screen, what information it shows and what you can do there. Capture the network requests behind each screen. Save trimmed, redacted sample responses. Write it all into `SPEC.md` (the template has a skeleton).

The goal is **parity of information, not of pixels**. You're recording what a screen tells the user and lets them do, not how it looks.

### 4. Choose data sources and auth

Read `references/data-access.md`. For each feature, work down this ladder: **official public API → the web app's own internal API → HTML scraping**. Stop at the first rung that works. Work out auth too: what the user must provide (for example a developer app client ID) and where tokens will live.

Check `references/hard-cases.md` for anything that won't translate cleanly: DRM media, realtime, rich editors, canvas apps, infinite feeds.

### 5. Plan checkpoint

Present a short plan and wait for approval:
- **Screens**, in three groups: *core* (the user's top tasks), *later*, and *not possible / dropped*, each with a one-line reason.
- **The data source for each core screen.** Flag every undocumented or internal API, because it can break without warning and may be against the service's terms.
- **What the user needs to do.** For example: register a developer app, paste a client ID, or install system packages.
- **Hard cases**, with the options you see.
- **Dropped junk**: ads, upsells, recommendation rails, and so on. That way the user can rescue anything they actually value.

### 6. Scaffold

1. Copy `template/` (next to this file) to the project directory.
2. Rename the app. Change `productName`, `identifier` and the window title in `src-tauri/tauri.conf.json`, the package `name` in `src-tauri/Cargo.toml` (keep the `[lib] name`, or update `main.rs` to match), the header and `<title>` in `ui/index.html`, and the `APP` constant in `ui/app.js`.
3. Apply the colorscheme to the `:root` variables in `ui/style.css`, and check it with `python3 scripts/contrast.py <project>/ui/style.css` (in this skill's folder). Record the choice under *Look* in `SPEC.md`.
4. Check prerequisites: `cargo --version`, plus the Tauri system dependencies for this OS (<https://tauri.app/start/prerequisites/>; on Linux that's webkit2gtk-4.1 and friends). If something is missing, tell the user the exact install command. Don't run `sudo` yourself.
5. Build and run the template once, before changing anything except the colors, (`cd src-tauri && cargo run`, or `cargo tauri dev` if the Tauri CLI is installed). That way, any later failure is your code and not the toolchain.

`references/architecture.md` explains the template: request flow, how to add a page, how to add an action, and how to check pages without a window.

### 7. Build page by page

Build vertical slices in this order: auth (if needed), then the user's #1 task, then the next one. For each screen:

1. **API.** Add a function in `api.rs` (or `api/<area>.rs`) with serde structs written from the saved samples. Only include the fields you render.
2. **Page.** Add `pages/<screen>.rs` returning `Markup`, following `references/ui-style.md`.
3. **Route.** Add a match arm in `router.rs`, and link to the page from wherever the user would expect to find it.
4. **Check.** Run `cargo run -- --render '/the/route'` to print the page as a standalone HTML document. Read it, or open it in your browser to look at it. Then compare it with the same screen in the web app. Is any information the user would miss gone? Is there junk you carried over?

Once the core screens work, remove the example `notes` pages from the template (`pages/example.rs`, its routes and the `notes` field in `state.rs`).

### 8. Verify and hand off

- Run `cargo clippy` and `cargo build` with no warnings you introduced, then render every route with `--render`.
- You probably can't see the native window, so ask the user to run the app and click through the core tasks.
- Update the project `README.md` (what works, how to run it, auth setup, known gaps and dropped features) and the status column in `SPEC.md`.

## Ground rules

- **The user's real account is not a test environment.** During recon, don't post, send, delete, buy, follow or change settings. If you need to see how an action works, ask the user to do it while you watch the network, or find it in the docs.
- **Secrets stay out of files.** Tokens, cookies and API keys never go into source code, `SPEC.md`, samples or logs. Redact them in samples (`"REDACTED"`). At runtime, tokens live in the app data directory or the OS keyring (see `references/data-access.md`).
- **Respect the service.** It's for personal use. Stay within rate limits, and don't circumvent DRM, paywalls or access controls. When you use an internal API, tell the user plainly that it's undocumented and could break or violate the terms.
- **Escaping is security.** API content goes on screen, and the webview can call Rust commands. Always render through `maud` (it escapes by default), and never build HTML with `format!` or insert `PreEscaped` around API data.
- **Keep the template's shape.** That means one module per screen, the router as the single map of routes, and styling only in `style.css`. The next agent or human should be able to find anything in seconds.

## Files in this skill

| Path | What it is | When to read it |
|---|---|---|
| `references/browser-access.md` | Getting and using browser tools, across agents | Step 2 |
| `references/recon.md` | Mapping screens and APIs, writing SPEC.md | Step 3 |
| `references/data-access.md` | API ladder, auth, token storage, caching | Step 4 |
| `references/hard-cases.md` | Media/DRM, realtime, editors, uploads, etc. | Step 4, or when stuck |
| `references/examples.md` | Worked notes for Spotify and a few others | When rebuilding one of those, or for a sense of scale |
| `references/ui-style.md` | The house style: widget → text translations | Before writing any page |
| `references/architecture.md` | How the template works, how to extend it | Step 6 onward |
| `scripts/contrast.py` | Checks text/background contrast of the palette in a `style.css` | After applying or changing colors |
| `template/` | The starter app you copy | Step 6 |

# unshittify

A method for AI coding agents to rebuild bloated web apps as small native desktop clients. The clients are built in Rust + Tauri, and they look like a plain HTML document: a header, text, links and the occasional image. Think die.net's Linux man pages, not a SaaS dashboard.

```
app — library  search  about                          « back  forward »
now playing: Airbag — Radiohead (1:23 / 4:44) [pause] [next]
──────────────────────────────────────────────────────────────────────
Radiohead — artist

English rock band. 12.3M followers.  [follow]

ALBUMS
  1. OK Computer (1997) — 12 tracks
  2. Kid A (2000) — 10 tracks
  « prev  page 1  next »
```

It's packaged as an [Agent Skill](https://agentskills.io): a folder with a `SKILL.md`, reference docs and a starter template. Any agent that can read files and run commands can use it.

## How it works

1. **Scope.** The agent asks which app you want rebuilt, what you actually use it for, and what colorscheme you want (a named one like Solarized Dark, "match the original app", or anything custom).
2. **Browser access.** The agent uses its browser tools to inspect the live app while you're logged in. If it has none, it asks you to connect some (Chrome DevTools MCP, Playwright MCP, your agent's own browser) or to export a HAR file.
3. **Recon.** It maps every screen type, the information on it, and the network requests behind it, and writes `SPEC.md`.
4. **Data plan.** For each feature it chooses the official API first, then the app's internal API, then scraping. It works out auth.
5. **Plan checkpoint.** You approve the screens, data sources, setup steps and what gets dropped.
6. **Build.** It copies the template and builds page by page, checking each against the real app.

## Use it

**Claude Code:** it works inside this repo already (`.claude/skills/unshittify` is a symlink). For use anywhere, link it into your user skills:

```sh
ln -s "$PWD/skills/unshittify" ~/.claude/skills/unshittify
```

**Agents that read `.agents/skills` or support the `skills` CLI** (Codex, Gemini CLI, Cursor, OpenCode, ...):

```sh
ln -s "$PWD/skills/unshittify" ~/.agents/skills/unshittify
# or, once this repo is on GitHub:  npx skills add <you>/unshittify
```

**Any other agent:** tell it to "read `/path/to/unshittify/skills/unshittify/SKILL.md` and follow it to rebuild <app>".

Then ask for something like:

> unshittify spotify for me. I mostly use it to search, browse albums and manage my playlists.

## Customize it

The method is meant to be forked and tuned. Each concern lives in one place:

| To change… | Edit |
|---|---|
| The default colors, or how the agent applies a user's colorscheme | `references/ui-style.md` → *Colors* + the `:root` variables in `template/ui/style.css` |
| The look (fonts, layout rules, widget → text mapping) | `skills/unshittify/references/ui-style.md` + `template/ui/style.css` |
| The workflow, checkpoints and ground rules | `skills/unshittify/SKILL.md` |
| Data source policy and auth approach | `references/data-access.md` |
| How the agent gets and uses a browser | `references/browser-access.md` |
| Recon depth and the SPEC format | `references/recon.md` + `template/SPEC.md` |
| The app architecture | `template/` + `references/architecture.md` |
| Policy for media, realtime, editors, etc. | `references/hard-cases.md` |

## Layout

```
skills/unshittify/
  SKILL.md                  the workflow (entry point)
  references/               detail, loaded only when needed
  template/                 starter Tauri app copied into each new project
    ui/                     index.html, app.js (hash router), style.css
    src-tauri/              Rust: router, pages, api, state, html helpers
    SPEC.md, AGENTS.md      skeletons for the generated project
AGENTS.md / CLAUDE.md       instructions for agents working on this repo
```

## Requirements for generated apps

Rust, plus the [Tauri 2 prerequisites](https://tauri.app/start/prerequisites/) for your OS. On Arch Linux: `sudo pacman -S --needed webkit2gtk-4.1 base-devel curl wget file openssl appmenu-gtk-module libappindicator-gtk3 librsvg xdotool`.

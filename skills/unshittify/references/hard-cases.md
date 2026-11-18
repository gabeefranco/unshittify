# Hard cases

Some features don't fit a text-first native client cleanly. For each one: name it at the plan checkpoint, offer the options below, and let the user choose. A smaller app that works beats a full one that's half broken.

## DRM-protected media (Spotify, Netflix, Apple Music, most video streaming)

The webview can't play DRM content. On Linux, WebKitGTK has no Widevine, and the official web SDKs depend on EME. So:
- **Remote control (usually the right answer).** Many services expose official APIs that control playback on the user's *other* devices: the official app, a speaker, a phone. The native client becomes a remote with a now-playing status line. Spotify's Connect/player endpoints work this way, and they need Premium.
- **A third-party protocol implementation** (e.g. librespot for Spotify) can play audio locally from Rust. It's an unofficial reimplementation that may violate the service's terms, so it's only an option if the user explicitly chooses it after you've explained that.
- **Drop playback** and keep browsing and library management. That's still useful.

## Non-DRM media (podcasts, self-hosted media, SoundCloud previews, YouTube via official embeds)

Put one `<audio id="player">` (or video) element in `index.html` **outside `<main>`**, so playback survives navigation. Add a small handler in `app.js` for links like `<a href="#" data-play="https://…">[play]</a>` that sets `player.src` and calls `play()`. Keep the now-playing text in the Rust status line, or update it from the `<audio>` events. Add the media host to `media-src` in the CSP if it isn't covered by `https:`.

## Realtime (chat, notifications, live dashboards)

Start with **polling the current page**: a `[refresh]` link and, if the user wants it, a periodic re-render of chat-like pages (`setInterval(show, 10000)` scoped to routes that opt in). Bring in websockets (`tokio-tungstenite` in Rust, pushing an event to the window with `app.emit`) only when polling is clearly not good enough.

## Infinite feeds and timelines

Paginate. Use `?page=` if the API takes offsets, or carry the API's cursor in the query string (`?after=abc123`) if it's cursor-based. "next »" is a link to the same route with the next cursor. Give up "page N of M" when the API can't tell you M.

## Rich editors (Google Docs, Notion, Confluence)

Edit as plain text or Markdown in a `textarea`, and save with a POST action. Converting rich formats both ways is lossy, so tell the user up front and consider making editing read-only plus "open in browser" for complex docs. Collaborative real-time editing is out of scope.

## Canvas and spatial apps (Figma, Miro, maps, spreadsheets as grids)

These are poor fits. Usually the useful part is the *data around* the canvas: the file list, comments, versions, the rows of a sheet as an HTML table, or a place's details and opening hours. Rebuild that, and link out to the browser for the canvas itself.

## Uploads

Use `<input type="file">` in a POST form. `app.js` currently sends only string fields, so extend it to read files with `FileReader` and send bytes (base64) to `act`, or use `tauri-plugin-dialog` to pick a path that Rust reads directly. The second is simpler for large files.

## Notifications and background sync

Off by default: unshittified apps don't nag. If the user asks for them, use `tauri-plugin-notification` and let them choose exactly what triggers one.

## Payments, purchases, account security settings

Leave these to the official website. Link out with "manage in browser" (external links open in the system browser).

## Anti-bot protection, frequent session invalidation, aggressive rate limits

If the internal API fights back (CAPTCHAs, signed requests, fingerprinting), stop. Report it to the user and fall back to the official API or a smaller feature set. Defeating those protections is out of bounds.

# Worked examples

These sketches give you a sense of scale and of the decisions involved. APIs change, so verify every claim here against current docs during recon; don't trust this file over what you observe.

## Spotify

**Typical top tasks:** search, browse an artist or album, manage playlists and liked songs, control playback.

**Data:** the official Web API (`api.spotify.com/v1`) covers search, artists, albums, playlists, the library, and player control on the user's devices. Some endpoints (recommendations, related artists, audio features and some editorial playlists) were restricted for new apps in late 2024, and developer-mode apps have had user and quota limits. Check the current policy and plan around what's actually available. The web player's internal GraphQL (`api-partner.spotify.com/pathfinder`) has more, but it's undocumented and uses rotating persisted-query hashes, so it's a rung-2 fallback only.

**Auth:** Authorization Code with PKCE. The user creates an app at developer.spotify.com, registers the redirect `http://127.0.0.1:<port>/callback`, and gives you the client ID. Scopes depend on the features, e.g. `user-library-read user-library-modify playlist-read-private playlist-modify-private user-read-playback-state user-modify-playback-state`.

**Playback:** DRM, so the client is a **remote** (see hard-cases.md). The player endpoints need Premium and an active device, so show "no active device — open Spotify somewhere" in the status line when there isn't one.

**Routes:**

| Route | Shows |
|---|---|
| `/` | Recently played, your playlists (links) |
| `/search?q=&type=` | Results grouped by type, with a `type` filter as links |
| `/artist/{id}` | Facts, albums (paged), top tracks |
| `/album/{id}` | Facts, track table with `[play]` |
| `/playlist/{id}` | Description, tracks (paged), `[play]` |
| `/library` | Liked songs (paged), saved albums, followed artists |
| `/queue` | Current and upcoming tracks |
| `/devices` | Devices with `[use this]` |
| `/login` | Login state and `[log in]` / `[log out]` |

**Status line:** `now playing: <track> — <artist> (1:23 / 4:44) [pause] [next] · on <device>`. Re-render it after every action and on a slow poll (about 10 s) while something is playing.

**Dropped:** the home feed's promotional shelves, Canvas videos, podcasts upsell, "made for you" rails (unless the user wants them), and lyrics (no official API).

## Hacker News

The official API (`hacker-news.firebaseio.com/v0`) is read-only and needs no auth. Item lists are just IDs, so `/top?page=2` fetches 30 items concurrently, which is fine with the cache. Voting and commenting need the website's form posts with a session cookie (rung 3), so offer them as "later". It's a good first app to try the method on.

## Gmail

The official Gmail API needs a Google Cloud project with OAuth credentials, and Gmail scopes are "restricted". For personal use, an app in testing mode with the user as a test user works. Fetch thread lists with `format=metadata` and render bodies as text/plain where available. HTML mail is the trap: either show the text part, or sanitize hard and render it inside a sandboxed `iframe` with `srcdoc`. Never insert mail HTML into `<main>`, because it would run in the window that can call Rust commands.

## Reddit

The official API (`oauth.reddit.com`) needs an app registration and OAuth, and it has strict rate limits and API terms, so read them with the user. `/r/{sub}?after=` maps directly to cursor pagination. Comments are a tree, so render them as nested `ul`s with indentation and a depth limit plus a "continue thread" link.

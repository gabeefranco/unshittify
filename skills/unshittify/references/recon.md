# Recon: mapping the web app

The output of recon is `SPEC.md` in the new project, plus sample responses in `docs/samples/`. Everything after this point is built from those files, so a good spec makes the build mostly mechanical.

Think in **screen types**, not screens. Spotify has millions of artist pages but one *artist* screen type. You need one example of each.

## Pass 1: the screen map

Start at the landing page after login. Follow every primary navigation item, then open one instance of each kind of thing you see (one artist, one album, one playlist, one search result page...). For each screen type record:

- **Name**, and the **web URL pattern** (`/artist/{id}`). If the URL doesn't change, note how you reached the screen.
- **Information shown:** the actual fields (title, year, track list with duration, follower count...). This is the checklist for the native page.
- **Actions:** what you can do here (play, save, reply, filter, sort, paginate).
- **Reached from:** which screens link to it. These become the `<a>` links.
- **Junk:** what's here that isn't for the user, like ads, upsells, "you might like" rails, cookie banners and engagement bait. You're recording it in order to drop it, and the user gets a chance to rescue anything at the plan checkpoint.

If the user asked for the colorscheme to **match the original app**, record its real colors now (background, text, accent, muted text), following `ui-style.md` → *Colors*.

Also note any **global chrome** that persists across screens, such as a player bar, a notification count or an account menu. These usually become the header or the status line (`#status`) in the template.

Don't try to cover everything in the app. Go deep on the user's top tasks and skim the rest. Settings pages, admin areas and rarely used flows can be listed as "later" with one line each.

## Pass 2: the data behind each screen

For each screen type, turn on network capture, reload the screen, and find the requests that carry its data. For each data request record:

- **Method and URL pattern**, with the important query params.
- **Auth:** which header or cookie carries it (`Authorization: Bearer …`, a session cookie, a CSRF header). Find out **where the token comes from**: look for a token or session request early in the page load. You'll need to reproduce that.
- **Pagination:** offset/limit, page numbers, a cursor or a "next" URL.
- **GraphQL:** note the operation name and variables. Many apps use *persisted queries*, which send a hash (`sha256Hash`) instead of the query text. Record the hash, and note that it changes when the app is redeployed.
- **Image URLs:** the CDN pattern and the available sizes, so you can request small thumbnails.

Save **one trimmed sample per endpoint** to `docs/samples/<endpoint-name>.json`:
- Cut arrays to 2–3 items, and drop huge irrelevant subtrees.
- Replace tokens, emails, real names and anything personal with `"REDACTED"`.
- These samples are what you write serde structs against, so keep the field names and nesting exact.

**Mutations** (save, like, send, delete): don't perform them in the user's account to see what happens. Prefer the official API docs. Otherwise ask the user to perform the action while you capture, or ask permission for a harmless, reversible one (like then unlike). If neither works, reading the app's JS bundle for the endpoint name is a last resort.

## Pass 3: the official API

Search for the service's public API docs (`<service> developer API`). For each screen type, check whether the official API covers its information and actions. Note what it requires: OAuth app registration, scopes, paid tier, rate limits, and whether new apps are restricted. Read `data-access.md` for how to weigh this against the internal API.

## Writing SPEC.md

Fill in the skeleton in the project's `SPEC.md`. Keep it scannable, because the user will review it at the plan checkpoint and later agents will use it as their map. The essential parts are:

1. **Screens table:** native route | web URL | data source | priority (core/later/dropped) | status. Native routes are short and readable (`/artist/{id}`, `/search?q=`), so reuse the web app's IDs.
2. **Endpoints:** one entry per endpoint, with method, URL, auth, pagination and a link to the sample file.
3. **Auth:** how the native app will get and refresh credentials, and what the user must provide.
4. **Global chrome:** what goes in the header and the status line.
5. **Dropped:** the junk list, plus features you won't rebuild and why.
6. **Open questions** for the user.

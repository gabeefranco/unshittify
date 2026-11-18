# Browser access

Recon needs a real browser, logged into the user's account, that you can drive. The most valuable thing is the **network traffic**: it tells you where each screen's data comes from. Without it, you're guessing from the rendered page.

## 1. Check what you already have

Look at your available tools before asking for anything. Any of these will do, in rough order of usefulness:

| Tool family | Typical tool names | Network capture? |
|---|---|---|
| Chrome DevTools MCP | `navigate_page`, `take_snapshot`, `list_network_requests`, `get_network_request` | Yes, with bodies. Best option. |
| Playwright MCP | `browser_navigate`, `browser_snapshot`, `browser_network_requests` | Yes (URLs and status; bodies vary by version) |
| A browser extension that drives the user's own browser (e.g. Claude in Chrome) | `navigate`, `read_page`, `read_network_requests`, ... | Usually yes |
| A built-in or in-app agent browser | varies | Sometimes |
| Computer use / screen control | `screenshot`, `click`, ... | No. You can only see pixels, so ask for a HAR as well (below). |
| A browser CLI you can run (`playwright`, `agent-browser`, ...) | shell | Yes, if you script it |

If your harness lists tools lazily (deferred tools, tool search), search for "browser", "chrome", "playwright" and "devtools" first.

## 2. If you have nothing, ask

Stop and ask the user. Don't try to work around it by fetching pages with `curl`: most web apps are client-rendered shells that need a logged-in session. Offer concrete options and let them pick:

> To map the app I need to drive a browser where you're logged in. Pick one:
>
> 1. **Chrome DevTools MCP** (recommended, because it shows network requests with bodies). Add it to your agent's MCP config: `npx -y chrome-devtools-mcp@latest`. It can launch its own Chrome, or attach to one you started with remote debugging. Check its README for the current flags.
> 2. **Playwright MCP**: `npx -y @playwright/mcp@latest`. Its README also describes attaching to your existing browser.
> 3. **Your agent's own browser integration**, if it has one (an extension or a built-in browser). Enable it and tell me when it's on.
> 4. **No automation:** open the app, open DevTools → Network, click through the screens you care about, then right-click → *Save all as HAR* and drop the file in the project folder. Add a few screenshots too.
>
> After it's connected, log in yourself in that browser window. I don't need, and won't ask for, your password.

Restarting the agent is often needed after adding an MCP server. Say so, and say what to tell you when they're back ("browser is connected").

## 3. Using it well

- **Work in your own tab.** Don't close or navigate the user's existing tabs.
- **Read the page as text first.** Prefer accessibility snapshots and page text over screenshots. They're cheaper, and they show content hierarchy, link targets and labels. Use screenshots for layout questions, such as what's in the sidebar or what's in a modal.
- **For network capture, reload the screen after capture is on.** Filter to fetch/XHR, then ignore analytics, ads, telemetry, fonts, CSS and images. The data requests usually carry JSON and have names like `/api/`, `/v1/`, `graphql`, `pathfinder` or `batch`.
- **Read a request's body only when you need its shape.** Large responses eat context. Fetch one, trim it, save it as a sample (see `recon.md`), and move on.
- **Don't trigger side effects.** Clicking "like", "send", "delete", "follow" or "buy" acts on the user's real account. If you need to see a mutation, ask the user to do it while you watch the network, or ask permission for a harmless, reversible one.
- **Logins, 2FA and CAPTCHAs are the user's job.** If you hit one, ask them to complete it in the browser window, then continue.
- **Never copy cookies or tokens out of the browser into files.** You need to know *how* auth works, such as which header and where the token comes from, not the token itself.

## 4. Working from a HAR file

If the user gave you a HAR (option 4), it's JSON: `log.entries[]` with `request.method`, `request.url` and `response.content.text`. Don't load it whole, because HARs are often tens of megabytes. Filter it with a script, for example:

```sh
jq -r '.log.entries[] | select(.response.content.mimeType | test("json")) | "\(.request.method) \(.response.status) \(.request.url | split("?")[0])"' app.har | sort | uniq -c | sort -rn | head -50
```

Then pull out individual response bodies by URL as needed. HARs contain the user's cookies and tokens: never commit the file, and suggest deleting it once recon is done.

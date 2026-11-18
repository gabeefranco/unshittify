# House style: a man page on the web

> **This file is the main customization point.** Forks that want a different look should edit this file and `template/ui/style.css`, and leave the workflow alone.

The reference is die.net's Linux man pages: a plain header, a title, sections of text, blue links, a few images, and nothing that moves. Every screen of the rebuilt app should feel like a document you could print. Two reasons: documents are fast to scan, and they're impossible to turn into a dark pattern.

## Principles

1. **Each page is a document about one thing.** An artist, an inbox, a search result, a ticket. Its `h1` names that thing.
2. **Text beats widgets.** If a control can be a word, make it a word: a link to navigate, a `[bracketed]` text button to act. Use native form controls (`input`, `select`, `textarea`) only where text input or choice is the point.
3. **Every name is a link.** An artist name, an album title, a user, a tag: if a page exists for it, link to it. Links are the whole navigation system.
4. **State is written out.** "now playing: Airbag — Radiohead (1:23 / 4:44)", not a progress bar. "3 unread", not a red dot.
5. **Images only when they're the content or identify it.** Album covers, product photos, photos in a photo app: yes, small, `loading="lazy"`. Decorative banners, avatars next to every row, background art and logos: no.
6. **Nothing moves.** No animations, transitions, skeleton loaders, carousels, autoplay or hover-only UI. Loading shows as the word "loading…" in the status line.
7. **One column, default-ish typography.** System fonts, about 50em wide. Monospace only for code, IDs and durations in tables. Colors come from the user's colorscheme. Without one, use browser-blue links and purple visited links (see *Colors*).
8. **The keyboard works.** Everything is a link, button or input, so Tab reaches it all. `/` focuses the page's search box, and Alt+←/→ go back and forward (the template handles both).

## Page anatomy

```
app — library  search  about                          « back  forward »
now playing: Airbag — Radiohead (1:23 / 4:44) [pause] [next]
──────────────────────────────────────────────────────────────────────
Radiohead — artist

English rock band. 12.3M followers. Genres: art rock, alternative rock.
[follow]

ALBUMS
  1. OK Computer (1997) — 12 tracks
  2. Kid A (2000) — 10 tracks
  3. In Rainbows (2007) — 10 tracks
  « prev  page 1  next »

TOP TRACKS
  #  title                album          time
  1  Creep               Pablo Honey    3:58   [play]
  2  No Surprises        OK Computer    3:49   [play]

SEE ALSO
  related artists · singles and EPs · appears on
```

- **Header** (`index.html`, the same on every page): the app name linking home, top-level sections as links, back and forward as text links.
- **Status line** (`#status`, filled by the Rust `status` command): global state that persists across pages, such as now playing, logged in as, unread count or sync state. It's empty when there's nothing to say.
- **`h1`:** "<name> — <kind>" (the kind is optional when obvious). The template uses it for the window title too.
- **Intro paragraph:** the key facts in a sentence or two, followed by the main actions.
- **`h2` sections**, styled small-caps like man page headings, in order of what the user came for.
- **SEE ALSO**: secondary destinations, as a line of links.

## Widget → text translations

| Web app has | Write instead |
|---|---|
| Icon button (heart, play, kebab ⋮) | `[like]` `[play]`, a text button via `html::action()`. It's a `<form>`, so put it in `div.actions`, `li` or `td`, never in `p` |
| Toggle (liked / not liked) | The current state in words + the opposite action: `liked [unlike]` |
| Tabs | A line of links: `overview · albums · related`, with the current one in bold, not a link |
| Card grid / carousel / shelf | An `ol`/`ul` of links, or a `table` when items have 2+ comparable fields |
| Infinite scroll | `« prev  page 2  next »` links carrying `?page=` (or a cursor) |
| Modal / dialog / drawer | Its own page, with a link back |
| Dropdown menu of actions | The actions written out inline, or on the item's own page |
| Hover card / tooltip | Nothing. Put the information on the detail page. |
| Toast / snackbar | A flash message at the top of the next page (`state.flash()`) |
| Progress / seek bar | Text: `1:23 / 4:44`, plus `[-10s]` `[+10s]` if seeking matters |
| Volume slider | `vol 60% [-] [+]` |
| Badge / dot / counter | Words: "3 unread" |
| Avatar | Omitted; the name is enough. Show one only in apps where faces matter (dating, contacts). |
| Rich text composer | `textarea`, plain text or Markdown |
| Filters sidebar | A GET form at the top: `select`s + `[filter]`, with the state kept in the query string |
| Empty state illustration | One sentence: "No playlists yet." |
| Skeleton / spinner | Nothing. The status line says "loading…". |
| Confirm dialog for destructive actions | A confirm page: "Delete playlist X? [delete] · cancel" |

## Colors

The user picks a colorscheme in step 1, and any answer is valid. A colorscheme changes **only colors**. Fonts, layout, text buttons and the no-icons rule stay the same, because the man-page structure is the point and the palette is personal taste.

All color lives in seven variables in the `:root` block at the top of `ui/style.css`. Never hard-code a color anywhere else: that's what makes a scheme swappable in one place.

| Variable | Role | Where it usually comes from in a scheme |
|---|---|---|
| `--bg` | page background | the background color |
| `--fg` | body text | the foreground / default text color |
| `--muted` | secondary text, the status line | the comment color |
| `--rule` | borders, header underline, table rules | a selection or line-highlight tone, between bg and muted |
| `--link` | links and `[actions]` | the blue, or the scheme's main accent |
| `--visited` | visited links | the magenta/purple, or a second accent |
| `--flash` | flash-message background | a subtle tint, near `--bg` (a yellow-ish one if the scheme has it) |

### Turning the answer into a palette

- **Default / no preference:** keep the template's palette. It's a die.net-style light scheme, plus a dark variant that follows the system setting.
- **A known scheme** (Solarized, Gruvbox, Nord, Dracula, Catppuccin, Tokyo Night, One Dark, Monokai, a Base16 scheme, ...): use the scheme's **published** hex values. If you can browse or search, look up the official palette. Remembered values drift, and the user will notice a wrong shade of their favorite theme. Map the colors by role, using the table above.
- **"Match the original app":** during recon (step 3), read the real colors from the live app rather than guessing from a screenshot. Using the browser's JS evaluation, take `getComputedStyle(...)` of `body` (background, color), a normal link or primary button (accent), and secondary text (muted). Look at the main signed-in screens, not the marketing page. If you can only see screenshots, sample the pixels. Take the colors only, never the app's fonts or layout.
- **Custom:** this covers hex codes, a description ("warm sepia", "green on black like an old terminal", "my company's brand colors"), an image, or a terminal or editor theme file (Alacritty, kitty, iTerm, Xresources, VS Code, Vim). Extract or design the palette. If the input leaves some roles open, derive them from what was given: muted is fg mixed toward bg, rule sits between bg and muted, and flash is a faint tint of the accent. Mention which ones you derived.

### Light, dark, or both

The template has a light `:root` block plus a `@media (prefers-color-scheme: dark)` override.
- **One fixed scheme** (e.g. "solarized dark"): put it in `:root` and delete the media block. Set `html { color-scheme: dark; }` (or `light`), so native inputs and scrollbars match.
- **A family with variants** (Solarized light/dark, Catppuccin Latte/Mocha, Gruvbox light/dark): put the light variant in `:root` and the dark one in the media block. That way the app follows the system setting, which is the default unless the user named a single variant.

### Readability

Run `python3 scripts/contrast.py ui/style.css` after setting colors. It checks body text, links and visited links against the background (WCAG AA, 4.5:1), and muted text at 3:1. Honor the user's scheme, but readable text matters more than exact fidelity. If a pair fails, swap in the nearest color *from the same palette* that passes. For example, in Solarized Dark the blue (#268bd2) on base03 is only 4.1:1, so links can use the cyan (#2aa198, 4.75:1) instead. Tell the user what you changed and why. If they still want the original, they get the original.

Record the result in `SPEC.md` under *Look*: the scheme name or source, and any substitutions.

## Content rules

- Dates: absolute and short (`2025-03-14`, or `14 Mar` within the current year). "3 hours ago" is fine for feeds.
- Durations: `m:ss` (`html::duration`).
- Numbers: plain, with an abbreviation when large (`12.3M`).
- Truncate long descriptions on list pages, and show them in full on the detail page.
- Never show raw IDs to the user unless they are something people actually copy, like a ticket number.
- No emoji or icon fonts. Typographic characters are fine: — · « » … ×.

## Checklist for each page

- [ ] The `h1` names the thing.
- [ ] Every entity mentioned is a link, if a page exists for it.
- [ ] All information the user needs from the web version is present, and nothing from the junk list.
- [ ] Actions are text buttons; destructive ones go through a confirm page.
- [ ] Lists longer than about 50 items are paginated.
- [ ] Images are small, lazy-loaded, and only where they carry content.
- [ ] It reads well with CSS disabled. That's what semantic HTML buys you.

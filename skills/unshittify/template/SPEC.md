# SPEC: <app> unshittified

Source: <https://…> · Recon date: <YYYY-MM-DD> · Data: <official API / internal API / mixed>

## Top tasks

What the user actually uses the app for, in priority order.

1. …

## Screens

| Native route | Web URL | Shows | Data source | Priority | Status |
|---|---|---|---|---|---|
| `/` | `/` | … | … | core | todo |

Priority: **core** (the top tasks), **later**, **dropped**. Status: todo / done / partial.

## Endpoints

### <name>
- `GET https://…/v1/thing/{id}?limit=&offset=`
- Auth: …
- Pagination: …
- Sample: `docs/samples/<name>.json`
- Official: yes/no (link to docs)

## Auth

How the app gets credentials, where they're stored, what the user must provide.

## Look

- Colorscheme: <default | name, e.g. "Solarized Dark" | "matches original app" | custom: description>
- Variants: <fixed light/dark | follows system>
- Substitutions for readability: <none | e.g. links use cyan instead of blue (4.1:1 failed)>

## Global chrome

- Header links: …
- Status line: …

## Dropped

| What | Why |
|---|---|
| Promotional shelves on home | Ads |

## Open questions

- …

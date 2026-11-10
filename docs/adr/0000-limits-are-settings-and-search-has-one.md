# 0000. Request limits are settings, and the search page has one

**Number:** next free. **Status:** Accepted, 2026-11-10. Gap audit 3, row 13 (KH-OPS-5, G33, RULES.md BL-SEC-5).

## Context

ADR "The API limits each key, and each address that sends wrong keys" made the API limits constants and left a `[limits]` table for later. BL-SEC-5 also names search, and `/search/?q=` reads the full-text index on every request, with no key in front of it. Only 4 of 138 operations in `openapi.json` said a 429 could come back, so SDK users could not see it.

## Decision

- **A `[limits]` table in `site.toml`, typed in `khalid-core` as `LimitSettings`:** `admin_per_minute` (600), `content_per_minute` (6000), `wrong_keys_per_minute` (10) and `search_per_minute` (120), each optional and each 1 to 100000. The defaults are the old constants, so no site changes behaviour. A change waits for a restart, like `[visitor_log]`, since the limiter is built at start.
- **One limiter type, `ApiLimits`, also counts searches.** Searches have their own bucket per network (an IPv4 address or an IPv6 /64), apart from wrong keys, and use the same address rule: the peer, or the last `X-Forwarded-For` entry when the peer is a local proxy. The search route keeps its own `ApiLimits`, so it needs nothing from the lazily built API router.
- **Only a search with words counts.** The empty search page is a plain form and costs nothing. 120 a minute lets a reader type in the live search (one request per 250 ms pause) and lets a small office share one address.
- **Past the limit, the answer is the empty search page with status 429, `Retry-After` and one sentence that says to wait**, as a search the filters refuse gets the empty page with 400. While typing, the reader script ignores any answer that is not 200, so it shows nothing new. The query is never shown back.
- **Every operation behind a key lists the 429 in `openapi.json`** through one shared `TooManyRequests` answer, with `Retry-After` and the `Problem` body, added where the document is finished, as `Accept-Version` is. An operation that lists its own 429, such as the media search, keeps it.

## Consequences

- A reader past the search limit sees the form under one sentence that says to wait a minute, then search again: the search page's third mode, `Busy`, beside `Live` and `Locked`.
- The search and API limits are counted in two memories of at most 50,000 buckets each.
- `khalid mcp` and the Ghost Content API use the site's `[limits]` too.

# Consent has categories, and a reader with GPC gets a kept page of their own

**Number:** next free.

**Status:** Proposed, 2026-11-08. Privacy plan U3, KTD1, KTD3 to KTD5, KTD9, KTD15. Supersedes ADR "consent is a form and a cookie" for the cookie, the form and the router; its page marks and its per-request page for a reader who answered stand.

## Context

The first consent banner was one yes or no for all of the site's code, and showed only when `[code]` waited. The privacy plan asks for five fixed categories, Global Privacy Control honoured, old answers mapped, embeds that ask for themselves, and new readers still served from kept pages with a tag and a Brotli copy (R1, R3 to R6, R11, R23).

## Decision

- **The cookie is `khalid-consent=2.<rev>.<bits>.<day>.<flags>`.** `rev` is U2's short hash of the categories that hold code, their vendors, the fixed list of embed providers and the lifetime. `bits` holds Preferences 1, Analytics 2, Marketing 4, External media 8. `flags` holds 1 for "chose despite GPC" and 2 for "answered only External media". A cookie whose `rev` is not the site's counts as no answer, so a new vendor or lifetime asks again and a wording change does not. It carries no identifier and is not signed: changing it changes only that reader's own pages. It lasts 30 days per month of the configured lifetime.
- **Old cookies:** `no` reads as Reject all; `yes` reads as no answer, because it never covered categories.
- **The form** posts `choice` = `accept-all`, `reject-all`, `save` with one switch per category named by its key, or `allow-external-media`, to `<mount>_consent/`. It keeps the `Sec-Fetch-Site` check (403) and the 2 KB limit (413), returns 400 for any other choice, and answers 303 to the same-site `back` page with `?consent=saved`, which shows the notice between the `khalid-saved` marks.
- **GPC.** A request with `Sec-GPC: 1` keeps Marketing off, or every category when the owner reads GPC as Reject all. Accept all under GPC sets every bit the signal leaves on. Save under GPC with a locked switch on sets the "chose despite GPC" flag, and that reader's choice stands. `/.well-known/gpc.json` answers `{"gpc": true}`.
- **Two kept pages per HTML page.** A page holding the `khalid-gpc` or `khalid-no-gpc` marks also keeps a GPC copy, made at render, with its own tag and Brotli copy: the `khalid-no-gpc` parts cut and the `khalid-gpc` parts shown. Pages without those marks keep one copy, so a site with no banner costs no memory. Kept pages say `Vary: accept-encoding, cookie, sec-gpc`; per-request pages say `Vary: cookie, sec-gpc`.
- **A reader with an answer gets a page of their own,** as before: the banner cut and the change form shown, the slots filled with the code of each category released to them, and the players between the `khalid-external-media` marks shown when External media is. A reader who allowed only External media from a card keeps the banner.
- **Every site uses the consent router,** not only one whose code waits: embed cards and the choices page can give any reader an answer.
- **An outside certified tool** (`[privacy] external_tool`) writes every piece of code unheld and shows no banner; the router still fills players for External media.
- **Search results** are drawn with the same code, slots and banner, and filled per reader through `khalid_server::drawn_for`.
- **A page on a domain the owner allows waits too** (privacy U10). `[media] embed_domains` lets a post frame a page from another site, such as a map or a calculator. That page is another site like a video player, so it gets the same card, held frame and marks, named by its host. Its frame keeps its own height and its forms.
- **Proven in a browser, not only in tests** (privacy U10). The layout check opens every template's demo with no consent, scripts on and off, and fails on any request to another address (`scripts/demo-check/network.mjs`, proved by `just network-selftest`). The served demo check does the same on every page it opens. The showcase site carries a tagged script, so every template is checked with the banner showing.
- **The server crate stays free of `khalid-core`.** It has its own `Categories` bit set; the `khalid` crate maps U2's `ConsentCategory` onto it, and an unknown category maps to Marketing, the strictest.

## Hooks for the units that follow

- U4: the banner and choices page post `choice` and the category keys above; the Marketing row goes between `khalid-no-gpc` marks, the honoured note between `khalid-gpc` marks, and the saved notice between `khalid-saved` marks.
- U5: each player goes inside `<!--khalid-external-media--><template>` and `</template><!--/khalid-external-media-->`; "Always load external media" posts `choice=allow-external-media`.
- U6 and U7: `consent_route::rev` is the one place the hash is read.

## Consequences

- A GPC reader with no answer costs what any new reader costs. A site with a banner keeps two copies of each HTML page in memory.
- Not checked on this server: a real browser sending `Sec-GPC`. The end-to-end test sends the header by hand.

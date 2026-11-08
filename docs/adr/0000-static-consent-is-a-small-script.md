# Static sites ask for consent with a small script, and fail closed without it

**Number:** next free.

**Status:** Proposed, 2026-11-08. Privacy plan U6, KTD7, KTD9. Supersedes the "a static site cannot ask" point of ADR "consent is a form and a cookie".

## Context

A static host only sends files, so it can neither read a cookie nor change a page per reader. The consent ADR therefore refused `mode = "static"` with code that waits for consent. The privacy plan (R1, R5, R14) asks static sites for the banner, categories and embed cards with scripts on, and that nothing optional ever loads with scripts off.

## Decision

- **Held code is inert in the page.** `khalid build` writes each held `script` as `type="text/plain"` with `data-khalid-consent="<category>"`, its own `type` kept as `data-type`. Every other held element (`style`, `meta`, `link`) sits inside `<template data-khalid-consent="<category>">`. No browser runs or fetches either. Each `[[code]]` entry waits for its own category (U2); `no-tracking` code is written open; a post's own head code waits as Marketing (KTD2). With an outside certified tool declared, the code is written open and no banner shows (KTD15).
- **`consent.js` does what the server does.** It reads the `khalid-consent` cookie in the version 2 form of KTD3, `2.<rev>.<bits>.<day>.<flags>`: bit 1 Preferences, 2 Analytics, 4 Marketing, 8 External media; flag 1 "Marketing allowed despite GPC", flag 2 "answered only External media". It makes a new `script` for each one allowed, puts allowed templates in place, opens the players of `.embed template` once External media is allowed, and shows the banner until the reader answers. `rev`, the lifetime in days, the GPC mode and the cookie path come from `data-khalid-*` attributes on its tag. `rev` is the same hash the server puts in its cookies (U3's `consent_route::rev`), so the cookie U3 parses and the one the script writes are one format. A month of the lifetime is 30 days, as on the server.
- **Global Privacy Control.** With `navigator.globalPrivacyControl`, Accept all leaves Marketing off, or every optional category when `[privacy] gpc = "reject-all"`. Such a category runs only when the reader ticks it on the choices page, which sets flag 1.
- **Buttons, not forms.** The static build locks every `<form>` (ADR "static mode locks forms after the page is rendered"), so U4's banner and choices page templates draw `type="button"` buttons the script answers when `ConsentKeeper::Script` keeps the choice, and the plain forms when `ConsentKeeper::Server` does. A static site that asks nothing gets `ConsentKeeper::Nobody`: its choices page says there is nothing to choose. A button inside an embed card's form is answered too, so its form is never sent.
- **The banner follows the skip link, hidden, with the script right after it** (changed in privacy U10). It carries `hidden` until the script shows it. The script is not deferred: it runs as soon as it is read, shows the banner before anything below it is drawn, and leaves the rest of its work until the page has loaded. So the banner takes its own room in the flow, as the server's does (KTD10), and showing it moves nothing. U6 first put the banner at the end of the body, stuck to the window's foot; once the showcase carried a tagged script, the layout check found that bar covering text and pictures in every template while a reader scrolls. The script is cached after the first page, and it is under 2 KB.
- **The script loads only where a page asks.** A site whose code waits loads it on every page; otherwise only pages with an embed and the choices page load it. A site that asks nothing gets no script, and its choices page says there is nothing to choose.
- **`gpc.json` is the server's file.** The build writes the same body U3 serves. A blog in a folder gets it in `_root/` with the other files for the root of the host.

## Consequences

- A static site with code that waits now builds instead of failing at start.
- The pull request includes U2, U3 and U4, and resolves their seams: U4's banner on U3's categories, the search route on U4's fallible banner, and U4's choices test on U3's cookie.
- Code that already ran cannot be stopped when a reader withdraws; the next page honours the new choice.
- With scripts off, an embed card's "Always load external media" form is locked like any other form on a static site, and "Load media" is the provider link (U5).

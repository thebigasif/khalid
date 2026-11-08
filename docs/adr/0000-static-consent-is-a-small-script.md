# Static sites ask for consent with a small script, and fail closed without it

**Number:** next free.

**Status:** Proposed, 2026-11-08. Privacy plan U6, KTD7, KTD9. Supersedes the "a static site cannot ask" point of ADR "consent is a form and a cookie".

## Context

A static host only sends files, so it can neither read a cookie nor change a page per reader. The consent ADR therefore refused `mode = "static"` with code that waits for consent. The privacy plan (R1, R5, R14) asks static sites for the banner, categories and embed cards with scripts on, and that nothing optional ever loads with scripts off.

## Decision

- **Held code is inert in the page.** `khalid build` writes each held `script` as `type="text/plain"` with `data-khalid-consent="<category>"`, its own `type` kept as `data-type`. Every other held element (`style`, `meta`, `link`) sits inside `<template data-khalid-consent="<category>">`. No browser runs or fetches either. Until U2 tags each `[code]` entry, all owner code and a post's own head code are Marketing (KTD2).
- **`consent.js` does what the server does.** It reads the `khalid-consent` cookie in the version 2 form of KTD3, `2.<rev>.<bits>.<day>.<flags>`: bit 1 Preferences, 2 Analytics, 4 Marketing, 8 External media; flag 1 "Marketing allowed despite GPC", flag 2 "answered only External media". It makes a new `script` for each one allowed, puts allowed templates in place, opens the players of `.embed template` once External media is allowed, and shows the banner until the reader answers. `rev`, the lifetime in days and the cookie path come from `data-khalid-*` attributes on its tag. The build computes `rev` from the categories with code, the fixed embed providers and the lifetime until U2 owns that hash.
- **Global Privacy Control.** With `navigator.globalPrivacyControl`, Accept all leaves Marketing off. Marketing runs only when the reader ticks it on the choices page, which sets flag 1.
- **Buttons, not forms.** The static build locks every `<form>` (ADR "static mode locks forms after the page is rendered"), so the banner and the choices page use `type="button"` buttons the script answers. A button inside an embed card's form is answered too, so its form is never sent.
- **The banner ends the body, hidden.** It carries `hidden` until the script shows it. At the end of the body, showing it moves nothing above it, so it adds no layout shift. U4's `consent.html` template replaces this markup when it lands.
- **The script loads only where a page asks.** A site whose code waits loads it on every page; otherwise only pages with an embed and the choices page load it. A site that asks nothing gets no script, and its choices page says there is nothing to choose.
- **`gpc.json` is a fixed file.** Its `lastUpdate` is the day static sites began to honour the signal, so builds stay reproducible. A blog in a folder gets it in `_root/` with the other files for the root of the host.

## Consequences

- A static site with code that waits now builds instead of failing at start.
- The banner, choices markup, cookie bits and `rev` are written against the plan's KTD3 and U4's description, before U2, U3 and U4 exist. Whichever lands second aligns the names: U2's `rev`, U3's cookie parser and U4's templates.
- Code that already ran cannot be stopped when a reader withdraws; the next page honours the new choice.
- With scripts off, an embed card's "Always load external media" form is locked like any other form on a static site, and "Load media" is the provider link (U5).

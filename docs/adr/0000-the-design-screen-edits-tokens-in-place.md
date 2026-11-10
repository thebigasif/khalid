# 0000. The Design screen edits theme/tokens.toml in place

**Number:** next free. **Status:** Accepted, 2026-11-10. Phase 4, task 4.7 (task/p1-design-1..7).

## Context

The admin's Design screen (mockup `docs/design/admin-mockups/design.html`) changes the colours, fonts, shape and logo in `theme/tokens.toml`. Two readers of that file already exist: the renderer's `Tokens::load_over`, which takes a partial file over the chosen pack's tokens, and `khalid-theme`'s `ThemeTokens`, which theme sync uses and which needs every field. Rewriting the whole file with `ThemeTokens` would write all seven colours into a site that set none, and so hide the pack's palette (ADR "a template pack carries its own default colours"). It would also drop the comments a person wrote.

## Decision

- **One model, `khalid_theme::DesignFile`, edits the file as a `toml_edit` document.** A change sets only the tokens sent, each in its place with the comment beside it. `[type]`, `[space]`, the font files and every other table stay as written. A site with no file gets one holding only what was changed, so the pack keeps the rest.
- **The values shown are the ones pages use:** the file's value, else the pack's, else the light default, layered as the renderer layers them and checked with `ThemeTokens`' rules.
- **Each value is checked with the renderer's rules before anything is written.** A refused value is a 422 naming its token; nothing is saved.
- **`updated_at` is the file's modified time,** as for `site.toml` on the Settings screen. A write always moves that time past the one it was read with, by one microsecond when the file system's clock has not ticked, so a stale copy is always a 409.
- **`khalid-api` depends on `khalid-theme`,** as it already depends on `khalid-lint`, `khalid-seo` and `khalid-media`. The route reads and writes the site's file itself through `DesignDesk`, and asks `khalid serve` only for the chosen pack's tokens, through a function, so a template chosen on the Look screen counts at once.
- **The route sits beside Settings and needs `settings:write`,** which among staff only the owner and admins hold.

## Consequences

- `khalid theme sync` still needs a full file. A site whose only file came from the Design screen, with some tokens, gets the same "missing field" message a hand-written partial file gets today; a site that synced first keeps a full file, and the Design screen keeps it full.
- Ticking "Keep when syncing" writes the `locked` list, so the next sync keeps that value (tested in `design_edit.rs`).
- `khalid serve` already reloads `theme/**` before each full render (#941), so a saved change shows without a restart.

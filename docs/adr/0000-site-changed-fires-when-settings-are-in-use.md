# 0000. `site.changed` fires once saved settings are in use

**Number:** next free. **Status:** Proposed, 2026-11-09. Audit brief G25, KH-API-3. Replaces the parts of the ADR "`site.changed` fires from a saved `site.toml`" that say where the event fires and what `restart_needed` means.

## Context

The first `site.changed` compared `site.toml` with each render of the Content API's catalog and always said `restart_needed: true`, because `khalid serve` read its settings only at start. Since then `khalid serve` reads `site.toml`, `routes.toml` and every file under `theme/` again before the full render that follows a change (`crates/khalid/src/site_reload.rs`). Most settings now apply at once; `mount` and `mode` keep their start value, and a few, such as `address`, `[mail]` or `preview_link_days`, reach the pages at once but the rest of the server only at the next start. A change to the theme or the routes fired nothing.

## Decision

- **`site.changed` fires from the settings reload, right after the new settings are in use.** The pages render with them next. One reload is one event, so a save fires once. The catalog no longer reads `site.toml`, so nothing fires twice.
- **Every settings file counts:** `site.toml`, `routes.toml` and each file under `theme/`. The body lists them under `files`, relative to the site folder, in name order.
- **`site.toml` counts only when a setting in it changed.** `changed` names its top-level settings and tables that changed, never their values, as before. A save that changes only comments or spacing fires nothing. `file: "site.toml"` stays in the body when `site.toml` is among the files, so receivers written for the first body keep working.
- **`restart_needed` says the truth:** `true` only when some change waits for the next start, the same settings the server's restart notice names.
- **A change that does not read fires nothing.** The server keeps the last good settings; the next good save fires with every file changed since the settings last in use.
- **Entry `.scheduled` events stay unfired.** Pages, glossary terms, case studies and changelog entries have no publish time in their front matter or through the entry routes, so no moment exists to fire them. The webhook docs say so, rather than firing them at a made-up moment.

## Consequences

- A theme save, such as a new `theme/site.css` or `theme/tokens.toml`, now tells receivers.
- `site.changed` needs `khalid serve`. `khalid build` fires no webhook, so a static or headless site that wants deliveries runs `khalid serve` beside it.
- A theme or routes file saved again with the same bytes counts as changed, since the reload goes by each file's size and change time.

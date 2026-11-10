# 0000. The trash empties itself after `trash_days`

**Number:** next free. **Status:** Proposed, 2026-11-10. Backlog 2026-11-10 row 16 (G52, KH-EDT-6). Amends "Deleted posts wait in `trash/`".

## Context

The trash ADR said nothing empties the trash on a timer, so a deleted post stayed under `trash/` until a person deleted it for good. Gap audit 3 lists that as a gap: on a busy site the folder only grows, and every list of the trash reads every copy. WordPress, Ghost and Strapi all empty their trash after a set time; WordPress waits 30 days.

## Decision

- **`trash_days` in `site.toml` sets how long a deleted post waits**, 30 unless set, up to 3650. `trash_days = 0` keeps the old behaviour: nothing is emptied on its own.
- **`khalid serve`'s scheduler empties the trash at start and then once an hour.** A copy whose delete time, the stamp in its key, is more than `trash_days` days old is deleted for good, under the same lock as restore and delete, so a restore never meets a half-deleted folder. A folder in `trash/` that is not a readable copy is left alone.
- **The value is read from the settings the pages use now**, so a saved `trash_days` takes effect at the next hourly round with no restart.

## Consequences

- The person still decides: the delete itself is theirs, and the 30 days are written in the docs and the changelog. An owner who wants every copy kept sets 0.
- A static site has no server and no scheduler, so its trash is not emptied. Static sites delete through the Admin API of a running server anyway.
- Each emptied copy is logged with its key.

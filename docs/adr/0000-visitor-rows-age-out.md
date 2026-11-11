# 0000. Visitor rows that nobody needs age out

**Number:** next free. **Status:** Proposed, 2026-11-12. Gap audit 5 row 3, KH-SEO-8, RULES.md BL-FS-1.

## Context

Some visitor tables in `data/site.db` grow for as long as the site runs. The 404 log held at most 10,000 addresses and then dropped every new one, so once scanners had filled it, real 404s and their spike alerts were never recorded again. Webhook deliveries keep their body and every attempt for ever. Sessions and sign-in links stay in the table after they can no longer be used. ADR "stored redirects answer after pages" set the 10,000 cap and the drop.

## Decision

- **A full 404 log makes room.** A new address takes the place of the one with the fewest hits, and of those the one seen longest ago (then by address, so the choice is stable). The cap stays at 10,000. An address hit twice outlives a wave of one-hit probes. This replaces "new ones are dropped" in the earlier ADR.
- **One prune, at start and once an hour, in `khalid serve`.** `khalid_db::Prune` deletes, for one site and in one transaction:
  - 404 log addresses whose last request is more than 90 days old;
  - webhook deliveries that were delivered or given up more than 30 days ago, with their attempts (by the foreign key);
  - sessions past their 14-day lifetime or idle for 12 hours, the same test `khalid-auth` uses to refuse them;
  - sign-in links past their 15-minute expiry, used or not.
  A delivery still being tried is never deleted, however old.
- **Fixed ages, not settings.** 90 and 30 days are constants in `khalid-db`. Nobody has asked to change them, and a setting is easy to add later.
- **Its own task, not the scheduler.** The prune runs beside the scheduler with its own stop, so it does not touch the scheduler's signature, which open work changes.

## Consequences

- The database of a long-running site stays bounded by traffic of the last weeks, not of its whole life.
- The Webhooks screen shows the last 30 days of finished deliveries.
- When full, each new 404 address costs one scan of the site's 10,000 rows in the writer task, off the request path.
- `khalid build` and the CLI do not prune; only a running server writes these rows.

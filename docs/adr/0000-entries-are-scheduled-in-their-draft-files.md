# 0000. Pages and collection entries are scheduled in their draft files

**Number:** next free. **Status:** Proposed, 2026-11-09. Audit brief G25, KH-API-3. Follows "Page and collection entry webhooks fire from saved files" and "A post's schedule lives in the database, beside its file".

## Context

The catalogue has `page.scheduled`, `glossary-term.scheduled`, `case-study.scheduled` and `changelog-entry.scheduled`. Until now they never fired: pages, glossary terms, case studies and changelog entries had no publish time, so nothing could schedule one. Posts keep theirs in the `post_schedules` table because the publish door owns them and `PostFront` refuses unknown fields. These entries have no door. They change when someone saves their file, `khalid serve` sees each save, and a draft already waits at the same place under `drafts/` as it would have under `content/`.

## Decision

- **A draft's front matter holds its time:** `publish_at:` with an RFC 3339 time, such as `2026-12-01T09:00:00+01:00`. The content reader never reads `drafts/`, so the field cannot break a render. A time that does not read leaves the draft unscheduled; it is logged through `tracing`, which `khalid serve` does not show yet, so the admin's future schedule field should check it.
- **The file comparison that fires the other entry events fires `.scheduled` too.** A new or moved time fires it with `publish_at` in UTC. A time taken out, or a scheduled draft removed without going live, fires it with `publish_at: null`, since the catalogue has no `.unscheduled`. A draft whose file is under `content/` in the same render went live, and fires only `.published`.
- **The scheduler in `khalid serve` puts a due draft live** in the same five-second round as the posts: it writes the draft to `content/` without the `publish_at:` line, whole and atomically, then removes the draft. The next render fires `.published`, or `.published.edited` when the draft replaced a live copy, which makes a scheduled update of a live page possible.
- **`post.scheduled` also fires when a post's time is taken away**, with `published_at: null`, so a receiver keeping a calendar hears of every change in both kinds.

## Consequences

- Scheduling an entry needs no database and survives a lost one: the time is in the file the writer sees.
- Only `khalid serve` puts entries live at their time, as with posts. `khalid build` leaves drafts where they are.
- The entry routes of the Admin API do not set `publish_at:` yet. When they do, they write the field and the same comparison fires the events, so the routes must not fire them as well.
- A time passed while the server was stopped is acted on at the next start, which then fires `.published` but no `.scheduled`, as with any change saved while the server was stopped.

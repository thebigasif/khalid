# 0000. Welcome sequences and counted links

**Number:** next free. **Status:** Proposed, 2026-11-10. Brief G39 in `docs/audits/2026-10-27-gaps-2.md`, backlog row 20 of gap audit 3, KH-GRO-2. Follows ADR "A newsletter email goes out once, under its key", whose consequences this closes.

## Context

The plan promises "a welcome email with a short sequence after signup" and "open and click analytics". The founder decided (phase 5 answers, newsletter 3) that numbers are totals per email, never per person, that open counting is off, and that click counting is a site setting. The earlier ADR left three gaps: no welcome when a confirmed reader joins a second newsletter, welcome files read once at start, and clicks not counted.

## Decision

- **A sequence is files named by day.** `welcome.md` is day 0, sent the moment a reader joins. `welcome-day-<n>.md`, with `n` from 1 to 365 and no leading zero, is sent `n` days later. No front matter, as before. A file name says when it goes out, so the writer never edits a schedule elsewhere.
- **A step is due from the later of confirming and joining.** For the first newsletter that is the confirmation; for one joined later it is the join.
- **Each step is claimed before it goes out.** `welcome_steps` (migration 0032) holds one row per subscriber, newsletter and day, written before the email is sent, so no step is sent twice and a failed one is not tried again, as with sends. This is the one place a per-reader record is kept, and only to stop repeats; it is deleted with the subscriber.
- **Later steps go out in the hourly alert round of `khalid serve`.** A step more than seven days past its day is skipped, so a step written long after a reader joined does not reach them out of the blue, and a server stopped for a few days still catches up.
- **A confirmed reader joining a new newsletter** is told apart by the store (`SignedUp::JoinedNow`) and gets that newsletter's day-0 welcome.
- **Welcome files are read each time** a reader is due one, in `spawn_blocking`, through a `WelcomeFiles` trait the binary implements. They are a few hundred bytes, read only on a confirmation or in the hourly round.
- **Click counting rewrites links once per email, not per reader.** With `[mail] count_clicks = true`, each `http` or `https` link in the issue's text is stored in `newsletter_links` with a number, and every copy carries `<blog>/subscribe/click?send=<id>&link=<n>`. The route adds one to that link's count and answers 302 to the stored address. Because the link is the same in every copy, a click cannot be tied to a reader. Only stored links lead anywhere, so the route is no open redirect.
- **Welcome emails are not counted**, as they are not sends with a key.

## Consequences

- Mail programs that open links to scan them add clicks; the docs say so.
- If a site removes `[mail]`, counted links in old emails answer "sign-ups are not open" instead of leading on.
- New newsletter folders still need a restart to take sign-ups; only the welcome files are read live.
- A step through `khalid serve` on a real day has no end-to-end test, since it needs a clock days ahead inside the server; `Newsletter::send_due_welcomes` is tested against SQLite with fixed clocks.

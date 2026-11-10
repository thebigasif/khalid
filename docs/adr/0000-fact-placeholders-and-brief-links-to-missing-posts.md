# 0000. Posts show facts with [[fact:key]], and briefs may not plan links to missing posts

**Number:** next free. **Status:** Proposed, 2026-11-10. GEO phase, tasks P6 and P7, gap audit 3 row 17 (KH-LNT-2, KH-LNT-3, KH-CON-6). Taken under the founder's yes in advance of 2026-10-27. It follows the two earlier records, "A site's facts live in facts.toml" and "A post's brief lives in briefs/", and changes three of their consequences.

## Context

The fact registry left three things open: posts could not pull a value from it, a fact had no `source`, and a `checked` date in the future kept `fact-not-checked` quiet for good. The brief left one open: nothing told a writer that a link the brief plans goes to a post the site does not have, so `brief-link-missing` pushed the writer to add a link to a page that does not exist.

Neither Steve Toth nor Koray Tugberk Gubur says how a placeholder is written or how a planned link is checked. Koray Tugberk Gubur asks a source to keep its declarations consistent, which a single value shown everywhere does by construction. The details below are our own calls.

## Decision

- **Placeholder syntax.** A post writes `[[fact:pro-plan-price]]`. It reuses the wikilink brackets, as `[[glossary:slug]]` already does, so code spans and code blocks keep it as written and table cells work as wikilinks do. The page shows the value as plain text; text after a pipe is ignored, because the value is the point.
- **Facts are content.** `facts.toml` is read by the content reader with the posts (`SiteContent::facts`), so build, serve, the APIs, search, lint and the publish door all see one registry. A broken `facts.toml` now stops every command that reads the content, as a broken post does.
- **Unknown keys stop the build.** A `[[fact:key]]` whose key is not in `facts.toml` stops `khalid build`, `khalid serve` and the publish door with the file and line, as a missing snippet does, since the page would have a gap. `khalid lint` stops with the same message. Where no body check runs, as when SerpDad's backend passes its own posts to the linter, the new rule `fact-unknown` names the key and blocks by default. `broken-wikilink`, whose message is about pages, leaves `fact:` alone.
- **`source`** is an optional full `http` or `https` address of the page that shows the value. `fact-not-checked` and the new rule below suggest it as the place to check.
- **`fact-checked-in-future`** warns on each post that gives a fact whose `checked` date is more than one day ahead. The day of slack covers a writer whose today is ahead of the server's.
- **`brief-link-unknown`** warns about a planned link to an address in a folder where the site's posts live, such as `/blog/`, that no post, page or glossary term has. An old address of a post suggests its current one. Tag, list and home pages are never judged, because the linter does not know them.
- `brief-link-unknown` and `fact-checked-in-future` warn. A site that wants a hard stop sets them to `block`.

## What we did not do

A brief file whose post does not exist, such as `briefs/en/a-typo.toml`, is still not flagged. A brief is normally written before its post, and drafts live outside `content/`, so the linter cannot tell a typo from a post still being written. The record "A post's brief lives in briefs/" chose this, and it stands.

## Consequences

- Changing a price is one edit to `facts.toml`; every post that names the fact shows the new value at the next build, and the linter reads the value the reader sees.
- `khalid serve` re-reads `facts.toml` when it re-reads the content. An edit to `facts.toml` alone shows after the next content change or a restart.
- The admin editor does not offer facts yet; a writer types the placeholder.

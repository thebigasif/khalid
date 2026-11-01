# A blocked imported post holds its old addresses

**Number:** next free. **Status:** Proposed, 2026-11-01. Fixes findings A and B of the second 1.0 readiness check. Builds on "an import plans first and maps every old address" and "a blocked post is left out of the site".

## Context

The second readiness check imported a WordPress blog onto a new host at its root. Its one old post had no internal link, so the publish gate blocked it, and build and serve left it out. But the import had already stored two 301s to it, from `/2024/03/keep-a-starter-alive/` and its image's page. A reader or a search engine following either old address was sent to the not-found page. On a real migration most old posts link to no Khalid post yet, so most old addresses would end on the not-found page on the day of the move: the very loss an import is meant to prevent (KH-API-7, KH-SEO-8).

The same check found that `/about/` and `/category/baking/`, whose paths do not change, were counted as moves and stored as redirects to themselves.

The report offered three ways out: import blocked posts as drafts with their redirects held, treat the link block as a warning on imported posts, or only warn in the summary.

## Decision

- **After writing the files, the import judges the site as `khalid build` does,** with `build_gate::screen`, so it sees exactly which imported posts readers will not see.
- **Each blocked imported post holds its redirects in its own `aliases`.** Every exact row of the map that leads to the post's address is taken out of what the import stores. Its old path goes into the post's `aliases` front matter instead. An alias goes live with the post, and only then, at the address the post has at that moment, in build and serve alike, with no further step.
- **An old path outside the blog's folder cannot be an alias,** because Khalid never sees it. Such rows go into `import/held-redirects.csv`, and the summary says to store them with `khalid redirects import` once their posts are live.
- **The post stays dated in `content/`, not moved to `drafts/`.** Publishing a draft dates it today, which would replace the date the post had for years and, under a dated route, its address too. A dated post the gate blocks is already left out, and goes live with its own date the moment it passes.
- **The summary says what was held,** in plain words: which posts are blocked, how many old addresses wait in their aliases, and how many in the held file. `--json` adds `held_posts` and `held_redirects`.
- **An address that keeps its path stays the same.** The WordPress and Ghost reports count a row as staying when its target has the same path, whatever the host; a Redirection plugin rule never counts, since it may lead to another site. A row that sends a path to the same path on the blog's own host is not stored, because the page answers it. The CSV file keeps every row, for the old host.

The link block was not softened for imported posts: the linter's rules are the founder's, and a post with no internal link is still not ready.

## Consequences

- No old address is ever sent to a page readers cannot see. Until a held post is fixed, its old address answers 404 directly, which search engines treat the same as a redirect to a 404 but without the extra hop.
- Fixing a post is the only step. `khalid lint` says what blocks it, and the next build or save brings the post and every old address with it.
- A pattern rule from the Redirection plugin may still lead to a held post's address. It cannot be tied to one post, so it is stored as before.
- The dry run cannot know which posts the gate will block, because it writes nothing to judge. It reports the map as before.

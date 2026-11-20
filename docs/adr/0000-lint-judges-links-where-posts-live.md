# 0000. The linter judges a site link only in a folder that holds posts or pages

**Number:** next free. **Status:** Proposed, 2026-11-20. Backlog 2026-11-12 row 18 (plan section 6, KH-LNT-2).

## Context

`khalid lint` checked `[[wikilinks]]` against the site, but an ordinary markdown link such as `[guide](/blog/no-such-guide/)` passed, however wrong. `redirecting-link` read front matter `aliases` and never the moves stored in `data/site.db`.

Knowing every address a site answers exactly would mean running the build: list pages, year pages, paged lists, feeds and the routes `khalid serve` adds all have addresses. Lint runs on 10,000 posts, so it cannot render the site first. A blog mounted at `/blog/` also links to its main site at `/pricing/`, which Khalid knows nothing about.

## Decision

- `SiteView` gains `known_paths`: the addresses the site answers and the folders its posts and pages sit in. The `khalid` binary fills it from the posts, pages, case studies, changelog entries, author and collection pages, each language's home, and a fixed list of pages the build and the server write (`archive/`, `search/`, `subscribe/` and the rest).
- The new rule `broken-internal-link` (warn) judges a link only when its folder holds a post or a page. Anything else, such as the main site beside a mounted blog, a tag folder or a paged list, is left alone. A folder above a known address, a file (a last part with a dot) and an old address that redirects are never broken.
- `SiteView` gains `redirects`, the moves stored in `data/site.db`. `redirecting-link` suggests their target as it does an alias's. `khalid lint` reads them only when the database exists, and never makes `data/`.
- With no `known_paths`, the rule skips, so other callers and every older fixture behave as before.

## Consequences

- The common mistake, a typo or a deleted post in a link beside the posts, is caught before publishing, with no build.
- A new fixed page the build writes beside the posts must be added to the list in `crates/khalid/src/lint_paths.rs`, or links to it warn. The rule is a warning, so a miss never stops a publish.
- `khalid build` reads the stored moves for its publish gate as `khalid lint` does. The post checks inside `khalid serve` get the known addresses but not yet the stored moves, so there a link to a stored move warns as broken instead of redirecting. Wiring the live store in is a follow-up.
- A site that links on purpose to a second site published under its own folder, as Khalid's website does with `/khalid/demo/`, turns the rule down to a tip in its `lint.toml`.

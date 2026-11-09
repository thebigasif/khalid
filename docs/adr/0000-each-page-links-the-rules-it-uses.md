# 0000. Each page links the rules it uses

**Status:** Proposed, 2026-11-07. Supersedes the "one stylesheet for every page" part of ADR 0022.

## Context

ADR 0022 chose one linked stylesheet for every page, when it weighed 1.8 KB with Brotli. Since then the shared rules, the template packs and the section parts have grown it to 9 to 14 KB with Brotli per template, and every page downloads all of it: a post ships the rules of the home page, and the home page ships the rules of every body block. The budget is now at most 10 KB of CSS per page (docs/standards/performance.md, "CSS weight").

Rules were measured three ways on the 31 template demos. Leaving out the rules a page's templates never name brings the home and list pages under 10 KB but leaves posts at up to 11.7 KB, because a post may hold any body block. Leaving out the rules a page's own HTML cannot match brings every page to 9 KB or less. Inline CSS is still ruled out by the Content Security Policy (ADR 0022).

## Decision

Each page links one stylesheet of its own: the site's stylesheet without the selectors that name, outside any brackets, a class no element of the page has.

- The renderer reads the whole stylesheet into rules once per theme (`PageStyles` in the `khalid` crate). After a page is rendered, its `class` attributes pick the rules; the page's `<link>` then names that sheet, `site.<hash>.css`. Pages with the same classes share one sheet, made once.
- A selector with no class, or one that names a class only inside `:is()`, `:not()` or `:has()`, or one with an escaped character, stays on every page. So do `@font-face`, `@keyframes` and every at-rule other than `@media`, `@supports`, `@container` and `@layer`.
- The classes the reader script adds count as on every page. So do all of the site's own rules from `theme/site.css`, since a script of the site's may add the classes they name.
- The whole stylesheet is still built and served. Pages made outside the site's render, such as a comment's refusal page or a live search's results, link it as before.
- Each HTML answer of `khalid serve` names its own sheet in its preload `Link` header.

## Consequences

- Every demo page downloads at most 10 KB of CSS with Brotli. `crates/khalid/tests/css_weight.rs` and the layout check hold that for every template.
- A reader who moves from a post to the home page downloads a second sheet, where before the one cached sheet served both. Pages of one kind, such as two category pages, mostly share a sheet.
- A build writes one sheet per distinct set of classes. A site whose posts use many different blocks gets more sheets, each made once and kept by its hash.
- Checked on all 87 demo builds the layout check makes, on every page it opens (866 pages): no selector of the whole stylesheet that can match an element of the page, counting states such as `:hover` as matching, is missing from the page's own stylesheet. The computed style of every element on those pages, at 390 and 1440 px in Chromium (1,732 page views), was the same before and after this change; the 20 views that differed on a first read were the same on a second, as page heights also vary between two reads of one build.

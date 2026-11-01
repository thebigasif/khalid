# Home page cards show their pictures after the full render

**Number:** next free.

**Status:** Accepted, 2026-11-01. Template visual pass B.

## Context

The founder judged the ten templates by eye on the public demo and found them plain: "no pictures". Two things kept every picture off the front page.

- A post's lead image reached its cards on every list page except the home page. "khalid serve answers before the full render" asks that the home page `khalid serve` answers first be byte for byte the page the full render makes. The first home page is made from front matter alone, before any image has its sizes, so the full render left the home page's cards without pictures too.
- A lead image that is an SVG drawing was left out everywhere, because only JPEG, PNG and WebP files get sizes. The showcase's covers are all drawings.

A front page without pictures cannot carry a featured post with its cover, card thumbnails or a lead story, which nine of the ten templates are built around.

## Decision

- **The full render gives the home page's cards their pictures**: the newest posts, the featured slot, the editor's picks and every home section, as every other list already had.
- **The first home page `khalid serve` answers stays as it is**: the same page without card pictures and author photos. The full render replaces it a moment later, as it replaces every first page. `the_first_pages_are_the_full_renders_home_pages_and_stylesheet` now checks that the first home page is the full one with its pictures taken out, so any other drift still fails.
- **A lead image that is an SVG file beside its post is drawn** at the size its `svg` element gives in `width` and `height`, or else its `viewBox`, so the page keeps its space while it loads. It has one file, so it has no WebP copy. It is not used as the share image, because social networks do not show SVG; the drawn share card stays. A drawing that gives no size is left out, as before.

## Consequences

- Every template's front page shows the covers its posts have. The cold start of `khalid serve` does not change: the first pages are made exactly as before.
- For the first moment after `khalid serve` starts, the home page shows its cards without pictures. A reader who loads it again after the full render sees them.
- Body images that are drawings still have no width and height. Drawing them the same way is a separate change.

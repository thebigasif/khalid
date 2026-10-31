# 0000. The demo check finishes inside thirty minutes

**Number:** next free. **Status:** Proposed, 2026-10-31. Amends ADR 0032.

## Context

CI's Demo check job stops at 30 minutes, and every run since the job began was cancelled there. The logs showed four things. The release build took 7.5 minutes, because `rust-cache` saves only after a green run, so it never saved. Lighthouse took 5.5 minutes. Lighthouse could not score the two not-found pages at all: it refuses any page that answers an error, which is what a 404 is. The browser checks then loaded every page twelve times, at six widths with scripts on and off, one at a time, and printed nothing until the cancel 16 minutes later. The HTML validator never ran, because the script passed Java a garbled path to it.

## Decision

- The not-found pages skip Lighthouse. The validator and axe still check them at every width. ADR 0032's "held to the performance and accessibility categories" is withdrawn, because Lighthouse cannot do it.
- `browse.mjs` loads each page once per scripts setting and resizes it through the six widths, with the layout check and axe at each width. Resizing lays the page out again, so no check is lost. Four pages are checked at once, one tab each, which the four cores of a CI runner allow.
- A page load or an axe run that takes over 60 seconds fails by name, and its tab is replaced, so one stuck page cannot use up the job's time. Each page prints a line when it is done, so a slow run shows where it is.
- Lighthouse stays one page at a time, so its performance scores are not measured on a busy machine.
- `scores.mjs` prints what each failing audit found, such as the console message, so a red run can be fixed from its log.

## Consequences

- Expected time on a CI runner: 7.5 minutes to build without a cache, about 5.5 for Lighthouse and about 3 for the browser checks. That is near 17 minutes, and the build is cached after the first green run.
- A page's checks at a wider width run on the page as it was loaded at 320 px and then resized. A page that only works when first loaded at a given width would be missed. Khalid's pages lay out with CSS alone, so none does.

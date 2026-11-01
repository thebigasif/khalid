# 0000. Layout blocks are directives that read in full with scripts off

**Number:** next free. **Status:** Proposed, 2026-11-01. Templates spec sections 4.4 and 4.6 item 6, extending ADR 0012.

## Context

The ten templates need four blocks markdown does not have: a pull quote, pros and cons, side-by-side columns and a long table that folds. Each must be a directive with a round trip (ADR 0012), and each must read in full with scripts off (BL-WEB-2).

## Decision

- `:::pullquote{by="..."}`: the words as the body, `by` optional. It renders as a `<figure>` holding a `<blockquote>`, with `by` as the caption. An empty body is refused.
- `:::proscons{pros="..." cons="..."}`: one `+ ` line per pro and one `- ` line per con, each line inline markdown. The titles default to "Pros" and "Cons", so a site in another language sets them. A side with no lines is left out. Each side has its title as text, so the drawn check and cross are never the only signal.
- `::::columns` around two to four `:::column{title="..."}` blocks, each body markdown. It renders as a grid that stacks on narrow screens. Fewer than two or more than four columns is refused: one column is a callout, and five do not fit a reading column.
- `:::table{fold=8 more="..."}` around one markdown table. The first `fold` rows show; the rest sit in a second table, with the header row repeated, inside a `<details>` whose summary is `more`, or "Show 16 more rows" when `more` is not set. `fold` is 1 to 100 and defaults to 8. A table with no more rows than `fold` renders as a plain table.

The labels these blocks write ("Pros", "Cons", "Show 16 more rows") are English by default and set per block with an attribute, like every other block's words today.

## Why

- A `<details>` is the only way to fold content that opens without a script in every browser. Folding inside one table needs a script or the new `::details-content` selector, which not every browser has.
- Two tables with the same header keep each half readable on its own by a screen reader; the columns line up because both tables fill the column width with fixed layout.
- Lines starting `+` and `-` read as pros and cons in a plain text file, which is where a writer types them.

## Consequences

- The admin editor's block list in `admin/src/editor/blocks/specs.ts` needs the four names before the editor shows them as cards. Until then the editor keeps them as markdown text, which round-trips.
- The pros and cons block could feed Google's `positiveNotes` and `negativeNotes` on a review. That is schema work for the SEO track, not assumed here.

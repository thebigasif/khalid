# A block's schema node wins over the same type from front matter

Number: next free. Amends 0021 and the ADR "every type in plan section 5.9 has a typed schema builder".

**Status:** Proposed, 2026-11-09. Brief G15, last part. KH-SEO-3.

## Context

A post can now describe one thing in three places: an editor block (such as `::event` or `::::recipe`), its own `schema` field of hand-written JSON-LD, and a schema template in `site.toml` that covers its category or collection. Until G15's last part only a few blocks wrote schema, so the showcase wrote its course and its dataset by hand in the `schema` field. With a block for each, a writer who adds the block and keeps the field would publish two `Course` nodes for one course. A page typed `FAQPage` in front matter with an FAQ block also wrote two `FAQPage` nodes. Google reads two nodes of one type as two things, and may show neither.

## Decision

- **The block wins, by type.** When a block on the page writes a node of a schema.org type, the nodes of that same `@type` from the site's schema templates, the site's custom JSON-LD and the post's `schema` field are left out of the page's graph. Nodes of other types from those sources stay.
- **Why the block:** Google asks that structured data describe what the reader can see. A block is drawn on the page; the field and the template are not. The block's node is also typed and checked by Khalid's own builders.
- **Blocks never drop each other.** Two recipe blocks on one page are two recipes and write two nodes. The one exception is two location blocks that name one place: they share its node, under the id the home page gives it (`<home>#location-<n>`).
- **A page typed `FAQPage` with an FAQ block is written as a `WebPage`,** and the block's `FAQPage` carries the questions. With no FAQ block the front matter type stands.
- The match is on the exact type name. A business card typed `Bakery` does not drop a hand-written `LocalBusiness`, because they may well be two businesses.

## Consequences

- A writer can move a thing from the `schema` field to a block without first deleting the field, and the page never carries both.
- A hand-written node of a type that a block also writes is never published. The linter does not yet say so; a later lint rule can name the field node that a block replaced.
- Sites that relied on a template for a type a post also writes as a block get the block's node, with the block's own values.

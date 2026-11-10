# Custom JSON-LD names are checked against a compact vocabulary

Number: next free. Amends 0021 and the ADR "types without a block come from the post's schema field".

**Status:** Proposed, 2026-11-10. Backlog row 27 of gap audit 3. KH-SEO-3.

## Context

Hand-written JSON-LD (a post's `schema` field, the site's custom JSON-LD, and the same field in the admin and the API) is read by `CustomSchema`. It checked the shape of every name, but not the name itself, so `Eevnt`, `startdate` or `Organisation` passed. Search engines skip a type or property they do not know, and say nothing. ADR 0021 says this field is checked against the schema.org vocabulary.

The whole vocabulary is about 900 types and 1,500 properties. As a plain list of names it is about 30 KB in the binary, and the release binary is at its 30 MB budget. It also cannot be fetched on the build server, so a full table would be typed from memory, which is worse than a short table that is right.

## Decision

- **A compact table, not the whole vocabulary.** `khalid-seo/src/schema/vocabulary.rs` holds about 100 type names and 200 property names, a few KB: every name Khalid writes itself, every name in Google's rules, the names custom JSON-LD uses most, and the real names one letter away from those, such as `Resort` beside `Report`, `weight` beside `height` and `reviews` beside `review`.
- **Only a slip is refused.** A name is refused when it is not in the table and is the same as a table name in other capitals, or two letters swapped, or one letter added, dropped or changed (two for names of ten letters or more). Names under four letters are only matched by their capitals. The message names the place and the name meant: "startdate is not a schema.org name; did you mean startDate?".
- **A name far from every table name passes.** A rare but real type such as `TouristAttraction` is never refused. The cost is that a slip in a rare name also passes.
- **Refused, not warned.** Warnings from `check_graph` reach no one yet: the publish check reads only what blocks. So a slip is refused where the field is read, like every other mistake in it: the API and admin refuse to save it, and the build stops and names the file.
- **Schema templates are not checked by this yet.** Their type goes through `SchemaType`, which local business locations share, and a business type such as `Motel` sits one letter from `Hotel`. Checking them needs the business types in the table first.

## Consequences

- Tests prove every graph in the crate's snapshots, and every name in Google's rules, reads with no slip, so Khalid never refuses what it writes itself.
- Adding a name to the table is one word in a sorted list; a unit test keeps it sorted.

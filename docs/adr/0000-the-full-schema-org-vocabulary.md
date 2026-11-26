# The full schema.org vocabulary is built in

Number: next free. Amends the ADR "custom JSON-LD names are checked against a compact vocabulary".

**Status:** Proposed, 2026-11-26.

## Context

The compact table of about 300 names only caught slips near common names. A slip in a rare name passed, and nothing could say whether a property belongs to a type, what a property expects, or which names schema.org has replaced. Those deeper checks need the whole vocabulary. The release binary has about 740 KB of room under its 30 MB budget.

## Decision

- **schema.org's own published tables are the source.** `scripts/schema-vocabulary.py` reads `schemaorg-all-https-types.csv` and `schemaorg-all-https-properties.csv` of release 30.1 and writes `crates/khalid-seo/src/schema/vocabulary.txt`: 942 types, 544 enumeration members and 1,540 properties, one line each, with what each type extends, what each property belongs to and expects, and what replaces a superseded name. No other tool's data is used.
- **Plain text, read on first use.** The file is 128 KB, about 4% of the binary's headroom, and is read into maps the first time a check asks for it. Compressing it would save about 100 KB at the cost of a build step; plain text keeps each vocabulary update a readable diff.
- **The slip check uses the whole vocabulary.** Every real name passes, and a slip near any real name is refused, rare names included.
- **Updating** to a newer release is: download the two tables, run the script, and update the counts the unit test expects.

## Consequences

- schema.org publishes its vocabulary under CC BY-SA 3.0. The data file names the source and the licence in its header.
- `Vocabulary` is public in `khalid-seo`, so the deeper JSON-LD checks, the reports and the admin validator all ask the same data.

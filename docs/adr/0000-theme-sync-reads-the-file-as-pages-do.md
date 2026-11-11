# 0000. Theme sync reads theme/tokens.toml as the pages do

**Number:** next free. **Status:** Accepted, 2026-11-10. Gap audit 4, row 13 (task/p1-partial-sync-1..2).

## Context

The Design screen writes only the values a person changed into `theme/tokens.toml` (ADR "the Design screen edits theme/tokens.toml in place"). Its consequence said `khalid theme sync` still needs a full file. So a site whose file came from the Design screen, or from a hand edit with a few values, could not sync or check its theme: both commands stopped with "missing field `background`", and the Design screen tells the owner to run that command.

## Decision

- **`khalid theme sync` and `khalid theme check` read the file through `DesignFile`,** layered as the renderer layers it: the file's value, else the chosen template's, else the light default. A missing token is no longer a mistake. A value the renderer refuses still is, and nothing is written then.
- **A sync over an existing file sets only the tokens that change, in place,** through `DesignFile::set`. Comments, order, `[type]`, `[space]` and every other table stay as written. A value the template already shares with the main site stays out of the file, so the template keeps it.
- **The first sync of a site with no file still writes the whole file,** as before, so a synced site keeps the full record of what came from the main site.
- **A refused value has no line number now.** The file is checked as one layered table, and the TOML reader gives a line only for a mistake in the TOML itself. The message still names the file and the value.

## Consequences

- This replaces the first consequence of the Design screen ADR.
- A site that synced first and then saved on the Design screen behaves as before: every token is in the file, so the pack under it changes nothing.
- The sync route the Design screen needs, and the announcement bar, icon and cover image fields (backlog row 13), are left for later tasks.

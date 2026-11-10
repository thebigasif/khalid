# 0000. khalid serve reads again what khalid mcp writes

**Number:** next free.
**Status:** Proposed, 2026-11-10. Gap audit 3 row 24 (`backlog-2026-11-10.md`), G26, KH-API-5.

## Context

`khalid mcp` mounts every route `khalid serve` mounts (ADR "MCP tools come from the OpenAPI document"), but it kept the comment, media and redirect routes on stores that answer that the site has none. A running `khalid serve` holds those three in memory: the redirect table, every media `item.toml`, and the approved comments each post page shows. A write from another process reached readers only after a restart. `khalid redirects import` already had that gap and told people to restart serve.

The two programs could share one process, or talk over a socket, but `khalid mcp` runs on its own beside a stopped serve too, and a socket adds a listener and a protocol for one signal.

## Decision

- **`khalid mcp` uses the real stores**: comments from the site database (when the site is in server mode, as serve decides), the media library from `media/` or the bucket `[media.s3]` names, with photo and GIF search as serve has them, and the stored redirects and 404 log from the database, built the way `khalid redirects` builds them.
- **A writer leaves a note.** After a tool call that worked and whose tool is not read-only, `khalid mcp` writes `data/outside-writes`, a count it adds one to, so two writes in one clock tick still change it. `khalid redirects import` does the same. `data/` holds only what Khalid makes, so no content file changes.
- **serve looks at the note every scheduler round**, every 5 seconds. When it changed, serve reads the redirect table, the media library and the approved comments again. It renders the post pages again only when the approved comments differ, so a post edit through `khalid mcp`, which the folder watcher already renders, costs one small read of each store.
- **The media library merges on reload.** An item serve saved while the files were read is kept when it is newer than its file, and a file that cannot be read keeps the library as it was.
- **Comments `khalid mcp` adds fire no webhook.** serve's `comment.added` webhook comes from the reader form; staff replies through the Admin API are not readers' comments.

## Consequences

- An agent can moderate comments, upload and edit media, and store redirects, and readers see each change within about 5 seconds, with no restart.
- A note that cannot be written is said on stderr; the change is stored, and serve shows it after a restart.
- Any other process that writes those stores can join by calling `outside_writes::note`.

# 0035. The Ghost-compatible API keeps Ghost's contract

**Status:** Accepted, 2026-11-05. U26 of the Khalid against Ghost and Strapi plan.

## Context

KTD13 of `docs/plans/2026-11-04-2259-feat-khalid-vs-ghost-strapi-plan.md` adds a read-only copy of Ghost's Content API v5 under `/ghost/api/content/`, so a front end built on Ghost's client moves to Khalid by changing its address and key. Khalid's own APIs follow `docs/standards/api.md`: problem details for errors, `Khalid-Content` keys, versions in the path, and one `OpenAPI` document from which the SDK and the MCP tools are made. Ghost's client expects other things.

## Decision

- **Ghost's contract wins on this path.** Ghost's paths, wrapper, field names, pagination and error shape (`{"errors": [...]}` with Ghost's error types), `?key=` and `Accept-Version: v5.x`. Khalid's standard still holds everywhere else, and `/api/content/v1/` keeps refusing a Ghost version.
- **It stays out of `/api/openapi.json`.** The document describes Khalid's contract. Putting Ghost's in it would put Ghost's shapes into the SDK and the MCP tools. `docs/ghost-api.md` describes this path instead, and Ghost's own client, run in CI at a pinned version, is its contract test.
- **One setting, off by default.** `ghost_content_api = true` in `site.toml`. Off, the path is any unknown address, and `khalid serve` does not even route it to the APIs. Changing it needs a restart, as the other settings that add routes do.
- **One language.** A Ghost site has one. The path answers in the site's default language only, so a translation that shares its slug with the original never makes a read by slug ambiguous.
- **No body means members only.** Khalid hands no body for a members-only post. The path shows it as Ghost shows such a post to a key with no member: `visibility: "members"`, `access: false`, empty `html`.

## Consequences

- A front end that reads a multilingual Khalid site through this path sees one language. It reads the others through `/api/content/v1/`.
- A change to Ghost's client that breaks the path fails CI's Ghost client job, and the pinned version moves only by a pull request that keeps it green.
- The Ghost path reads the same store as Khalid's Content API, so what that store lists, such as tags and plain pages, shows on both.

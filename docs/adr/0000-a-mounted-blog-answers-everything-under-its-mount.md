# 0000. A mounted blog answers everything under its mount

**Number:** next free. **Status:** Proposed, 2026-11-01. Phase 1, bug from the first real deploy (KH-INS-3).

## Context

A site made with `khalid init --mount /blog/` answered its pages under `/blog/`, but the admin at `/admin/` and the APIs at `/api/`. The proxy rule `khalid init --stack` prints passes on only `/blog/`, so the admin and the APIs could not be reached at all. Sign-in links, the webhook `site` field, `Location` headers, `admin_path` and the SerpDad address on the Connect page left the mount out too. The APIs' routes are written as `/api/...` across `khalid-api`, and `khalid mcp` and the other commands call that router in process with those paths.

## Decision

- The APIs keep their routes at `/api/`. On a mounted blog, `khalid serve` wraps the whole router in `crates/khalid/src/mounted.rs`: a request under `<mount>api/` reaches the APIs at `/api/`, and its answer is given back under the mount. A `Location` path gets the mount in front, and so does `admin_path` in the two answers that carry it. The `OpenAPI` document gets a `servers` entry, the blog's address with its mount, such as `https://example.com/blog`.
- `/api/` itself keeps answering on a mounted blog, for a program that reaches Khalid directly without the proxy, and for every existing test and integration. Khalid never gives that address out.
- The admin router takes the mount and answers at `<mount>admin/`. The app's page names the mount in a `khalid-mount` meta, which the app reads for its router's base, its API calls, the "View blog" link and the SerpDad address. Vite builds with a relative base, so the lazy editor chunk loads beside its script, and the server names the page's own two files from the host's root. An old `/admin/...` address moves to `<mount>admin/...` with a 308.
- Every address Khalid gives out starts from `mounted::blog_address`: sign-in links (`staff::admin_address`), the local sign-in link, and the webhook envelope's `site`.
- The `::newsletter` block's form is pointed at `<mount>subscribe` after render, as form blocks already are (ADR "form blocks post under the mount path").

## Consequences

- A blog at `/` answers exactly as before: the wrapper is not added, the admin page's files are named under `/admin/` as Vite named them, and no `servers` entry is added.
- `crates/khalid/tests/serve_mount.rs` crawls the mounted demo and probes each family of addresses. Anything that answers outside `/blog/` must move into it or answer as its twin under `/blog/` does. Each printed proxy rule must pass on all of `/blog/`.
- Changing `admin_path` happens in the wrapper by text, on the two routes that carry it. If the field is renamed, `mounted::tests` fail and point here. Passing the mount into `khalid-api` would let that go.
- There is no MCP endpoint over HTTP: `khalid mcp` speaks over standard input and calls the API in process (ADR "the MCP server calls the API in process"), so a mount does not change it.

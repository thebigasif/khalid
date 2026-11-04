# 0034. A demo publication is an overlay with checked photos

**Status:** Accepted, 2026-11-04. U8 of the Khalid against Ghost and Strapi plan.

## Context

KTD2 of `docs/plans/2026-11-04-2259-feat-khalid-vs-ghost-strapi-plan.md` gives each template's demo a publication of its own, put over a copy of the showcase site by `scripts/demo-publication.sh`. KTD4 keeps the photographs out of git: they come from `demo-photos.tar.gz` on the `demos` release and are checked against `photos.lock`. The plan names the files but not their formats, how a `site.toml` fragment merges, or how the browser scripts learn which pages to open.

## Decision

- **photos.lock is TOML,** one `[[photos]]` entry per photo with `file`, `sha256`, `source`, `licence` and `credit`. It is easy for a person to read and for an agent to write. The script reads only `file` and `sha256`, with `awk`, so it needs no TOML tool. `file` is the photo's place inside `content/`, and inside the archive it sits at `<template>/<file>`. A path with `..` or a leading `/` is refused, since the archive is downloaded data.
- **Photos are cached outside the repository,** in `~/.cache/khalid/demo-photos` or `DEMO_PHOTOS_CACHE`. The archive is fetched only when a photo is missing from the cache or its checksum differs, then checked again. `flock` keeps the four parallel builds of the layout check to one fetch. `DEMO_PHOTOS_URL` may be an https address or a local file, so the tests run offline.
- **The fragment merge works key by key and table by table.** A key or table the fragment sets replaces the copy's. A table also replaces every table inside it, so `[seo.publisher]` takes `[seo.publisher.logo]` along. The name, languages, menus, footer groups, announcement and call to action of the showcase are always taken out, even when a fragment leaves one out, so no showcase page is linked. A comment goes or stays with the line after it. Khalid's `site.toml` has no top-level `description`, so the publication's description is its `[seo.titles]` tagline.
- **The browser scripts find a publication's pages beside its site.toml.** The script copies `check-pages.txt` there, and `site-files.mjs` reads it for both `layout.mjs` and `pictures.mjs`. Each line is a kind and a path. A `post` or `table` page gets the reading check, and the home page is always added. The second layout pass builds each ready publication as `<pack>-publication`.
- **The tests use their own sample,** `fixtures/test-publications/harbor/`, which is marked ready and never moves to `fixtures/showcase-publications/`. The build tests live in `crates/khalid/tests/demo_publication.rs`, not in `website.rs` as the plan lists. That file tests the website in `site/` and is near the 600-line limit.

## Consequences

- The default photo address is `https://github.com/thebigasif/khalid/releases/download/demos/demo-photos.tar.gz`. Until that asset exists, a ready publication with a `photos.lock` builds only with `DEMO_PHOTOS_URL` set to a copy.
- A top-level key in the copy's `site.toml` that spans several lines would not merge cleanly. The showcase has none.
- The CI workflow runs `just layout-check` and does not yet run `just publication-selftest`. The Rust tests cover the build side in CI. The self-test runs where a person or an agent asks for it.

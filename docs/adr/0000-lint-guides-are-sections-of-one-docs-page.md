# 0000. Lint guides are sections of one docs page made from the catalogue

**Number:** next free. **Status:** Proposed, 2026-11-20. Plan 6 ("it links to a SerpDad guide for the long version"), D-12, backlog 2026-11-12 row 19.

## Context

Every catalogue message names a `guide` slug, and ADR 0019 left the slug's address for later. 109 rules share 39 guides. No guide pages exist. The user docs already reach serpdad.com through the docs pack (`scripts/docs-bundle.sh`), served at `/khalid/docs/<slug>/`, and a heading there gets the usual anchor: its words in lower case joined by hyphens.

## Decision

- **One page, one section per guide.** `docs/lint-rules.md`, "What the linter checks", holds a `##` section per guide slug, whose heading is the slug in words ("Internal links" for `internal-links`), so its anchor is the slug. Each section lists the rules that link to it with their three sentences.
- **The page is made from the catalogue.** `khalid_lint::rules_page` writes it. A test in khalid-lint compares the file with the catalogue, rewrites it when they differ and fails, so a changed message cannot leave the page behind.
- **The address is fixed in khalid-lint.** `Message::guide_url` gives `https://serpdad.com/khalid/docs/lint-rules/#<guide>`. The CLI prints it, the finding JSON carries it as `guide_url` beside `guide`, and the dashboard's `SiteIssue` carries it as `guide`.

## Consequences

- Every link works as soon as the docs pack ships the page; longer guides can replace a section later without changing any address a finding gives, by keeping the anchor.
- A site cannot point the links at its own help pages. Nobody has asked for that; a `[lint] guide_base` setting could add it.

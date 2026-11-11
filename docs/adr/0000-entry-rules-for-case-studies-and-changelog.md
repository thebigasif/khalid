# 0000. Case studies and changelog entries get their own lint rules

**Number:** next free. **Status:** Proposed, 2026-11-10. Gap audit 4, row 23, plan section 5.13 (KH-LNT-2, KH-LNT-3, KH-LNT-6). Taken under the founder's yes in advance of 2026-10-27.

## Context

Plan 5.13 gives each of the three extra collections lint rules: a case study needs a client, an outcome and a date, and a changelog entry needs a date and a version or a month. `CaseStudyFront` and `ChangelogFront` leave those fields optional so the linter can explain them, but no rule read them: the lint model held only posts, pages and glossary terms, and both rule traits report against a post.

## Decision

- The lint model gains `LintEntry`: the locale, the slug `khalid build` gives the page, the date, and an `EntryKind` that carries the collection's own fields (`client` and `outcome` for a case study, `version` for a changelog entry), so a case study can never hold a version. `SiteView::entries` hands them over, empty by default, so SerpDad's backend can pass its own.
- A third trait, `EntryRule`, checks one entry at a time. `Linter::site_health` runs these rules after the site rules, so `khalid lint`, the site report and the dashboard's top issues all show them. A finding names the entry by its language and slug, as a post finding does.
- `case-study-incomplete` gives one finding per missing field, with the field as the subject. A blank value counts as missing.
- `changelog-entry-undated` fires when a changelog entry has no date. The date also gives the month the changelog groups the entry under, so "a version or a month" holds for every dated entry, and a version stays optional.
- Both rules block by default, as the plan says the fields are needed. A site lowers them in `lint.toml`.

## Consequences

- `khalid lint` exits 1 on a site with an incomplete case study or an undated changelog entry. Every sample site already passes.
- `khalid build` and `khalid serve` leave out an entry these rules block, as they leave out a blocked post (gap audit 5, row 4). `PublishGate::entry_verdict` gives the decision. The publish door in `khalid-api` has no entry routes yet, so it has nothing to hold back; a future entry route calls the same verdict.
- `Location` has a kind: a post, a case study or a changelog entry. Only findings in a post count towards a post's score, so an entry that shares a slug with a post never costs that post points.
- The third item of the row, a glossary term with no inbound link, is not built here.

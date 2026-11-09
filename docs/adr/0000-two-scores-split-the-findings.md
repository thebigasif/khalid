# 0000. The SEO and readability scores split the same findings by rule

**Number:** next free. **Status:** Proposed, 2026-11-02. Yoast package WP3, the admin SEO interface (KH-LNT-4).

## Context

Yoast shows two traffic lights per post, one for SEO and one for readability, in the editor and as columns of the posts list. Khalid already gives each post one score out of 100 from its lint findings (see the ADR "The SEO score sums up a post's findings with fixed weights"). The linter belongs to another track, and its rules carry no group, so the split has to come from the outside without changing what the linter finds.

## Decision

`SeoScores::of(report)` in `khalid-api` splits a report's findings by rule ID. The rules in `READABILITY_RULES` (`hard-to-read`, `long-paragraph`, `heading-shape`, `answer-capsule`, `plain-answer`, `comparison-answer-late`) make the readability score; every other rule makes the SEO score. Each score uses the same weights as the one score: 30 per blocking rule and 5 per warning rule, each rule once, and at most 40 when the group has a block.

The editor's check sends both scores and the word count. The posts list sends both scores on the rows of the page it returns, as it already does with the finding counts, because a full report compares a post with every other post and costs too much for every post at once.

The admin shows each score as a light: green from 80, amber from 50, red below.

## Consequences

- The one score stays as it is, so nothing that reads it changes.
- A readability rule added later, such as those Yoast package WP1 brings, counts towards SEO until its ID joins `READABILITY_RULES`.
- The readability value beside the light is the readability score. The LIX number behind `hard-to-read` is private to the linter; it can be shown once the linter makes it public.

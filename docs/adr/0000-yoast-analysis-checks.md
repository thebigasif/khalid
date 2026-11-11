# 0000. The Yoast analysis checks: tips, word forms, prose and two scores

**Number:** next free. **Status:** Proposed, 2026-11-02. Phase 1, Yoast package WP1 (KH-LNT-2, KH-LNT-4).

## Context

The founder asked that Khalid has every feature of the Yoast SEO plugin, free and Premium, where it applies. The comparison of 2 November 2026 found Khalid's linter missing most of Yoast's analysis: keyphrase synonyms and word forms, the related-keyphrase checks, keyphrase length, distribution and subheadings, the SEO title width in pixels, the image and outbound link checks, cornerstone content, and nearly all of the readability analysis with its own score. The brief asks for all of it, without breaking existing sites, in the lint crate, which must stay small and free of the other Khalid crates (ADR 0011).

## Decision

1. **A fourth level, `info`.** Yoast shows some results as tips. A finding at `info` costs no points and never blocks; `lint.toml` accepts `"info"` like the other levels. The secondary keyword checks, the title start check and the inclusive language check start there; every other new rule starts at `warn`. None blocks.
2. **Word forms through Snowball stems.** `rust-stemmers` (MIT and BSD-3-Clause) was already in the tree through `khalid-search`, so it adds nothing to the binary. English, German, French, Spanish, Italian, Portuguese and Dutch get stems; other languages compare words as written. The five stemmers after English and German add about 71 KB to a release binary, measured in a scratch crate with the release profile. A word whose first letter, accent folded, differs from the stem's skips the stemmer, which keeps one post's lint inside its 20 ms budget.
3. **Synonyms count as the keyword**, in every place check and in the density, from `keyword_synonyms:` in the header.
4. **Readability reads the writer's own prose.** `LintPost::prose_text` is the plain text without code, quotes, tables, forms, maths and the parts Khalid writes or copies itself (contents list, bookmark cards, file links, pull quotes, the gated-content note). The binary builds it from the rendered HTML. A caller that sends none, such as a `SerpDad` draft, gets `body_text` read instead.
5. **Shares wait for 200 words.** The checks that judge a share of sentences or words (long sentences, linking words, passive voice, complex words) stay quiet under 200 words of prose, and the keyword spread under 300, because one sentence swings a short text too far. The subheading share waits for three subheadings for the same reason.
6. **Two scores.** The existing post score becomes the SEO score and leaves the nine readability rules out; a new readability score counts only them, as Yoast keeps the two apart. Both use the same points. The readability score travels in a new `Analysis` on a one-post report, with the word count, the LIX and up to ten prominent words, so the admin reads it from the lint result it already gets. A whole-site report computes none, so `khalid lint` stays as fast.
7. **Pixels first, characters as the fallback.** `title-too-wide` measures the search title with Arial widths at 20 pixels against 600. `long-title` keeps its character count for a title with characters the table lacks, and stays quiet otherwise, so one long title gives one warning.
8. **The inclusive term list lives in the catalogue**, under the `non-inclusive-term` entry, as `terms = [{ lang, term, instead }]`, so the words and the message are changed in one place.
9. **`paragraph-max-words` defaults to 150**, as in Yoast. A site that set it keeps its value.
10. **The settings stay in `Thresholds`.** The brief asks for a typed settings struct with serde and docs that the SEO settings screen can edit. `khalid_lint::Thresholds` already is that struct, with every limit documented and read from `lint.toml`; it now also serialises with the same keys. Moving it to `khalid-core` would make the content and server crates depend on the linter's numbers for no gain, so it stays where the rules read it.
11. **Pillar posts are cornerstone content.** Khalid's topic group pillar (`cluster: role: pillar`) plays the part of Yoast's cornerstone flag, so the 900-word floor and the six-month freshness check apply to it.

## Consequences

- Existing sites keep publishing: every new rule warns or hints, and a site can switch any of them off by name.
- Sites that relied on the old 225-word paragraph default see new warnings until they set `paragraph-max-words = 225`.
- The post score in the admin rises for posts whose only problems were readability ones; the readability score now shows those.
- The admin filters findings to `block` and `warn` today, so it does not show `info` tips yet; the SEO screens of the Yoast package can add them.
- Passive voice, linking words and complex words cover English and German. A new language needs its word lists in the same shape.

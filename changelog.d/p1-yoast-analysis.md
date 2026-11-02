Added
- **Yoast-style content analysis.** The focus keyword now counts in any word form ("internal link" for "internal links", in English and German), and `keyword_synonyms:` in the header lists other words that count as it. Each secondary keyword gets the title, first paragraph, subheading and density checks as tips. New checks warn when the keyword is too long, is only small words, is spread through only a few parts of the text, sits in too few or too many subheadings, or is already used by an older post, and a tip shows when it sits late in the title. The title is now measured in pixels, as Google cuts it. New checks also warn about a post with no image, a post whose links to other sites all carry `nofollow`, and pillar posts under 900 words or not updated for six months. See `docs/lint-analysis.md`.
- **Readability checks** on your own prose, with code, quotes, tables and Khalid's own text left out: long sentences, long stretches without a subheading, three sentences in a row that start alike, too few linking words, too much passive voice and too many long rare words, in English and German, and a tip with other words for terms that can leave readers out.
- **Two scores.** A post's lint result now carries a readability score apart from the SEO score, with its word count, its reading index and its ten most prominent words.
- A fourth lint level, `info`, for tips that cost no points and never block.

Changed
- `paragraph-max-words` now defaults to 150, as in Yoast. A site that set its own number keeps it.
- The post score is now the SEO score: the readability checks count towards the readability score instead.

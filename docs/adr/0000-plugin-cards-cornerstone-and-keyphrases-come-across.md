# 0000. Plugin share cards, cornerstone marks and keyphrases come across

**Number:** next free. **Status:** Proposed, 2026-11-02. Yoast package WP4.

## Context

The WordPress importer read each SEO plugin's title, description, canonical, robots, focus keywords and primary category, and listed the focus keywords in the report because post front matter had no field for them then. Post front matter now holds `focus_keyword`, up to four `secondary_keywords`, `share_cards` for X and Facebook, and `topic_group` with `main_post`. Yoast, Rank Math and All in One SEO keep more than the importer read: Open Graph and X titles, descriptions and images, a cornerstone or pillar mark, related keyphrases and synonyms, a breadcrumb title and schema page and article types.

## Decision

- **Share cards come across.** Each plugin's Open Graph fields become `share_cards.facebook`, its X fields `share_cards.x`, with the plugin's variables filled in the same way as its titles. A part the plugin left empty stays out, so Khalid falls back to the post's own title, description and lead image.
- **A cornerstone or pillar post becomes a main post.** Yoast's `_yoast_wpseo_is_cornerstone` and Rank Math's `rank_math_pillar_content` write `main_post: true` with `topic_group` set to the post's category slug, or its own slug when it has none, so the linter's cluster rules start from it.
- **Keywords are written, not only reported.** The first focus keyword becomes `focus_keyword`. The rest, then Yoast Premium's related keyphrases and synonyms and All in One SEO's additional keyphrases, become `secondary_keywords`, in that order, without repeats. Beyond the four a post holds, the rest stay in the report as focus keywords.
- **The breadcrumb title and schema types are listed, not guessed.** Khalid has no breadcrumb title and chooses a post's schema type from its blocks, so those fields appear in the report as fields not carried, with their values.
- **Pages are unchanged:** their front matter holds none of these fields, so their keywords stay in the report as before.

## Consequences

- An imported post keeps the work the writer put into its social cards and keyphrases, and its cornerstone posts are marked where Khalid looks for them.
- A site that marked a whole category's posts as cornerstone gets several main posts in one topic group; the linter's cluster rules then say which to keep.

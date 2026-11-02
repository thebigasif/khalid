# 0000. The demos live on serpdad.com, and are never indexed

**Number:** next free. **Status:** Proposed, 2026-11-02. Founder request: all demo traffic and authority on serpdad.com.

## Context

The showcase, the eleven template demos and the six use case examples were published only on GitHub Pages at thebigasif.github.io/khalid/. The founder wants them on serpdad.com, under SerpDad's product page at https://serpdad.com/khalid/, which another repository builds. The demos are made-up blogs: a search result for one sends a reader to a fictional studio, and eighteen near copies of the showcase look like duplicate content. Khalid had a robots setting per archive type and per post, but none for a whole site.

## Decision

- **A whole-site setting.** `[seo.index] site = "noindex"` makes every page `noindex, follow`, over the setting per type and a post's own `robots`, and so empties every sitemap, which already lists only indexable pages. A post's own `nofollow` stays. `robots.txt` is unchanged, so crawlers can still read the tag.
- **Every demo carries it, in both copies.** `scripts/pages.sh` adds it to each demo, template and example it builds. The website's own pages, the template gallery included, stay indexable.
- **One archive for serpdad.com.** `scripts/demos-bundle.sh` builds the demos for https://serpdad.com at `/khalid/`, plus the gallery page alone, and packs `demo/`, `templates/` and `examples/` as `khalid-demos.tar.gz`. The gallery's menu is cut to the product page, Templates and the demo, any other link to the website goes to `/khalid/`, and its stylesheet and script move to `templates/assets/`, so the archive holds nothing outside those three folders. The Pages workflow publishes it at /khalid/khalid-demos.tar.gz.
- **GitHub Pages credits serpdad.com.** `pages.sh --canonical https://serpdad.com` rewrites each page's canonical and language links to the same path on serpdad.com after the build. Links between pages are paths, so the Pages copy keeps working for old links.

## Consequences

- The website's pages that serpdad.com does not serve (features, use cases, start, compare, about) name serpdad.com addresses as canonical too. Until serpdad.com serves or redirects them, search engines will likely ignore those canonicals and keep the Pages copy.
- A demo's logo is linked by full address, so each copy loads it from its own host: the archive's logos work only once it is deployed at serpdad.com.
- The gallery pictures in the archive are the screenshots the Pages build draws; built elsewhere without Chromium they are plain cards.

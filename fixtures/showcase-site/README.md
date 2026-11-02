# Showcase site

The public demo: the blog of Larkspur Studio, a fictional content studio, written to show everything Khalid can do at the depth of a large SEO blog. The brand, its people, its benchmark numbers and its sample data are invented. Every other fact links to its source.

`fixtures/demo-site` stays the small, stable site that benchmarks, snapshots and most tests use. This folder is for people to look at, and `crates/khalid/tests/showcase.rs` keeps it honest: every post passes `khalid lint` with no finding, the built site passes `khalid audit --offline` with no warning, the posts use every block listed in the test, and the demo keeps at least 30 English posts, 5 German posts, 20 glossary terms and 4 authors.

The page `/blog/feature-tour/` lists every feature with a link to the page that shows it. Start there.

| What | Where |
|---|---|
| Settings: menus, announcement, publisher, AI info page | `site.toml` |
| Five categories, named in both languages | `categories.toml` |
| A news collection at `/blog/news/`, marked as news | `routes.toml` |
| 32 English posts: six flagship guides, a topic hub per category, a review, a comparison, a best-of list, a fact check, a dataset, a course, a podcast episode, a video and one members-only post | `content/en/posts/` |
| A news post in each language | `content/<language>/news/` |
| Five German posts, each a translation of an English one | `content/de/posts/` |
| Four authors with portraits, roles, credentials and profile links, in both languages | `content/<language>/authors/` |
| About, contact, editorial policy, tools and the feature tour | `content/<language>/pages/` |
| 27 glossary terms, reached by wikilinks | `content/<language>/glossary/` |
| A snippet placed in two posts | `content/en/snippets/` |
| Illustrations: small SVG files made for this site | beside each post's `index.md` |
| A 12 second silent animation and a 4 second chime, made for this site | `crawl-index-rank.mp4` in the audit guide, `episode-1-sample.wav` in the podcast post |

`lint.toml` turns off one rule, `missing-translation`, because the site translates only some posts on purpose.

One block is not shown: recipes, because recipe markup on a page that is not a recipe breaks the search engines' rules.

Types with no block of their own, ClaimReview, Course, Dataset, ItemList and PodcastEpisode, are written by hand in the post's `schema` field.

To look at it:

```sh
cargo run -p khalid -- --site fixtures/showcase-site build
```

The pages are in `fixtures/showcase-site/public/`. Delete `public/` and `data/` afterwards, so they stay out of the fixture.

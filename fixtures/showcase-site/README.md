# Showcase site

The public demo: the blog of Larkspur Studio, a fictional content studio, written to show everything Khalid can do. The brand, its people and its benchmark numbers are invented. Every other fact links to its source.

`fixtures/demo-site` stays the small, stable site that benchmarks, snapshots and most tests use. This folder is for people to look at, and `crates/khalid/tests/showcase.rs` keeps it honest: every post passes `khalid lint` with no finding, the built site passes `khalid audit --offline` with no warning, and the posts use every block listed in the test.

| What | Where |
|---|---|
| Settings: menus, announcement, publisher, AI info page | `site.toml` |
| Four categories, named in both languages | `categories.toml` |
| Thirteen English posts: a pillar guide with its cluster, strategy, technical SEO, AI search and one members-only post | `content/en/posts/` |
| Three German posts, each a translation of an English one | `content/de/posts/` |
| Three authors with roles and credentials, in both languages | `content/<language>/authors/` |
| About pages, with the business card and the contact form | `content/<language>/pages/` |
| Glossary terms, reached by wikilinks | `content/<language>/glossary/` |
| A snippet placed in the benchmark post | `content/en/snippets/` |
| Illustrations: small SVG files made for this site | beside each post's `index.md` |

`lint.toml` turns off one rule, `missing-translation`, because the site translates three posts on purpose.

Three blocks are not shown: audio and video files, because the fixture keeps no sound or film files, and recipes, because recipe markup on a page that is not a recipe breaks the search engines' rules. The video embed shows a public talk instead.

To look at it:

```sh
cargo run -p khalid -- --site fixtures/showcase-site build
```

The pages are in `fixtures/showcase-site/public/`. Delete `public/` and `data/` afterwards, so they stay out of the fixture.

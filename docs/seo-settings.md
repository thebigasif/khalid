# SEO settings

Every SEO setting lives in `site.toml`, under `[seo]`. Each one has a default, so a site works with none of them set. The admin's SEO settings screen edits the same values. This page explains each setting added for the Yoast comparison of 2 November 2026; the Rust docs of `khalid_core::SeoSettings` cover the rest.

## Titles and descriptions

`[seo.titles]` holds a title and a description template per page type, such as `[seo.titles.tag]`. A template is text with variables in braces. A variable with no value on a page writes nothing, and a separator left alone at either end is dropped.

| Variable | What it writes |
|---|---|
| `{title}` | The page's title. |
| `{site}` | The site's name. |
| `{tagline}` | The tagline, from `[seo.titles] tagline`. |
| `{sep}` | The separator, `\|` unless `[seo.titles] separator` says otherwise. |
| `{excerpt}` | The page's summary: a post's `description`. |
| `{category}` | A post's category name. |
| `{author}` | A post's author name. |
| `{date}` | The day a post was published, such as `14 September 2026`. |
| `{modified}` | The day a post was last really updated, or published when it never was. |
| `{page}` | On page 2 and after of a long list, `Page 2 of 5`. Nothing on page 1. |
| `{term}` | On a tag or category page, the tag or category name. |
| `{term_description}` | On a tag or category page, its description from `categories.toml` or `tags.toml`. |
| `{searchphrase}` | On a search results page, what the reader searched for. |
| `{currentyear}` | The year the page was made, such as `2026`. Handy for "best tools of 2026". |
| `{currentmonth}` | The month the page was made, such as `November`. |
| `{tag}` | A post's first tag. |
| `{focuskw}` | A post's `focus_keyword`. |

Dates, month names and the page line are written in the page's language, with the same words the templates use.

```toml
[seo.titles.post]
title = "{title} {sep} {site}"
description = "{excerpt}"

[seo.titles.tag]
title = "{term} {page} {sep} {site}"
description = "{term_description}"
```

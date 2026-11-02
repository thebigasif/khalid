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

## Show in search results

`[seo.index]` says which pages search engines may list. Each value is `"index"` or `"noindex"`. A post's own `robots` field always wins.

| Key | Default | What it covers |
|---|---|---|
| `posts` | `index` | Every post. |
| `pages` | `index` | Every standalone page, such as About. |
| `collections.<name>` | as `posts` | The posts of one collection, such as `news`. |
| `tags`, `categories` | `index` | Tag and category pages. |
| `authors` | `index` | Author pages. `"off"` builds none. |
| `dates` | `noindex` | Year and month pages. `"off"` builds none. |
| `search` | `noindex` | Search results. |
| `thin_archives_below` | `3` | A tag or author page with fewer posts is `noindex`. |
| `site` | `index` | `"noindex"` keeps the whole site out, for a staging copy. |

An archive set to `"off"` has no pages and no sitemap entries, and each of its addresses sends readers to the home page with a permanent redirect. Use it on a blog with one author, where the author page only repeats the home page.

```toml
[seo.index]
pages = "noindex"
authors = "off"
dates = "off"

[seo.index.collections]
news = "noindex"
```

## Feeds

`[seo.feeds] off` lists the feed formats Khalid does not write: `"rss"` (`feed.xml`), `"atom"` (`atom.xml`) and `"json"` (`feed.json`). Every format is written by default.

```toml
[seo.feeds]
off = ["atom", "json"]
```

## Breadcrumbs

`[seo.breadcrumbs]` shapes the trail above each page, when the template shows it, and the `BreadcrumbList` search engines read.

| Key | Default | What it does |
|---|---|---|
| `separator` | `/` | What stands between two steps. Screen readers skip it. |
| `home` | the site name | The first step's name, such as `Home`. |
| `archive_prefix` | none | Words before the last step on a tag, category, author or date page, such as `Archives for`. |
| `search_prefix` | none | Words before the last step on the search page, such as `You searched for`. |
| `bold_last` | `false` | Shows the last step, the page itself, in bold. |
| `taxonomy` | `category` | Which term a post's trail passes through: `category`, `tag` (its first tag) or `none`. |

A post may set `breadcrumb_title:` in its front matter to name itself more briefly in the trail, such as `Sitemaps` for a long headline.

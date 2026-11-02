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

## Link previews on X and Slack

A post's preview on X and Slack shows two short lines under its title: who wrote it and how long it takes to read, in the page's language. This is Yoast's "enhanced Slack sharing". A site that hides reading time (`[reader] reading_time = false`) hides it here too. To turn both lines off:

```toml
[seo.social]
share_labels = false
```

## robots.txt

`[seo.crawlers]` shapes `robots.txt`.

| Key | Default | What it does |
|---|---|---|
| `disallow` | none | Paths no crawler may read, such as `/preview/`. |
| `allow`, `block` | none | Crawlers allowed or blocked by name, such as `GPTBot`. |
| `disallow_search` | `true` | Keeps crawlers off the search page, so they do not spend their visits on endless result pages. |
| `extra_lines` | none | Lines added as written, after the rules and before the sitemap, such as `Crawl-delay: 5`. Each must be one line. |

## llms.txt

`llms.txt` is a short guide to the site for language models. `[seo.llms_txt]` shapes it.

| Key | Default | What it does |
|---|---|---|
| `off` | `false` | `true` stops writing the file. |
| `intro` | none | Text after the tagline, such as who writes the site and how to cite it. |
| `pages` | none | Paths listed first, under "Key pages", in the order given, such as `["/about/"]`. |
| `exclude` | none | Paths left out, with every page below them, such as `["/tag/"]`. |

## Whole-site schema

Each language's pages that search engines may list have one JSON-LD graph together, at `schema.json` in the language's folder (`/blog/schema.json`, `/blog/de/schema.json`). Each node with an `@id` appears once. `robots.txt` names every file on a `Schemamap:` line, so an AI agent or a search engine reads all of the site's structured data in one request instead of one page at a time. This is Yoast's schema aggregation. To turn it off:

```toml
[seo.schema_map]
off = true
```

## Front matter for search

These fields go in a post's front matter, between the `---` lines. `page_type` also works on a page in `pages/`.

| Field | What it does |
|---|---|
| `breadcrumb_title` | A shorter name for the post in its breadcrumb trail. |
| `page_type` | The kind of web page in structured data: `WebPage` (the default), `FAQPage`, `ContactPage`, `AboutPage`, `QAPage`, `ItemPage` or `CollectionPage`. |
| `article_type` | The kind of article: `BlogPosting` (the default), `Article`, `NewsArticle`, `TechArticle` or `Report`. It wins over the collection's default, so a report in a news collection stays a report. |
| `news` | `false` leaves a post of a news collection out of the news sitemap. |
| `stock_tickers` | Up to five stock tickers a news post is about, such as `["NASDAQ:GOOG"]`, for Google News. |

```yaml
---
title: Our 2026 survey of 500 SEO teams
article_type: Report
stock_tickers: ["NASDAQ:GOOG"]
---
```

The video block takes an optional `thumbnail`, the picture search results show for the video when it should not be the poster:

```markdown
::video{src="/media/tour.mp4" title="A tour" poster="/media/tour.jpg" thumbnail="/media/tour-thumb.jpg"}
```

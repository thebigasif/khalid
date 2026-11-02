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

## Search filters

Bots fill a site's search page with long or spam queries, so the page shows their words and links. `[seo.search_filter]` refuses such searches before anything is looked up, as Yoast's crawl settings do. A refused search answers 400 (a bad request) with the empty search page, and the query is not shown back. Every filter is on.

| Key | Default | What it does |
|---|---|---|
| `max_length` | `50` | The most characters a search may have. `0` sets no limit. |
| `symbols` | `true` | Refuses a search with an emoji or another symbol that is not a letter, a digit or punctuation. |
| `spam` | `true` | Refuses a search with a link, markup, a spam domain such as `.xyz`, or one character six times in a row. |

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

## Local business

A shop, office or clinic lists each of its locations in a `[[seo.locations]]` table. Each one becomes a schema.org node on the home page, so search engines can show its address and hours.

```toml
[[seo.locations]]
type = "Dentist"
name = "Mill Lane Dental"
street = "12 Mill Lane"
locality = "Leeds"
postal_code = "LS1 4AB"
country = "GB"
telephone = "+44 113 496 0000"
map = "https://maps.example.com/mill-lane"
geo = { latitude = 53.7997, longitude = -1.5492 }

[[seo.locations.hours]]
days = ["mon", "tue", "wed", "thu", "fri"]
opens = "09:00"
closes = "17:30"
```

| Key | What it does |
|---|---|
| `type` | The schema.org type, such as `Dentist`. `LocalBusiness` when left out. |
| `name`, `street`, `locality`, `postal_code`, `country` | The name and address. The country is two capital letters, such as `GB`. |
| `region`, `telephone`, `price_range`, `map` | Optional: the county or state, the phone number, how expensive it is, a link to a map. |
| `geo` | Optional: the point on the map. It puts the location in `locations.kml`. |
| `hours` | When it is open, one table per group of days, on a 24-hour clock. |

A post or page shows one location's card with the location block, by its name as written in `site.toml`:

```markdown
::location{name="Mill Lane Dental"}
```

The card shows the address, the phone number as a link, the opening hours day by day and a link to `map`, or to OpenStreetMap at the `geo` point. With scripts on, it also says "Open now" or "Closed now" by the reader's clock. A name `site.toml` does not list shows nothing, and `khalid build` warns about it.

A site with locations also gets a locations page at `/locations/` in each language. It lists every location's card in the order of `site.toml`, so it works with scripts off. When two or more locations have a `geo` point and the browser can tell where the reader is, a button sorts the list nearest first. The page is in the sitemap. A page of your own at `/locations/` takes its place.

`[seo.local]` switches what a site with locations gets. Both are on by default.

| Key | Default | What it does |
|---|---|---|
| `kml` | `true` | Writes `locations.kml`, a geo sitemap with a pin for each location that has a `geo` point, and lists it in the sitemap index, as Yoast Local does. |
| `page` | `true` | Builds the locations page at `/locations/`. |

## Category and tag pages

Each category or tag page can have its own search settings, one table per language under `seo`. Categories live in `categories.toml` and tags in `tags.toml`, both in the site folder. A tag needs an entry in `tags.toml` only when it has settings: its name still comes from the posts.

```toml
[[tags]]
slug = "crawl-budget"

[tags.seo.en]
title = "Crawl budget guides {sep} {site}"
description = "How search engines decide how much of a site to read."
intro = "Every guide on **crawl budget**, newest first."
robots = "noindex"

[tags.seo.en.image]
url = "https://www.example.com/media/crawl.png"
alt = "A spider on a sitemap"
```

| Key | What it does |
|---|---|
| `title` | The page's title, with the same variables as `[seo.titles]`. |
| `description` | The meta description. Titles can use it as `{term_description}`. |
| `robots` | The page's own robots directives, such as `noindex`. |
| `intro` | Markdown shown above the posts on the first page of the list. |
| `image` | The picture link previews show, with `url`, `alt` and optional `width` and `height`. |

A term page with no intro shows search engines only a list of links. `khalid build` names such pages in one note, and still builds.

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

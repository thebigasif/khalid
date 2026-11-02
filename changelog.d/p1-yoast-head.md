Added
- **Ten more title and description variables**, as Yoast has: `{date}`, `{modified}`, `{page}`, `{term}`, `{term_description}`, `{searchphrase}`, `{currentyear}`, `{currentmonth}`, `{tag}` and `{focuskw}`. Dates and month names are written in the page's language. Every variable is listed in `docs/seo-settings.md`.
- **Show in search results per content type**: `[seo.index] posts`, `pages` and `collections.<name>`. **Author and date archives can be switched off** with `authors = "off"` and `dates = "off"`: their pages are not built and their addresses redirect to the home page.
- **Feeds per format**: `[seo.feeds] off = ["atom", "json"]` stops writing those feeds.
- **Breadcrumb settings** in `[seo.breadcrumbs]`: the separator, the home step's name, a prefix for archive and search pages, a bold last step, and which term a post's trail passes through. A post can set a shorter `breadcrumb_title:`. The trail and its `BreadcrumbList` agree.
- **Enhanced X and Slack previews**: a post's preview names its author and reading time (`twitter:label1` to `data2`). `[seo.social] share_labels = false` turns them off.
- **robots.txt**: the search page is kept from crawlers by default (`[seo.crawlers] disallow_search`), and `extra_lines` adds lines as written.
- **llms.txt options** in `[seo.llms_txt]`: an intro, key pages listed first, paths to leave out, and `off`.
- **Schema aggregation**: the whole site's structured data in one file per language, at `schema.json`, named in `robots.txt` on a `Schemamap:` line. `[seo.schema_map] off = true` turns it off.

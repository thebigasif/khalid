Added
- **Ten more title and description variables**, as Yoast has: `{date}`, `{modified}`, `{page}`, `{term}`, `{term_description}`, `{searchphrase}`, `{currentyear}`, `{currentmonth}`, `{tag}` and `{focuskw}`. Dates and month names are written in the page's language. Every variable is listed in `docs/seo-settings.md`.
- **Show in search results per content type**: `[seo.index] posts`, `pages` and `collections.<name>`. **Author and date archives can be switched off** with `authors = "off"` and `dates = "off"`: their pages are not built and their addresses redirect to the home page.
- **Feeds per format**: `[seo.feeds] off = ["atom", "json"]` stops writing those feeds.

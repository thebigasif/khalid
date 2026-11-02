Added
- The settings API reads and saves the `[seo]` table of `site.toml` one part at a time, such as `titles` or `crawlers`, keeping every other line and comment of the file. `docs/seo-settings.md` lists every part and its defaults.
- An SEO settings screen in the admin, under Site. Search appearance sets the separator, the tagline, and the title and description templates of each kind of page, with buttons that add each variable and a live search result.

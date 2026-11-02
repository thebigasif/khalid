# SEO settings

Every SEO setting of a site lives in the `[seo]` table of `site.toml`. The
admin's **SEO settings** screen edits them through the settings API
(`GET` and `PUT /api/admin/v1/settings/`), which reads the table as written
and changes it one part at a time. A part the screen saves replaces that
part whole; any other table and comment in the file stays as it was. Every
change is checked as `khalid serve` checks the file at start, and a change
the site could not start with saves nothing. The running server reads
`site.toml` at start, so a saved change applies after a restart.

A setting left out keeps its default, so a site with no `[seo]` table at all
gets every default below.

| Part | What it sets | Default | Admin section |
| --- | --- | --- | --- |
| `[seo.titles]` | `separator`, `tagline`, and a `title` and `description` template per page type: `home`, `post`, `page`, `author`, `tag`, `category`, `collection`, `date`, `search`, `glossary`, `glossary_term`, `not_found` | separator `\|`, `{title} {sep} {site}` with `{excerpt}`; the home page `{site} {sep} {tagline}` | Search appearance |
| `[seo.index]` | `index` or `noindex` for `tags`, `categories`, `authors`, `dates`, `search` and the whole `site`; `thin_archives_below` | tags, categories and authors indexed; dates and search not; thin below 3 | Show in search |
| `[seo.publisher]` | `type` (`organization` or `person`), `name`, `logo`, `same_as` (social profiles), `email`, `telephone`, `founded` | an organisation named after the site | Site representation |
| `[seo.social]` | `twitter` (the X handle) and `image`, the default share image | none | Social sharing |
| `[seo.verification]` | `google`, `bing`, `yandex`, `baidu`, `pinterest` codes, and `files` | none | Webmaster tools |
| `[seo.crawlers]` | `disallow` paths, `allow` and `block` crawler names | every crawler reads every page | Crawlers and AI bots |
| `[seo.feeds]` | `content` (`full` or `excerpt`), `before_item`, `after_item`, `source_line`, `limit` | full posts, 20 items | Feeds |
| `[seo.indexnow]` | `key` and `endpoint` | off | IndexNow |
| `[seo.ai_page]` | `off`, `contact`, `covers`, `cite` | on | AI info page |
| `[seo.links]` | `affiliate_domains` and the other outbound link rules | none | Not on the screen yet |
| `[[seo.locations]]` | the site's shops or offices, for local search | none | Not on the screen yet |
| `[[seo.schema]]` | schema nodes per category or collection | none | Schema templates screen |

The template variables are `{title}`, `{site}`, `{tagline}`, `{sep}`,
`{excerpt}`, `{category}` and `{author}`.

The AI bot toggles on the screen write the crawler's name into
`[seo.crawlers] block`. The known AI crawlers are `GPTBot`, `ChatGPT-User`,
`OAI-SearchBot`, `ClaudeBot`, `Claude-Web`, `anthropic-ai`, `PerplexityBot`,
`Google-Extended`, `Applebot-Extended`, `CCBot`, `Bytespider`,
`meta-externalagent` and `Amazonbot`.

Breadcrumbs and `llms.txt` are added by the Yoast package WP2. Their parts
join this table, and the screen, once that package lands.

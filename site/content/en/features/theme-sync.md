---
title: "Theme sync and languages: a blog that matches your site"
slug: theme-sync
date: 2026-10-21
author: khalid-team
description: Theme sync copies your main website's colours, fonts and logo into your blog. Pick a template, and publish in more than one language.
focus_keyword: theme sync
---

Your blog should look like the rest of your website. Theme sync copies your site's colours, fonts and logo, and Khalid can publish the same blog in several languages.

## What theme sync does for you

- **Match your brand in one step.** Theme sync reads your main website and copies its colours, fonts and logo into the blog.
- **Stay readable.** If a colour would be too hard to read, theme sync picks a stronger one that passes the contrast rules for accessible text.
- **Keep what you chose.** Lock any colour or font so the next sync leaves it alone.
- **Pick a layout.** A template sets where things sit on the page. Your colours and fonts carry over to every template.
- **Write in more than one language.** Each language has its own folder. Khalid links each post to its translations, so search engines show readers the right one.

## How it works

Run this inside your blog's folder, with your main website's address:

```sh
khalid theme sync https://www.example.com
```

The colours and fonts land in `theme/tokens.toml`, a short file you can also edit by hand. To change one kind of page entirely, put your own version of its template in `theme/templates/`. This website does that for its home page. The [templates guide](https://github.com/thebigasif/khalid/blob/main/docs/templates.md) explains both.

You can also run theme sync again whenever your main site changes. It also writes a preview, a sample post in the new colours, so you can look before you build the blog. Nothing you locked moves.

Languages are listed in `site.toml`, such as `languages = ["en", "de"]`. Each post names its translations, and Khalid adds the links between them that search engines look for. Search inside the blog understands English and German words best today.

## See it live

- [The demo blog in German](/khalid/demo/de/) shows a second language, with links between the versions.
- [The templates gallery](/khalid/templates/) shows each layout.
- The [multilingual example site](/khalid/examples/multilingual/) is a whole blog in several languages.

Next: back to [the writing checks](/khalid/features/writing-checks/). Also see [the blog editor](/khalid/features/blog-editor/).

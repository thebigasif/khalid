---
title: "SEO and AI search, built in"
slug: seo-ai-search
date: 2026-10-31
author: khalid-team
description: Khalid writes the tags, structured data, sitemaps and feeds that search engines and AI search tools read, with no plugin to install.
focus_keyword: AI search
---

Search engines and AI search tools read more than your words. They read tags, structured data and sitemaps. Khalid writes all of these on every page, with no plugin to install.

## What it does for search engines and AI search

- **Search engines understand each page.** Every page gets a title, a description, a canonical address and structured data. Structured data is a short summary in a format search engines read, saying for example "this is an article by this author".
- **New posts are found quickly.** Khalid writes an XML sitemap, a news sitemap when you ask for one, and RSS, Atom and JSON feeds.
- **AI search tools know what you cover.** Khalid writes an `llms.txt` file and a page for AI tools that says what you write about and how to credit you.
- **Moved pages keep their visitors.** When you change a post's address, Khalid sends readers from the old address to the new one.
- **You can check the whole site.** The audit crawls your blog the way SEO tools do and lists what to fix, most important first.

## How you set it up

Most of it needs nothing from you. Khalid reads the title, description and author from each post. A few settings in `site.toml` add more:

- `[seo.publisher]` names your company, or you.
- `[seo.ai_page]` fills in what that AI info page says.
- `[seo.crawlers]` lets in or keeps out named crawlers, such as AI training bots.
- `[seo.indexnow]` tells Bing and other search engines about a new post the moment it goes live.

Blocks in a post add their own structured data: a FAQ, how-to steps, a review, a product or a video. To check the whole site, run this inside your blog's folder:

```sh
khalid audit
```

If you use Google Search Console or Bing Webmaster Tools, Khalid can bring their reports to your command line too. The [deploy guide](https://github.com/thebigasif/khalid/blob/main/docs/deploy.md) shows how to connect them.

## See it live

- [Structured data for blog posts](/khalid/demo/structured-data/) uses FAQ and how-to blocks. View the page source to see the data.
- [The demo's AI info page](/khalid/demo/ai/) says what the blog covers and how to cite it.
- [The demo's site map](/khalid/demo/sitemap/) lists every page, written by Khalid.
- This website is built with Khalid as well, so it has [one of its own](/khalid/ai/).

Next: [the blog editor in your browser](/khalid/features/blog-editor/). Also see [the newsletter](/khalid/features/blog-newsletter/).

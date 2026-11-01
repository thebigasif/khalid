---
title: "Blog migration: bring your old blog with you"
slug: blog-migration
date: 2026-10-25
author: khalid-team
description: Blog migration from WordPress, Ghost, Medium, Substack or markdown files, keeping your addresses, your SEO settings and your redirects.
focus_keyword: blog migration
---

A blog migration is scary because of what can break: links, rankings and old addresses. Khalid's importer keeps them, and it tells you what it will do before it writes anything.

## What blog migration with Khalid keeps

- **Your posts, from where they are now.** Khalid imports from WordPress, Ghost, Medium, Substack and folders of markdown files. It can also read a live WordPress site from its address.
- **Your addresses.** Each post keeps its slug, the short name at the end of its address.
- **Your search rankings.** When an address does change, Khalid stores a redirect, so old links and search results still reach the post.
- **Your SEO work.** From WordPress, Khalid reads the titles and descriptions set in Yoast, Rank Math, All in One SEO and SEOPress, and the redirects from the Redirection plugin.
- **Your peace of mind.** A dry run lists what would come in and what would stay behind, and writes nothing.

## How it works

Export your old blog from its own admin, then run the import with `--dry-run` first, inside your new blog's folder:

```sh
khalid import wordpress export.xml --dry-run
khalid import wordpress export.xml
```

Imported posts keep their dates. Each one goes live at the next build once it passes the writing checks, and the build tells you which ones still need work.
Moving from another tool works the same way: name it in place of `wordpress`, such as `ghost`, `medium`, `substack` or `markdown`, and point at its export.

Want to try it before you move? Use the [sample WordPress export on GitHub](https://github.com/thebigasif/khalid/blob/main/crates/khalid-import/fixtures/wordpress/basic.xml).

## See it live

An import happens on your computer, so there is nothing to click here. The [getting started guide](https://github.com/thebigasif/khalid/blob/main/docs/getting-started.md) walks through a blog migration with the sample export, and a test runs every command in that guide, so it works as written.

Next: [the blog API, SDK and MCP](/khalid/features/blog-api/). Also see [backups and safety](/khalid/features/blog-backups/).

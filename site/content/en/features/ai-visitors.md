---
title: "Analytics and AI visitors: who reads and who quotes you"
slug: ai-visitors
date: 2026-10-26
author: khalid-team
description: Count page views without cookies, see AI visitors and the AI tools that send you readers, and find posts AI reads but never cites.
focus_keyword: AI visitors
---

You want to know which posts work, and more of your readers now arrive through AI tools. Khalid counts visits on your own server, without cookies, and shows your AI visitors: the AI tools that read your blog and the people they send to it.

## What it shows about AI visitors and readers

- **Page views without cookies.** Khalid counts visits on the server, so there is no tracking script and no cookie banner for analytics.
- **Readers sent by AI tools.** Visitors who arrive from ChatGPT and other AI assistants are named as such.
- **AI crawlers at work.** Khalid can keep a log of the bots that read your pages. A bot only counts as, for example, Google or OpenAI when its address is on that company's published list, so fakes are not counted.
- **Posts AI reads but never cites.** A report lists the posts AI crawlers fetch that never send you a reader, so you know which ones to improve.
- **A warning when something breaks.** Alerts fire when a crawler stops visiting or starts hitting errors. A watch checks that your blog is up and fast.

## How it works

Page views are counted as soon as Khalid runs as a server. The log of crawlers and AI visitors is off by default. Switch it on in `site.toml`:

```toml
[visitor_log]
on = true
```

Then ask for the reports on the command line, inside your blog's folder, or open them in the admin:

```sh
khalid analytics ai
khalid analytics uncited
khalid crawl health
```

A blog published as plain files has no server to count visits, so these reports need server mode. The [deploy guide](https://github.com/thebigasif/khalid/blob/main/docs/deploy.md) explains the visitor log and how long it keeps each record.

## See it live

The demo is plain files, so it has no visit counts. The [SaaS example site](/khalid/examples/saas/) explains how a product team reads these reports. To see your own numbers, [start a blog](/khalid/start/) in server mode.

Next: [blog migration from another tool](/khalid/features/blog-migration/). Also see [hosting and speed](/khalid/features/blog-hosting/).

---
title: "Publish and schedule posts when they are ready"
slug: schedule-posts
date: 2026-10-29
author: khalid-team
description: Keep drafts private, share a preview, publish only what passes the checks, and schedule posts to go live on the day you choose.
focus_keyword: schedule posts
---

A post should go live when it is ready, not before. Khalid keeps drafts private, checks each post as it is published, and lets you schedule posts for the day you pick.

## What it does when you schedule posts

- **Drafts stay private.** Readers never see a draft, and search engines never find one.
- **Get a second opinion first.** A preview link lets someone read a draft without an account. The link stops working after a few days.
- **Only good posts go live.** Publishing runs the writing checks first. If a check blocks the post, Khalid says why and publishes nothing.
- **Publish on the right day.** Set a date and time, and Khalid publishes the post then. You can also set a post to come down on a date, for an offer that ends.
- **Tell search engines at once.** When a post goes live, Khalid can tell search engines that support IndexNow, and your webhooks.

## How it works

On the command line, inside your blog's folder, a new draft and its publishing look like this:

```sh
khalid new --draft "Why internal links matter"
khalid publish why-internal-links-matter
```

The first command writes the draft into the `drafts` folder. The second checks it and moves it into the live posts with today's date. In the admin, the same steps are buttons, and you can schedule posts from there too.

Scheduled posts need Khalid running as a server, because something has to be awake at the time you chose. A blog published as plain files goes live when you build it.

If your team keeps the blog in git, the tool that tracks changes to files, Khalid can write drafts on a branch and rebuild the blog when you push. The [deploy guide](https://github.com/thebigasif/khalid/blob/main/docs/deploy.md) explains that setup.

## See it live

- [The content refresh playbook](/khalid/demo/content-refresh/) shows how an updated post tells readers what changed.
- [The demo's RSS feed](/khalid/demo/feed.xml) lists each post with the date it went live.

Next: [the blog newsletter](/khalid/features/blog-newsletter/). Also see [analytics and AI visitors](/khalid/features/ai-visitors/).

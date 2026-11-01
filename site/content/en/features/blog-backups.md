---
title: "Blog backups and safety: easy to save, hard to break"
slug: blog-backups
date: 2026-10-22
author: khalid-team
description: Blog backups in one file, a safe restore, undo for mistakes, and a record of who changed what.
focus_keyword: blog backups
---

Your blog is years of work. Khalid makes blog backups easy and mistakes hard to make. Your posts are text files you can copy anywhere, and one command saves everything else.

## What blog backups and safety give you

- **One file holds your whole blog.** A backup is one archive with your posts, settings and a full copy of the database.
- **Back up while the blog runs.** You do not need to stop anything.
- **Restore without mixing things up.** A restore only fills a new or empty folder, so it never mixes an old copy into a live blog.
- **Undo mistakes.** Deleted posts go to the trash first. Earlier versions of each post are kept. An update to Khalid can be rolled back.
- **See who did what.** An activity log records who changed what in the admin.
- **Safe by default.** Passwords are stored in a scrambled form that cannot be turned back. Khalid never fetches addresses on your private network for anyone. Visitors are asked before any tracking code you add runs.

## How it works

Run these inside the folder that holds your blog:

```sh
khalid backup --out blog.tar.gz
khalid restore blog.tar.gz blog-restored
```

The first command writes the archive. The second makes the blog again in a new folder, database included. The backup leaves out the built pages and the resized pictures, because Khalid makes those again.

Because posts are plain text files, you can also keep the whole site folder in git and see every change line by line.

To put a backup in place of a live blog, stop Khalid, move the old folder aside, restore into the old folder's name and start Khalid again. Delete the old copy only once the blog looks right. Run a restore into a spare folder now and then, so you know your blog backups work before the day you need one.

## See it live

Blog backups happen on your own computer, so there is nothing to click here. The [getting started guide](https://github.com/thebigasif/khalid/blob/main/docs/getting-started.md) shows the full steps to swap a live blog for a backup.

Next: [theme sync and languages](/khalid/features/theme-sync/). Also see [SEO and AI search](/khalid/features/seo-ai-search/).

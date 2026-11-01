---
title: "Blog hosting and speed: one small program, fast pages"
slug: blog-hosting
date: 2026-10-23
author: khalid-team
description: Blog hosting your way. Run Khalid as one program on a small server, in Docker or as plain files, with fast pages that need no script.
focus_keyword: blog hosting
---

Blog hosting should be simple and cheap, and pages should be fast. Khalid is one program with no database server to look after, and its pages need no script to read.

## What it does for blog hosting

- **Nothing else to install.** Khalid is one file. It keeps its data in a database file inside your site folder.
- **Pick how it runs.** Run it as a server with every feature, or publish it as plain files to any static host, many of which are free.
- **Fast pages.** A post needs no script to read, and the styles are small. Pictures get the right size for each screen.
- **Speed is checked on every change.** The project has [written speed budgets](https://github.com/thebigasif/khalid/blob/main/docs/standards/performance.md) for page building, start time, memory and page weight. A change that breaks one is not merged.
- **HTTPS on its own.** In server mode, Khalid can get and renew the certificate for your domain by itself.

## How it works

Pick the way that suits you:

- **Plain files:** run `khalid build` and upload the `public` folder to any static host.
- **A server:** run `khalid serve`, or install it as a service that starts with the machine.
- **Docker:** use the official image, or the Compose file in the repository.
- **A hosting platform:** ready files for Fly.io, Railway, Coolify and a Hetzner Cloud server.
- **Cloudflare:** a button makes a blog on Cloudflare's free plan.

The [deploy guide on GitHub](https://github.com/thebigasif/khalid/blob/main/docs/deploy.md) covers each kind of blog hosting, and [the start page](/khalid/start/) has the first steps.

In plain-file mode, the features that need a server are off: comments, forms, sign-ups, search, the admin and scheduling.

## See it live

- This website is plain files built by Khalid and served by GitHub Pages. Open any page with scripts turned off: it reads the same.
- [The demo blog](/khalid/demo/) is built the same way.

Next: [blog backups and safety](/khalid/features/blog-backups/). Also see [the writing checks](/khalid/features/writing-checks/).

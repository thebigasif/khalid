---
title: "The blog editor and admin: write in your browser"
slug: blog-editor
date: 2026-10-30
author: khalid-team
description: Khalid's blog editor lets your team write, edit and manage the blog in a browser, with the writing checks shown as you go.
focus_keyword: blog editor
---

Not everyone wants to edit text files. The admin is Khalid's blog editor in the browser. Your team signs in and writes there, and every post is still saved as a plain text file.

## What the blog editor does for you

- **Write the way you like.** Use rich text, or switch to markdown, the simple text format for headings and links.
- **Add blocks without code.** Type a slash to add a callout, a FAQ, a table, a picture, a button, a video or any other block.
- **See the score as you write.** The writing checks run after every save, so you fix things before you publish.
- **Never lose work.** Khalid keeps earlier versions of each post, and you can bring one back. A deleted post goes to the trash first.
- **Work as a team.** You see who else has a post open. Roles decide who can write, edit, publish or manage the site.
- **Keep pictures in one place.** The media library holds your pictures, with folders, cropping and a focal point, the part of a picture that stays in view.

## How to open it

The admin lives at `/admin/` on your blog. Make yourself the owner from the command line, inside your blog's folder, and Khalid shows a sign-in link once:

```sh
khalid staff owner --email you@example.com --name "Your Name"
```

After that you sign in with a password or a passkey. A passkey is a sign-in that uses your phone or computer instead of a password. Then invite the rest of your team from the admin.

The admin needs Khalid running as a server. A blog published as plain files, like the demo, has no admin. Each admin screen started as a drawing, and you can look through [the admin mockups on GitHub](https://github.com/thebigasif/khalid/tree/main/docs/design/admin-mockups).

## See it live

The demo is plain files, so it has no admin to sign in to. You can still see what the blog editor makes:

- [The SEO writing guide](/khalid/demo/seo-writing-guide/) uses a table of contents, callouts and links between posts.
- [The editing checklist](/khalid/demo/editing-checklist/) shows a checklist, pictures and a before and after.
- [The content benchmark](/khalid/demo/content-benchmark/) shows a chart and a table.

To try the admin itself, follow [the start guide](/khalid/start/). It takes a few commands on your own computer.

Next: [publish and schedule posts](/khalid/features/schedule-posts/). Also see [comments and forms](/khalid/features/blog-comments/).

---
title: "Blog comments and forms: hear from your readers"
slug: blog-comments
date: 2026-10-27
author: khalid-team
description: Turn on blog comments and contact forms, with spam kept out and no third-party script on your pages.
focus_keyword: blog comments
---

Readers have questions, and some of them become customers. Khalid has blog comments and forms built in, so you hear from readers without adding another company's script to your pages.

## What blog comments and forms do for you

- **Comments you approve.** New comments wait for your approval, so nothing shows up on your blog before you see it.
- **Simple replies.** Readers can reply to a comment, one level deep, so threads stay easy to follow.
- **They work with scripts off.** Comments and forms are plain web forms, so they work in every browser.
- **Forms that reach you.** A contact or request form sends each answer to your email, and keeps a copy on the server.
- **Spam is kept out.** Forms carry a hidden trap that bots fill in and people never see. Comments are limited per visitor, so a flood cannot get through.
- **Other tools can listen.** Each form answer can fire a webhook, a message Khalid sends to another tool, such as your sales system.

## How it works

Add a form to any page with the form block, written in the post like this:

```md
:::form{name="contact"}
:::
```

Set where the answers go with `forms_to` in `site.toml`. Blog comments are switched on in the reader settings of the same file. Both need Khalid running as a server, because a plain file cannot receive a message. The [deploy guide](https://github.com/thebigasif/khalid/blob/main/docs/deploy.md) shows how to run one.

Comments sit under each post in the order they came in. You approve, reply to or remove them in the admin.

## See it live

- [The demo's about page](/khalid/demo/about/) has a contact form. The demo is plain files, so you can see the form but not send it.
- The [agency example site](/khalid/examples/agency/) and the [local business example site](/khalid/examples/local-business/) use forms for enquiries.

Next: [analytics and AI visitors](/khalid/features/ai-visitors/). Also see [the APIs and the SDK](/khalid/features/blog-api/).

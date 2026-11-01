---
title: "Blog newsletter and members: reach readers by email"
slug: blog-newsletter
date: 2026-10-28
author: khalid-team
description: Collect subscribers, send a blog newsletter with your posts, and keep some posts or parts of posts for readers who sign up.
focus_keyword: blog newsletter
---

A reader who subscribes comes back. Khalid has a blog newsletter built in: it collects sign-ups, sends your posts to subscribers, and can keep some posts for members only.

## What the blog newsletter does for you

- **Collect subscribers on your own blog.** A sign-up form fits in any post or page. Each new subscriber confirms by email, so your list stays clean.
- **Run more than one list.** For example, a weekly digest and a product news list.
- **Send posts by email.** Send one post, or a digest of recent posts. A welcome email goes to each new subscriber.
- **Keep posts for members.** Mark a post for members only, or protect it with a password.
- **Offer a download for an email.** A gated block shows its content after the reader leaves an email address.

## How it works

Khalid sends email through a provider you already have: any SMTP mail server, Resend or Postmark. You add its settings in `site.toml`. Khalid sends nothing until you add a postal address, because the law in many countries asks for one in every newsletter. To send a post, run this inside your blog's folder:

```sh
khalid newsletter send why-internal-links-matter
```

Members-only posts carry `access: members` at the top of the post. The blog newsletter, sign-ups and members need Khalid running as a server.

Two things Khalid does not do: it does not take payments, so there are no paid memberships, and it does not count email opens. The full guide is [the newsletter guide on GitHub](https://github.com/thebigasif/khalid/blob/main/docs/newsletters.md).

## See it live

- [The content brief template](/khalid/demo/content-brief-template/) is a members-only post in the demo. The demo is plain files, so it shows what a visitor sees before signing in.
- The [newsletter example site](/khalid/examples/newsletter/) and the [membership example site](/khalid/examples/membership/) show both ideas as a whole blog.

Next: [blog comments and forms](/khalid/features/blog-comments/). Also see [how to move an old blog](/khalid/features/blog-migration/).

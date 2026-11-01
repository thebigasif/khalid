---
title: "A membership blog with posts for signed-in readers"
slug: membership-blog
date: 2026-10-27
author: khalid-team
description: How to run a membership blog on Khalid, with free posts for everyone and members-only posts, downloads and guides for readers who sign up.
focus_keyword: membership blog
---

Some of your best work is worth a sign-up. Khalid runs a membership blog where most posts are open to everyone, and some posts or parts of posts are kept for readers who join.

## Who this membership blog setup is for

A business or writer who offers guides, templates or longer posts in exchange for an email address, to grow a list of interested readers.

## The problem it solves

Gating content usually means another service, another login screen and another script on your pages. Readers drop off at the extra step, and search engines cannot tell what the page is about.

## What Khalid gives a membership blog

- **Members-only posts.** Mark a post with `access: members`, and only signed-in readers see the whole post.
- **Password posts.** Protect a post with a password for a small group, such as a class or a team.
- **Gated downloads.** A gated block shows a download or a section after the reader leaves an email address.
- **One list.** New members join your [blog newsletter](/khalid/features/blog-newsletter/) list, and confirm by email.
- **Search engines still understand the page.** Every post keeps its [SEO and AI search data](/khalid/features/seo-ai-search/), whatever part of it is gated.

Membership in Khalid is a free sign-up. Khalid takes no payments, so it has no paid plans or billing. If you charge for access, keep that part in a tool built for payments.

## How to set it up

1. Set up email in `site.toml` as the [newsletter guide](https://github.com/thebigasif/khalid/blob/main/docs/newsletters.md) shows.
2. Add `access: members` to the posts you want to keep for members.
3. Add a gated block to a free post, offering a download for an email address.
4. Run Khalid as a server, because something has to check who is signed in.

Start with one members-only post that is clearly worth more than a free one, and say in your free posts what members get.

## See it live

- Open [the membership example site](/khalid/examples/membership/), a whole membership blog.
- [The content brief template](/khalid/demo/content-brief-template/) is a members-only post in the demo.

Next use case: [a SaaS blog](/khalid/use-cases/saas-blog/). Also see [an agency blog](/khalid/use-cases/agency-blog/).

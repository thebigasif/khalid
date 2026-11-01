---
title: "A multilingual blog for readers in more than one language"
slug: multilingual-blog
date: 2026-10-28
author: khalid-team
description: How to run a multilingual blog on Khalid, with one folder per language and the links search engines need to show each reader the right version.
focus_keyword: multilingual blog
---

If your customers speak more than one language, your blog should too. Khalid runs a multilingual blog from one site folder, and it tells search engines which version of each post is for which reader.

## Who this multilingual blog setup is for

A business that sells in more than one country or language, with writers or translators for each language.

## The problem it solves

Translations often live as separate sites, or in plugins that fight each other. Search engines then show readers the wrong language, or treat a translation as a copy of the same page. Readers land on a page they cannot read and leave.

## What Khalid gives a multilingual blog

- **One folder per language.** English posts sit in `content/en/`, German posts in `content/de/`, and so on.
- **Linked translations.** Posts that share a translation key are linked to each other, and Khalid adds the language links search engines look for, known as hreflang.
- **Each language gets its own pages.** The home page, lists, feeds and sitemaps come in every language.
- **A check for gaps.** [The checks on every post](/khalid/features/writing-checks/) warn when a post has no version in a language you publish in.
- **One look everywhere.** [Theme sync](/khalid/features/theme-sync/) gives every language the same brand.

Two limits today: search inside the blog understands English and German words best, and the system pages and confirmation emails Khalid writes itself come in English and German.

## How to set it up

1. List your languages in `site.toml`, such as `languages = ["en", "de"]`.
2. Write each post in its language's folder.
3. Give each pair of translated posts the same `translation:` key, so Khalid links them.
4. Translate your about page and author pages too, so every reader can see who writes.

The [SEO and AI search features](/khalid/features/seo-ai-search/) do the rest. Google's own guide to [managing multi-regional and multilingual sites](https://developers.google.com/search/docs/specialty/international/managing-multi-regional-sites) explains why the language links matter.

## See it live

- Open [the multilingual example site](/khalid/examples/multilingual/), a whole multilingual blog.
- [The demo blog in German](/khalid/demo/de/) shows the language switch on a smaller scale.

Next use case: [a membership blog](/khalid/use-cases/membership-blog/). Also see [a local business blog](/khalid/use-cases/local-business-blog/).

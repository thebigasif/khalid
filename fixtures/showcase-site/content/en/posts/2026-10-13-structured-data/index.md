---
title: Structured data for blog posts, step by step
slug: structured-data
date: 2026-10-13
reviewed: 2026-10-20
author: tomas-lindqvist
reviewed_by: maren-okafor
category: technical-seo
description: What structured data tells search engines about a blog post, which fields matter, and how to add it and test it without breaking anything.
translation: structured-data
focus_keyword: structured data
tags: [schema, search results]
image:
  src: cover.svg
  alt: A blog page linked by dotted lines to three cards showing a person, a calendar and a star
---

Structured data is a short block of code that tells search engines, in their own language, what a page is: a blog post, who wrote it, when it was published and what it is about. Readers never see it. Search engines and AI tools read it first, because it saves them from guessing.

This post explains what to put in it for a blog post, how to add it, and how to check that it works.

## What structured data does for a post

A search engine can read your page and work out most of the facts. Structured data removes the guesswork. It states the headline, the author, the publish date and the image in a fixed format called schema.org vocabulary, usually written as JSON-LD inside the page.

That certainty pays off in three places:

- **Search results.** The date and the author can appear next to your link, which helps a reader trust it.
- **Author pages.** Linking each post to a person with a profile shows who stands behind the advice.
- **AI search.** Tools that quote pages want to name the author and the date. Clear markup hands them both.

:::callout{kind=note}
Structured data does not push a page up the rankings on its own. It helps search engines understand and present the page. The writing still has to earn the place.
:::

## The fields a blog post needs

Google's guide to [article markup](https://developers.google.com/search/docs/appearance/structured-data/article) lists no required fields, only recommended ones. In practice these are the ones that matter for a blog post:

- **headline:** the post's title, as readers see it.
- **author:** a person, with a name and a link to their profile page.
- **datePublished and dateModified:** when the post went live and when it last changed in a way that matters.
- **image:** the lead image, at a good size.
- **publisher:** the organisation behind the blog.

Here is the shape of the markup for one post. The dates and the address are left out to keep it short; a real page fills them in.

```json
{
  "@context": "https://schema.org",
  "@type": "BlogPosting",
  "headline": "Structured data for blog posts, step by step",
  "author": {
    "@type": "Person",
    "name": "Tomas Lindqvist",
    "url": "https://www.example.com/blog/authors/tomas-lindqvist/"
  },
  "publisher": {
    "@type": "Organization",
    "name": "Larkspur Studio"
  }
}
```

::bookmark{href="https://schema.org/BlogPosting" title="BlogPosting on schema.org" description="Every property a blog post can carry, with what each one means."}

## Hand-written markup or markup from your blog

You can write the block yourself, or let your blogging tool write it from the post's own fields. Each way has its place.

::::columns
:::column{title="Written by your blog"}
The markup comes from the title, author and dates you already fill in. It never drifts out of step with the page, and nobody has to remember it.
:::
:::column{title="Written by hand"}
You can describe things your blog does not know about, such as an event or a product. You also have to keep it correct every time the post changes.
:::
::::

Our advice: let the blog write the markup for every post, and add hand-written blocks only for the rare page that needs more.

## Add markup to a post in five steps

::::howto{name="Add structured data to a blog post" time="30:00"}
:::step{name="Fill in the post's own fields"}
Give the post a title, an author with a profile page, a publish date and a lead image. Good markup starts with complete fields.
:::
:::step{name="Check what the page already has"}
Open the page source and search for "ld+json". Many blogging tools add the markup for you, and two copies confuse search engines.
:::
:::step{name="Add only what is missing"}
If the page has no markup, add one BlogPosting block in the head of the page. Fill in each field from the list above.
:::
:::step{name="Test the page"}
Paste the address into Google's Rich Results Test. Fix every error, then read the warnings and fix the ones that apply.
:::
:::step{name="Keep the dates honest"}
Change the modified date only when the content changes in a way a reader would care about. Fixing a typo is not an update.
:::
::::

:::toggle{summary="Why not mark up every page as an article?"}
Markup has to describe what is really on the page. A category page is a list, an about page is about a company, and a product page is a product. Marking them all up as articles is misleading, and search engines can ignore the markup or the page for it.
:::

## Mistakes that make search engines ignore the markup

Most broken markup we see comes from a short list of slips:

1. **Markup that disagrees with the page.** The headline in the code says one thing and the page says another.
2. **Authors without a page.** A name alone is weak. Link the person to a profile that says who they are.
3. **Dates that change every day.** Some tools set the modified date to today on every visit. Search engines learn to distrust it.
4. **Copied blocks.** A template copied from another post still names the old title.

Run the test again after every template change. A small change to the theme can break the markup on every post at once.

## Where structured data fits with the rest

Markup is one part of how a post presents itself in search. The text people see still does the most work: write [a meta description that earns the click](/blog/meta-description/) and a clear title before you worry about code.

Speed matters too. A page that loads slowly gets fetched less often, so pair this work with the habits in our guide to [the three Core Web Vitals](/blog/core-web-vitals/). And because AI tools lean on named authors and dates, clear markup is part of [how AI search decides what to quote](/blog/ai-search-sources/).

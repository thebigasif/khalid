---
title: "Hreflang for a two-language blog, step by step"
slug: hreflang-guide
date: 2026-08-31
author: tomas-lindqvist
reviewed_by: maren-okafor
category: technical-seo
description: How to use hreflang to tell search engines which language each version of a post is in, with the tags from our English and German posts.
translation: hreflang-guide
focus_keyword: hreflang
secondary_keywords: [x-default, language versions]
cluster:
  name: technical-seo
  role: member
related: [technical-seo-audit, xml-sitemaps, core-web-vitals]
tags: [crawling, translation]
image:
  src: cover.svg
  alt: Two page cards in violet and mint joined by a pair of curved arrows, with a small globe between them
---

Hreflang is a short tag that tells search engines a page exists in other languages, and where each version lives. Get it right and a reader in Hamburg sees the German post while a reader in Leeds sees the English one. Get it wrong and search engines quietly ignore the lot.

We run this blog in English and German, so we have made most of the hreflang mistakes ourselves. This post shows the tags on our own pages, the rules behind them, and the errors we see most in audits. It belongs to our [full technical SEO audit](/blog/technical-seo-audit/), where language tags are one check among many.

:::callout{kind=summary}
- **Every version lists every version,** itself included.
- **Links must go both ways.** If the German page names the English one, the English page must name the German one.
- **Use a language code, then an optional region.** Never a region on its own.
- **Add x-default** for readers whose language you do not cover.
:::

::toc

## What hreflang tells search engines

A translated post is a set of language versions: pages that say the same thing to different readers. The tag names every member of that set and its language. Google can then pick the version that suits the person searching, and show it in place of the others.

The glossary entry on [[hreflang|language tags]] has the short version. The details below are from Google's guide to [localised versions of your pages](https://developers.google.com/search/docs/specialty/international/localized-versions), which is the reference we check against.

## The rules every set of tags must follow

Google's rules are few, and each one matters. Break one and the whole set can be dropped.

- **Name every version, including the page itself.** A page that leaves itself out of its own list is a common slip.
- **Return links are required.** If page A points to page B, page B must point back to A. Without the return link, Google ignores the pair.
- **Use full addresses.** Write the whole address with the domain, not a path that starts with a slash.
- **Codes follow two standards.** The language comes first, as a [two-letter ISO 639-1 code](https://www.loc.gov/standards/iso639-2/php/code_list.php). A region can follow, as an [ISO 3166-1 Alpha 2 country code](https://www.iso.org/iso-3166-country-codes.html), so `de` is German and `de-AT` is German for readers in Austria.
- **Add a fallback.** The value `x-default` names the page to show when no version matches the reader's language.

## Our own example: one post in two languages

Our post on [Core Web Vitals for writers](/blog/core-web-vitals/) has a German translation at `/blog/de/core-web-vitals/`. You can browse every German post from our [German home page](/blog/de/). Both versions of that post carry the same three lines in the head of the page:

```html
<link rel="alternate" hreflang="en" href="https://www.example.com/blog/core-web-vitals/">
<link rel="alternate" hreflang="de" href="https://www.example.com/blog/de/core-web-vitals/">
<link rel="alternate" hreflang="x-default" href="https://www.example.com/blog/core-web-vitals/">
```

The lines are the same on both pages. That is the point: each page names itself, names the other, and names the fallback. Our fallback is English, because a reader whose language we do not cover is more likely to read English than German.

:::figure{src="pairs.svg" alt="Hreflang diagram of the English and German versions of a post pointing to themselves and to each other, with a fallback arrow to the English page"}
Each page points at itself and at its partner, and both name the English page as the fallback. Notice that the arrows run both ways: remove one and Google can ignore the pair.
:::

:::callout{kind=note}
Our blog engine writes these tags for us. Each post's front matter has a `translation` field, and every post that shares the same value is treated as one set. If your engine has no such field, you will be writing the tags by hand or with a plugin.
:::

## Where to put the tags

Google reads the same information from three places. Pick one and use it everywhere, so you have one list to keep in step.

| Place | Best for | Watch out for |
|---|---|---|
| Link tags in the page head | Most blogs | Every template must print them |
| HTTP headers | Files such as PDFs that have no head | Needs access to the server settings |
| The sitemap | Large sites with many versions | The file grows fast with each language |

If you choose the sitemap route, each address lists its partners inside its own entry. Our guide to [what belongs in an XML sitemap](/blog/xml-sitemaps/) covers the rest of that file. We link the [[xml-sitemap|sitemap format]] in the glossary if you need a primer.

## Set up hreflang on a two-language blog

::::howto{name="Add hreflang tags to a blog in two languages" time="45:00"}
:::step{name="List each pair"}
Make a sheet with one row per post: the English address in one column and its translation in the next. Leave the second cell empty where there is no translation yet.
:::
:::step{name="Choose your codes"}
Use plain language codes, such as `en` and `de`, unless you write different versions for different countries. Most blogs do not need a region.
:::
:::step{name="Pick the fallback"}
Choose the version to show readers in other languages. For most blogs that is the original language.
:::
:::step{name="Add the tags to both pages"}
Put the full set on every version, including a line for the page itself and one for the fallback.
:::
:::step{name="Check the return links"}
Open each translated page and confirm it names its partner. A missing return link is the most common reason a set is ignored.
:::
::::

:::callout{kind=tip}
Translate the slug as well as the post when the words differ. A German reader trusts an address in German, and the tags do not care what the address says.
:::

## Common hreflang mistakes and their fixes

| Mistake | What happens | Fix |
|---|---|---|
| The page leaves itself out | The set is incomplete | Add a line for the page itself |
| No return link | Google ignores the pair | Add the tag to the partner page too |
| `en-UK` as a code | The region code is not valid | Use `en-GB` |
| A region on its own, such as `gb` | Read as a language code, not a country | Start with the language: `en-GB` |
| A canonical tag points to the other language | The two signals clash | Point each page's canonical at itself |
| A tag points to a redirect or an error | That version is dropped | Point it at the final, live address |

The canonical mistake deserves a word. A [[canonical-url|canonical address]] tells search engines which page is the real one among copies. A German translation is not a copy of the English post, so each version should name itself as canonical.

:::callout{kind=warning}
Never point the canonical tag of a translation at the original. You are telling Google the German page is a copy, and it may stop showing it at all.
:::

## What the tags do not do

Hreflang helps Google choose which version to show. It does not merge the versions, and it does not lift either one up the rankings. A good translation still has to earn its place.

The tags also do not stop Google treating two near-identical pages in the same language as duplicates. If you publish one English post for readers in Britain and one for readers in Ireland, and only the spelling differs, expect one of them to be picked as the main version. Hreflang still helps Google show the right address, but the content itself must differ enough to justify two pages.

::cta

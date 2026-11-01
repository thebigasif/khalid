---
title: How AI search chooses the pages it quotes
slug: ai-search-sources
date: 2026-10-27
reviewed: 2026-10-30
author: priya-raman
reviewed_by: tomas-lindqvist
category: ai-search
description: What we learned about how AI search picks the pages it quotes, and the writing habits that make a blog post easy to cite and hard to ignore.
translation: ai-search-sources
focus_keyword: AI search
tags: [ai search, citations, writing]
---

AI search tools answer a question in a few sentences and name a handful of pages as their sources. Being one of those sources is the new first place. This post explains what those tools look for when they choose a page, and how to write posts they can quote with confidence.

None of this replaces good writing for people. The pages AI search quotes are, almost without exception, pages a human reader would also find clear.

:::callout{kind=summary}
- **Answer early.** A self-contained answer right under a question heading is the easiest passage to quote.
- **Add something new.** Pages that repeat the top results give a tool no reason to cite them.
- **Name your sources.** A number with a source and a year is safer to repeat than a number alone.
- **Show who wrote it.** A named author with a profile makes the page easier to trust and to credit.
:::

## How does AI search pick a source?

An AI search tool first runs an ordinary search for the question, then reads the best pages it finds. It looks for short passages that answer the question directly and agree with other good sources. The pages it names are the ones whose passages it used, so clear, quotable answers win.

## What makes a passage easy to quote?

A quotable passage answers one question completely in a few sentences, without needing the paragraphs around it. It names the subject instead of saying "this" or "it", gives the number with its source, and stops. We call this pattern an [[answer-capsule|answer capsule]], and we put one under every question heading.

## Does ranking in ordinary search still matter?

Yes. Most AI search tools start from the pages a normal search engine already ranks, so a page nobody can find is a page nobody quotes. Good ordinary rankings put your post on the reading list. Clear passages, real sources and a named author then decide whether the tool actually cites it.

## What we saw in our own numbers

We looked at the questions our readers ask most and checked which pages the main AI search tools named as sources. The pattern was clear: the pages that were cited answered in the first paragraph under a heading, and most of them named a source for every number.

:::callout{kind=stat}
**3 in 5** of the pages AI tools cited in our [2026 content benchmark sample](/blog/content-benchmark/) answered the question within the first paragraph under its heading.
:::

The pages that were passed over had the same facts, but buried. They opened with a story, saved the answer for the end, or spread it across a long paragraph that also covered two other points.

:::pullquote{by="Priya Raman, research editor"}
The tools are not looking for the longest page. They are looking for the sentence they can lift without changing a word.
:::

## Give the tool something it cannot find elsewhere

If ten pages say the same thing, a tool can cite any of them, and it often picks the one with the strongest name. A small blog wins by adding [[information-gain|something the other pages lack]]: a test you ran, a number you counted, a worked example or a clear method.

That is why our own posts lean on first-hand work. The benchmark numbers we publish exist nowhere else, so a tool that wants them has to cite us.

## Mark up who wrote it and when

AI search tools want to tell their readers who said something and how recent it is. Make both easy to find:

- Show the author's name on the post, with a link to a profile that lists their experience.
- Show the publish date, and the updated date with a note on what changed.
- Add [structured data for the author and dates](/blog/structured-data/), so machines read the same facts that people see.

## Write for the reader, then check for the tool

The habits that help AI search are the same habits our [SEO writing guide](/blog/seo-writing-guide/) teaches for people: one question per post, the answer near the top, and short paragraphs with one idea each.

Start from the reader's goal. A post that matches [what the searcher really wants](/blog/search-intent/) is the post a tool can quote, because it answers the question that was asked. Then read your draft as a tool would: cover each paragraph under a question heading and ask whether it still answers the question on its own.

## Questions about AI search

::::faq
:::question{text="Can I block AI search tools from my blog?"}
You can. Most AI search tools respect the rules in your [robots.txt file](https://developers.google.com/search/docs/crawling-indexing/robots/intro), and each names its own crawler. Blocking them keeps your posts out of their answers, which also means no citations and no visits from them. For a blog that wants readers, we recommend letting them in.
:::
:::question{text="Do AI search tools send visitors?"}
They send fewer visitors than a top result in ordinary search, because the answer is often enough. The visitors they do send arrive already convinced the page is worth reading, so they stay longer. A citation also puts your brand name in front of every reader who sees the answer.
:::
:::question{text="Should I write shorter posts for AI search?"}
No. Write posts as long as the question needs, and make every section easy to quote on its own. A long guide with a clear answer under each heading gives a tool many passages to choose from. A short post with no clear answer gives it nothing to lift.
:::
::::

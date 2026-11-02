---
title: "How to optimise content for AI search: the complete guide"
slug: ai-search-guide
date: 2026-06-23
updated: 2026-10-28
updated_note: We rewrote the crawler section, added a robots.txt example and a way to measure visits from AI tools.
reviewed: 2026-10-28
author: priya-raman
reviewed_by: tomas-lindqvist
category: ai-search
description: How AI search tools find, read and cite pages, and what a small blog can do about it. Crawler rules, answer-first writing, sources and measuring.
translation: ai-search-guide
focus_keyword: AI search
secondary_keywords: [AI crawlers, AI referrals]
cluster:
  name: ai-search
  role: pillar
featured: true
related: [ai-search-sources, answer-capsules, ai-referral-traffic, podcast-answer-first, structured-data]
tags: [ai search, citations]
image:
  src: cover.svg
  alt: An answer panel with a sparkle, joined by three dotted lines to three page cards, one of them marked with a tick
---

AI search is any search tool that answers a question in its own words and names a few web pages as its sources. Google's AI features, ChatGPT search, Perplexity and others work this way. To optimise for AI search, a page has to be easy to find, easy to read in parts, and worth quoting.

The good news for a small blog is that most of the work is the work you should already be doing. The bad news is that much of what is sold as "AI optimisation" is guesswork. This guide sets out what is known, what is not, and what we do on our own posts.

::toc

:::callout{kind=summary}
- **Ranking still matters.** AI search tools mostly read pages a search index already holds, so a page nobody can find is a page nobody quotes.
- **No special markup is needed.** Google says its AI features use the same systems and rules as ordinary search.
- **Decide on crawlers on purpose.** Search crawlers and training crawlers are different, and you can treat them differently.
- **Answer first, then explain.** A plain answer under each question heading is the easiest passage to quote.
- **Add what others lack.** Original numbers, named sources and a real author give a tool a reason to cite you.
:::

## What AI search is

AI search is a search tool that writes an answer to the question, drawing on web pages it has found, and links some of those pages as sources. Google's AI Overviews and AI Mode, ChatGPT search and Perplexity all work this way. The page that gets cited earns a mention, and sometimes a visit.

That makes the old goal of "rank first" a little wider. A page can now win by being one of the three or four sources in an answer, even when it is not the top blue link. It can also lose visits it used to get, because the answer was enough. Both are true, and a good plan accepts both.

:::header{image="cover.svg" alt="The answer panel and page cards from the cover, shown as the banner for part one"}
## Part one: how AI answers are built

What happens between the question and the answer, and what Google has said about it.
:::

## How AI answers draw on pages

The details differ by tool and change often, but the broad pattern is public and stable. An AI search tool does not answer from memory alone. It searches first, then writes.

1. **The question is split.** The tool turns one question into several related searches, one per part of the question. Google calls this [[query-fan-out|query fan-out]].
2. **Each search runs against an index.** That is a store of pages a crawler has already fetched and processed, just like ordinary search.
3. **Passages are picked.** The tool reads the best pages and pulls out short passages that answer each part.
4. **The answer is written and grounded.** The model writes an answer from those passages, and links the pages it drew on.

:::figure{src="retrieval-flow.svg" alt="Flow diagram of an AI search answer: one question splits into three searches, each search reads the index, passages are chosen, and an answer is written with two cited sources"}
The path from question to answer. Notice that your page has two hurdles: it must be in the index to be read at all, and one of its passages must answer a part of the question better than the others.
:::

Two lessons follow. First, if search engines cannot crawl and index your page, AI search cannot use it. Second, the unit that gets chosen is often a passage, not a page. A long post with one clear answer under each heading offers many passages. A long post where the answer is spread across five paragraphs offers none.

Fan-out also explains why a single post can be cited for questions it never targets. Someone asks "how often should a small team publish, and how do we keep it up?", and the tool might run one search on pace and another on planning. A post with a clear passage on each can be picked twice.

## What Google says about its AI features

Google has published a page for site owners called [AI features and your website](https://developers.google.com/search/docs/appearance/ai-features). It is short, and every site owner should read it. In plain words, it says:

- **The same rules apply.** The SEO advice for ordinary search still holds for AI Overviews and AI Mode.
- **There are no extra requirements.** A page must be indexed and eligible to show with a snippet in Google Search. Nothing more is needed to be eligible.
- **No special files or markup.** You do not need new markup or a special text file to appear.
- **The usual controls work.** Rules such as `nosnippet`, `max-snippet` and [[noindex|a noindex rule]] limit what Google can show, in AI features as in ordinary results.
- **Traffic is counted.** Visits from AI features are included in the Search Console performance report, within the web search type. They are not split out.

An [[ai-overview|AI Overview]] is the short AI answer Google shows above or among the ordinary results for some searches. AI Mode is a separate tab where the whole results page is a conversation. Both draw on Google's normal index, which is crawled by Googlebot.

:::pullquote{by="Tomas Lindqvist, technical SEO lead"}
Every week someone asks me for the secret AI tag. There isn't one. The page that gets cited is the page that was crawlable, clear and worth citing.
:::

## What is known, and what is not

It helps to keep the two lists apart. Most bad advice comes from mixing them.

::::columns
:::column{title="Known"}
AI answers draw on pages found by search. Google's AI features use Google's index and the same rules as Search. Each crawler named below is documented by the company that runs it. Clear passages are easier to lift than buried ones.
:::
:::column{title="Not known"}
The exact way any tool scores and picks sources. How much weight a brand name carries. Whether a citation leads to a visit for a given question. Any "ranking factor" for AI answers that a vendor claims to have measured.
:::
::::

:::callout{kind=note}
Tools change month to month. When this guide says what a company does, it means what the company's own documentation said when we last checked, in October 2026. Check the source before you change your site.
:::

:::header{image="cover.svg" alt="The answer panel and page cards again, shown as the banner for part two"}
## Part two: make your pages easy to find and quote

Crawler rules, writing, sources, authors and measuring.
:::

## Let the right AI crawlers in

A crawler is a program that fetches pages for a search engine or a model. Your [[robots-txt]] file, a plain text file at the root of your site, tells each crawler which paths it may fetch. The rules are an honour system, but the major companies say their crawlers follow them.

The key point is that not all AI crawlers do the same job. Some fetch pages to show and link in search answers. Others collect pages that may be used to train models. You can allow one kind and refuse the other.

| Name in robots.txt | Run by | What the company says it is for | If you block it |
|---|---|---|---|
| Googlebot | Google | Crawling for Google Search, which includes AI Overviews and AI Mode | Your pages leave Google Search, AI features included |
| Google-Extended | Google | Not a separate crawler. A token that controls whether Google may use your pages to train Gemini models | Google says it does not affect your inclusion or ranking in Google Search |
| GPTBot | OpenAI | Collecting pages that may be used to train OpenAI's models | Your pages are not used for that training |
| OAI-SearchBot | OpenAI | Finding pages to show and link in ChatGPT's search features | Your pages may not appear as sources in ChatGPT search answers |
| PerplexityBot | Perplexity | Finding pages to show and link in Perplexity's answers | Your pages are less likely to appear as Perplexity sources |
| ClaudeBot | Anthropic | Collecting public pages that may be used to train Anthropic's models | Your pages are not collected for that training |

Google lists its crawlers and tokens in its [overview of Google crawlers and fetchers](https://developers.google.com/search/docs/crawling-indexing/overview-google-crawlers), and OpenAI describes its own in its [documentation on OpenAI's bots](https://platform.openai.com/docs/bots). Some tools also fetch a page at the moment a user asks about it, under yet another name, and the companies treat those visits in their own ways. Read each company's page before you write a rule for it.

Here is one common choice for a blog that wants readers from search and AI answers, but does not want its posts used to train models. It is a business choice, not an SEO rule.

```text
# Every crawler: stay out of the admin.
User-agent: *
Disallow: /admin/

# Training only: opt out of model training.
User-agent: GPTBot
User-agent: ClaudeBot
User-agent: Google-Extended
Disallow: /

# Googlebot, OAI-SearchBot and PerplexityBot follow the first group,
# so they can read every public post.
Sitemap: https://www.example.com/sitemap.xml
```

:::callout{kind=tip}
A crawler follows only the most specific group that names it, and ignores the `*` group. If you give a crawler its own group, repeat any shared rules in it. Then test the file with your search console's robots.txt report before you rely on it.
:::

:::callout{kind=warning}
Never block Googlebot to keep your pages out of AI Overviews. It removes them from Google Search as a whole. If you must limit what Google shows, use `nosnippet` or `max-snippet` on the page instead, and accept that it limits ordinary snippets too.
:::

For AI search, crawl rules are only the first step. AI crawlers also need pages that load as plain HTML, without waiting for scripts, and a site with no broken links or redirect chains. Our [technical SEO audit](/blog/technical-seo-audit/) checks all of that in an afternoon.

## Write answer-first

Once an AI search tool can read your page, the next question is whether it can lift a passage from it. The easiest passage to lift is a plain answer that sits right under the question it answers and makes sense on its own.

We call this an [[answer-capsule|answer capsule]]: a question as the heading, then forty to sixty words that answer it in full, naming the subject instead of saying "it". Then the rest of the section explains, adds examples and covers edge cases. Our [step-by-step post on answer capsules](/blog/answer-capsules/) has the full method, with before and after examples.

:::figure{src="ai-answer-mock.svg" alt="Mock AI answer screen with three numbered labels: the answer text lifted from one page, the source link to that page, and follow-up questions below"}
An annotated mock of an AI answer. The first label marks a passage taken almost word for word from one page. The second marks the link that page earned. The third marks follow-up questions, each a new chance for another of your passages to be picked.
:::

Answer-first writing is not a trick for machines. It is how good reference writing has always worked, and it helps people who skim. Our [SEO writing guide](/blog/seo-writing-guide/) shows the same habit from the reader's side. It also works beyond the blog: we tried it on audio in [our podcast episode on answering first](/blog/podcast-answer-first/), where the opening line of each segment does the same job as a capsule.

A few habits make passages easier to quote:

- **Name the subject.** "A content calendar is..." beats "It is...". A passage lifted alone loses the sentence before it.
- **One idea per paragraph.** A paragraph that answers two questions is a poor fit for either.
- **Put numbers next to their source.** A figure with its source in the same sentence can be quoted safely.
- **Use the words people search with.** If readers ask "how often", say "how often", not "cadence".

## Add information others lack

When ten pages say the same thing, a tool can cite any of them. It has no reason to choose a small blog over a big one. What changes that is [[information-gain|information gain]]: something your page adds that the others do not. A test you ran, a count you made, a worked example from your own work, or a clear method.

:::callout{kind=stat}
In 2026, **3 in 5** of the pages AI tools cited in our [benchmark sample of 140 blogs](/blog/content-benchmark/) answered the question in the first paragraph under its heading.
:::

Named sources matter for AI search in the same way. In 2026, only 19% of the blogs in that benchmark linked their numbers to a source. A number with no source is risky for a tool to repeat. A number with a link and a year is safe to repeat and easy to credit. Our study of [how AI tools pick the pages they quote](/blog/ai-search-sources/) shows the pattern in our own data.

:::callout{kind=tip}
Run one small study a year and publish the numbers on their own page, with your sample and method. It does not have to be big. It has to be yours, so that anyone who repeats the number has a reason to cite you.
:::

## Show who wrote it

AI search tools want to tell their readers who said something and how current it is. So do people. Google's guidance groups these signals under [[e-e-a-t|experience, expertise, authoritativeness and trust]], and it applies to AI features too.

Make the people behind each post easy to find:

1. **A named author on every post**, linked to a profile page that says what they have done and why they know the subject.
2. **A reviewer** for posts where a mistake would cost the reader, with the date of the review.
3. **Clear dates**: published, updated, and a short note on what changed.
4. **An about page and an editorial policy** that say who runs the site and how you check facts. Ours are on [the about page](/blog/about/) and in [our editorial policy](/blog/editorial-policy/).

Then say the same thing to machines. [[structured-data|Structured data for each post]] is a small block of code that labels the author, the dates and the organisation in a format search engines read. It does not make a page eligible for AI features on its own, and Google says no special markup is needed. It does remove doubt. That linked post shows the code step by step, and [schema.org](https://schema.org/) holds the full vocabulary.

An entity, in this sense, is a thing a search engine can tell apart from other things: a person, a company, a product. Use the same name for your company and your authors everywhere: on the site, in the markup, and on profiles elsewhere. Consistent names help machines connect the dots.

## Measure AI referrals

You cannot improve your AI search results if you cannot see them. The good news is that most AI referrals show up in ordinary analytics. A referral is a visit that arrives by a link from another site, and AI tools often pass their own address when a reader clicks a source.

Look for visits from addresses such as `chatgpt.com`, `perplexity.ai`, `gemini.google.com`, `copilot.microsoft.com` and `claude.ai`. Some tools also add a tag to the link, such as `utm_source=chatgpt.com`. Group them into one channel in your analytics tool with a pattern like this:

```text
chatgpt\.com|perplexity\.ai|gemini\.google\.com|copilot\.microsoft\.com|claude\.ai
```

:::maths{label="AI referral share"}
\text{AI referral share} = \frac{\text{visits from AI tools}}{\text{all visits}} \times 100
:::

Say your blog had six thousand visits last month, and a hundred and twenty came from AI tools. Divide the second by the first and multiply by a hundred: your AI referral share is two in every hundred visits. Track it monthly, and track which posts receive those visits. That list tells you which passages are being cited.

:::figure{src="referrals-chart.svg" alt="Bar chart of example data: monthly visits from AI tools to a small blog rising from 40 in June to 180 in October"}
Example data for one small blog. The numbers are small next to search, but they rise steadily. Notice which posts drive the rise, not just the total.
:::

There are limits. Google's AI Overviews and AI Mode do not appear as a separate source: their clicks are counted within Google search traffic. Some apps strip the referrer, so a share of AI visits will show as direct traffic. Treat the number as a floor, not a total. Our guide to [tracking visits from AI tools](/blog/ai-referral-traffic/) covers the setup step by step, including the edge cases.

:::toggle{summary="Should we also track mentions without a link?"}
Yes, if you have time. Once a month, ask the main tools the five questions your readers ask most, and note whether your site is named or linked. Keep the questions fixed so the results compare. It is rough, but it shows trends that analytics cannot.
:::

## What not to do

AI search is young, and gimmicks spread fast. Here is what we avoid.

- **Special AI markup or files.** Google's guidance says none is needed for its AI features. Proposed files for AI tools may do no harm, but treat them as optional and low priority.
- **Hidden text for machines.** Text that people cannot see but crawlers can is a spam tactic in ordinary search, and it is no safer here.
- **Mass-produced pages.** Hundreds of thin pages written by a model to catch every variant of a question add nothing new, and [Google's spam policies](https://developers.google.com/search/docs/essentials/spam-policies) cover scaled content made mainly to rank.
- **Chasing every tool.** Write for the reader's question. A clear passage works across tools. A trick tuned to one may stop working next month.
- **Blocking everything in a panic.** Blocking search crawlers costs you citations and visits. Decide crawler by crawler.

:::proscons
+ Blocking training crawlers keeps your posts out of future model training, where the company honours the rule
+ It costs nothing in Google Search: Google-Extended does not affect ranking
- Blocking search crawlers such as OAI-SearchBot or PerplexityBot removes you as a source in those tools
- Rules apply from now on and do not remove what was collected before
- New crawler names appear, so the file needs a check every few months
:::

## Your AI search checklist

::::howto{name="Prepare a blog for AI search" time="4:00:00"}
:::step{name="Check that pages are indexed"}
Open the page indexing report in Search Console and fix pages that should be indexed but are not. AI search can only use what search engines can read.
:::
:::step{name="Write your crawler rules on purpose"}
List the crawlers in the table above, decide for each one, and write the robots.txt rules. Test the file before you rely on it.
:::
:::step{name="Add an answer under each question heading"}
Go through your top ten posts. Under every question heading, put a plain answer of forty to sixty words that makes sense alone.
:::
:::step{name="Source every number"}
Link each number to its source and give its year. Remove the numbers you cannot source.
:::
:::step{name="Show the author and reviewer"}
Add a named author, a profile page, a reviewer where it matters, and the updated date with a note.
:::
:::step{name="Add one thing nobody else has"}
For each key post, add a worked example, a small test or a number you counted yourself.
:::
:::step{name="Set up AI referral tracking"}
Create one channel for AI tools in your analytics, then review it monthly with the list of posts that receive those visits.
:::
::::

::cta

:::callout{kind=read}
To find the questions worth answering first, start with [the intent behind a search, explained for writers](/blog/search-intent/). To plan which posts to write and refresh, read [our pillar guide for small content teams](/blog/content-strategy-guide/). To help crawlers and readers reach every post, read [internal linking for blogs](/blog/internal-linking/).
:::

## Questions about AI search

::::faq
:::question{text="Do I need a separate strategy for AI search?"}
No. Most of it is good SEO and good writing: crawlable pages, a plain answer near the top, sources and a named author. The new parts are small. Decide your crawler rules on purpose, and track AI referrals so you can see what gets cited.
:::
:::question{text="Will blocking Google-Extended remove my site from AI Overviews?"}
No. Google says Google-Extended controls whether your content is used to train Gemini models, and it does not affect Google Search. AI Overviews draw on Google's search index, which Googlebot crawls. To limit what appears there, use snippet controls on the page.[^1]
:::
:::question{text="Does structured data help me get cited?"}
It helps machines read who wrote the page and when, which is useful. Google says no special markup is needed for its AI features, so it is not a switch for citations. Add it because it removes doubt, not because it guarantees anything.
:::
:::question{text="Why do AI answers send so few visits?"}
Because the answer often settles the question, so many readers never click. The visits that do come tend to be from people who want more depth. Write posts that reward the click: worked examples, templates and data that the short answer cannot hold.
:::
:::question{text="How often should I review my robots.txt rules for AI crawlers?"}
Check them every three months, and whenever a tool you care about announces a new crawler. Names and roles change. Read the company's own documentation, update the file, and test it before you rely on it.
:::
::::

[^1]: The controls are described on Google's page on AI features and in its documentation of robots meta tags. They limit snippets in ordinary results as well, so use them with care.

::newsletter{label="Your email address" button="Get our AI search notes"}

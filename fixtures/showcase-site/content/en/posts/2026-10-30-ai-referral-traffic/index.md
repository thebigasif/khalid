---
title: "AI referral traffic: finding AI visits in analytics"
slug: ai-referral-traffic
date: 2026-10-30
author: priya-raman
reviewed_by: tomas-lindqvist
category: ai-search
description: Find the AI referral traffic that assistants send to your blog, group them in your analytics, and learn what the numbers can and cannot tell you.
translation: ai-referral-traffic
focus_keyword: AI referral traffic
secondary_keywords: [AI referrals, Search Console]
cluster:
  name: ai-search
  role: member
related: [ai-search-guide, answer-capsules, content-roi]
tags: [ai search, analytics]
image:
  src: cover.svg
  alt: A chat answer with a link, and a dotted path from that link to one orange row in an analytics table of visit sources
---

AI referral traffic is the set of visits that reach your blog when a reader clicks a link inside an AI assistant's answer. Some of them show up in your analytics with a clear source. Others arrive with no source at all and hide among your direct visits. This post shows how to find the ones you can see, group them, and read them without fooling yourself.

It sits in our [AI search guide](/blog/ai-search-guide/), which explains how these tools choose what to cite in the first place.

:::callout{kind=summary}
- **Some assistants pass a referrer.** Their visits can be grouped with one rule.
- **Some visits arrive with none.** They land in direct traffic and cannot be separated.
- **Google's AI features are counted with web search** in Search Console, not apart from it.
- **Compare quality, not just volume.** Look at which pages they land on and what readers do next.
:::

::toc

## Where AI referral traffic comes from

When you click a link on one site, your browser tells the next site where you came from. That note is the referrer, and analytics tools read it to name the source of a visit. The [Referer header on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referer) explains how it works and when a browser leaves it out.

Several AI assistants send a referrer when a reader clicks a cited link in the browser. The domains we see in our own reports include:

- `chatgpt.com` for ChatGPT
- `perplexity.ai` for Perplexity
- `gemini.google.com` for Gemini
- `copilot.microsoft.com` for Microsoft Copilot
- `claude.ai` for Claude

Some assistants also add a tracking tag to the link, such as `utm_source=chatgpt.com`. Your analytics will then show that value as the source even if the referrer is missing.

## Why some AI visits look like direct traffic

Not every click carries a referrer. A link opened from a phone app, a link copied into a new tab, or a strict privacy setting can all strip it. Those visits arrive with no source, and analytics files them as direct.

So the AI referral traffic you can see is a floor, not a total. The real number is higher by an amount you cannot measure. Treat any count as "at least this many", and watch the trend over months rather than the size of one month.

:::callout{kind=warning}
Do not add your direct traffic to your AI count to "correct" it. Direct visits also include bookmarks, typed addresses and links from email. There is no honest way to split them.
:::

## What Search Console shows for Google's AI features

Google's [[ai-overview|AI Overviews]] and AI Mode link to pages too. Clicks from them come from Google itself, so analytics files them as organic search, not as AI referrals.

In Search Console, Google counts those clicks and impressions in the Performance report for web search, together with the other results. Google's page on [AI features and your website](https://developers.google.com/search/docs/appearance/ai-features) describes this. At the time of writing the report has no filter that splits them out, so treat any tool that claims to show them apart with care. Our [Search Console setup guide](/blog/search-console-setup/) covers the report itself, and our study of [which pages AI tools cite](/blog/ai-search-sources/) shows what earns those links.

## Build an AI channel in your analytics

To track AI referral traffic over time, give it a home. Most analytics tools let you define your own channel, or at least a saved segment, from a rule on the source of a visit. The rule below matches the domains listed earlier. It is a regular expression, a short pattern that matches many values at once.

```text
.*(chatgpt\.com|openai\.com|perplexity\.ai|gemini\.google\.com|copilot\.microsoft\.com|claude\.ai).*
```

The leading and trailing `.*` let the pattern match whether your tool checks the whole value or only part of it.

::::howto{name="Group AI referrals into their own channel" time="20:00"}
:::step{name="Find where channels are defined"}
In Google Analytics this is the channel groups setting in the admin area. In other tools look for channels, sources or segments. The [Analytics Help Centre](https://support.google.com/analytics) has the current steps for Google's tool.
:::
:::step{name="Copy the default group"}
Work on a copy, so the original reports stay as they were. Name the copy something plain, such as "Channels with AI".
:::
:::step{name="Add a channel named AI assistants"}
Set its rule to "source matches regex" and paste the pattern above.
:::
:::step{name="Move it above referral"}
Channel rules run in order, and the first match wins. If the AI channel sits below the referral channel, those visits are counted as referrals before your rule sees them.
:::
:::step{name="Check it against raw sources"}
Open a report of sources for the last month. Every AI domain you can see there should now fall into the new channel.
:::
::::

:::callout{kind=tip}
Review the pattern every quarter. New assistants appear and old ones change their domain. A rule written once and never checked will slowly miss more visits.
:::

## What to compare once you can see them

A count of AI referral traffic on its own tells you little. The useful questions are about which pages and which readers.

| Compare | Why it helps |
|---|---|
| Landing pages | Shows which posts the assistants cite most |
| Engaged visits | Shows whether readers stay or bounce straight back |
| Sign-ups and enquiries | Shows whether the visits are worth anything to you |
| The same pages from organic search | Shows whether AI readers behave differently from searchers |

:::figure{src="sources.svg" alt="Bar chart of example data showing monthly AI referral traffic to a blog, split by assistant"}
Example data for a small blog, not real figures. Notice that one source sends most of the visits, so a change in that one assistant moves the whole line.
:::

In our own reports, the landing pages tell the clearest story. The posts that draw the most AI referral traffic are the ones with a plain answer under each question heading. If you want more of them, our guide to [[answer-capsule|writing answer capsules]] is the place to start. To judge whether those visits pay their way, see our post on [what content is worth to a business](/blog/content-roi/).

:::callout{kind=note}
AI crawlers that fetch your pages do not run the analytics script, so they never appear in these reports. They show up only in your server logs. A crawl is not a visit: count only the readers.
:::

## The limits of what you can measure

Keep three limits in mind when you report on AI referral traffic.

1. **The count is a floor.** Visits with no referrer are lost to direct.
2. **Google's AI features are mixed in.** They sit inside organic search and inside web search in Search Console.
3. **Sources change.** Domains move, apps change how they open links, and new tools arrive.

None of this makes the numbers useless. It makes them a trend to watch, not a total to report as fact.

::newsletter{label="Your email address" button="Get our AI search notes"}

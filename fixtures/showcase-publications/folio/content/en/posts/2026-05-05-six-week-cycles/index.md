---
title: "How we plan in six-week cycles"
slug: six-week-cycles
date: 2026-05-05
author: greta-voss
reviewed_by: ines-halloran
category: how-we-work
description: How the Tern team plans in six-week cycles with two weeks of cool-down, what we decide at the start of each cycle and what we refuse to plan.
focus_keyword: six-week cycles
tags: [planning]
image:
  src: lead.jpg
  alt: An open paper planner showing a month as a grid of ruled squares with dates printed in small red numbers, lying on a wooden desk
---

We plan Tern in six-week cycles: six weeks of building on a few chosen projects, then two weeks of cool-down for fixes, small jobs and deciding what comes next. We never plan more than one cycle ahead. That gives us six releases a year, each with a clear start and end, and no roadmap that slips quietly every month.

We did not start this way. In 2023 we kept a long roadmap in a shared document, and every item on it was always "next quarter". Six-week cycles replaced it after a year of missed dates and a release that took five months instead of two.

## What six-week cycles look like

Each cycle runs for eight weeks in total. The two parts have different rules.

| Weeks | Name | What happens | Who decides |
|---|---|---|---|
| 1 to 6 | Build | Two or three projects, each with a one-page spec, built and shipped | The people building each project |
| 7 | Cool-down | Bug fixes, small polish, support follow-ups, writing | Anyone, from a shared list |
| 8 | Planning | New specs written, reviewed and chosen for the next cycle | The whole team, then Greta and Ada |

Every project in a cycle starts with a [spec on one page](/blog/one-page-spec/). A project that cannot be built in six weeks is cut down until it can, or split across two cycles with a release at the end of each.

## The appetite comes first

The most important number in a spec is not the estimate. It is the appetite, a kind of [timebox](https://en.wikipedia.org/wiki/Timeboxing): how much time the problem deserves. We decide that first. Offline search had an appetite of six weeks with two engineers. Calmer reminders had an appetite of three weeks with one.

Setting the appetite first changes the conversation. We stop asking "how long will this take?" and start asking "what is the best version we can build in this time?" The second question has an answer. The first one rarely does.

:::figure{src="pencils.jpg" alt="Two sharpened grey pencils lying side by side in the corner of a sheet of bright yellow card, one for each project in most six-week cycles"}
Two pencils, two projects. Most cycles carry two main projects, which is as many as nine people can do well at once.
:::

## What we refuse to plan

Six-week cycles work because we say no to three things:

- **No backlog.** Ideas that are not chosen for the next cycle are not kept in a list. If an idea matters, it comes back, usually with better evidence.
- **No estimates beyond the cycle.** We will not say when a feature will ship unless it is in the current cycle.
- **No extending a cycle.** If a project is not done in six weeks, we ship what works, or we stop, and we decide in planning whether it deserves another cycle.

:::callout{kind=note}
Stopping a project at six weeks feels wasteful the first time. In practice it has happened four times in two years, and in three of those cases the second attempt was smaller and better.
:::

## Cool-down is not spare time

The two cool-down weeks are the part visitors to our team find strangest. Nobody is assigned to anything. People fix the bugs that annoyed them, answer support threads, write posts like this one, or try ideas that might become the next spec.

Cool-down is where most of our small improvements come from. The faster start in [Tern 2.1](/blog/tern-2-1-quick-capture/) came from one engineer's cool-down week. So did the fix for the white flash in dark mode, which had annoyed Ines for a year.

## What we have learned

After twelve six-week cycles, three lessons stand out. Small teams need hard edges on their time, or work expands to fill it. A clear appetite is worth more than a careful estimate. And people work better when they know exactly when the current push ends.

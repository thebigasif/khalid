---
title: "What forty interviews taught us this year"
slug: forty-interviews
date: 2026-11-04
author: rafe-morrow
reviewed_by: ines-halloran
category: essays
featured: true
description: What forty interviews with people who use Tern taught us in 2026, the five lessons that changed the app and how we run an interview that teaches us something.
focus_keyword: forty interviews
tags: [research, support]
image:
  src: lead.jpg
  alt: A small zebra-striped succulent in a round charcoal pot on a long wooden table, a second potted plant and bright office windows blurred behind it
---

Forty interviews this year taught us that people use Tern less to store notes than to think, that they find notes by memory of when, not where, and that they trust Tern most when it does least. Those lessons shaped every release from 2.1 to 2.5. This essay collects what we learned, and how we ran the interviews, in one place.

Forty interviews do not happen by accident. Every month, the team talks to between three and five people who use Tern, for 45 minutes each, on a video call. We do not show them new designs or ask what they want. We ask them to show us how they used Tern last week, and we watch. It is plain [user research](https://en.wikipedia.org/wiki/User_research), done often.

::toc

## How we ran forty interviews

We recruit from people who write to support, people who leave Tern, and a random sample of people who use it every week. The mix matters. People who write to us are often the most engaged. People who leave tell us the hardest truths.

| Group | Interviews | Why we talk to them |
|---|---|---|
| Wrote to support | 14 | They have a recent, specific problem we can watch |
| Use Tern every week | 17 | They show us habits we never designed for |
| Left in the last month | 6 | They tell us what was missing, plainly |
| New in their first week | 3 | They show us where Tern confuses people |

Two people from the team join each call. One asks questions; the other writes notes in Tern, linked to a note for the person and a note for each topic. After forty interviews, the topic notes and their backlinks are our best map of how people really use the app.

## Lesson one: people use notes to think

We expected people to describe Tern as a place to keep things. Most described it as a place to work things out. A teacher drafted lesson plans across a week of short notes. An engineer wrote a note per bug, with a sentence each time she learned something new.

This changed how we saw [linked notes](/blog/tern-2-0-linked-notes/), and how Ada tuned [search on your device](/blog/how-offline-search-works/). Links are not only for finding notes later. They are how people join small thoughts into bigger ones.

## Lesson two: people remember when, not where

When we asked people to find a note from last month, almost nobody started by browsing. They searched, or they said "it was the day of the dentist" or "the week before my holiday". People remember the time around a note far better than the place they put it.

That lesson led straight to two releases: search filters by date in [Tern 2.4](/blog/tern-2-4-offline-search/), and daily notes in [Tern 2.5](/blog/tern-2-5-daily-notes/). It also confirmed, for the tenth time, why [Tern has no folders](/blog/no-folders/).

## Lesson three: trust comes from doing less

The people who trusted Tern most were not the ones who used the most features. They were the ones who never had a surprise. Their notes were always there, sync never asked them anything, and reminders arrived when they said.

Several people described other apps they had left because the apps kept asking for attention. That is the root of [why Tern's reminders stay quiet](/blog/quiet-reminders/).

:::figure{src="seedling.jpg" alt="A tiny seedling with a few pale green leaves rising on a thin red stem above a bed of bright green moss, the background a soft green blur, a picture of how small notes grow, as forty interviews showed us"}
Most notes start small. The interviews showed us that people grow them over days and weeks, which is why links matter more than filing.
:::

## Lesson four: the hardest moment is the first week

The three interviews with new people were the hardest to watch. Each of them opened Tern to an empty screen and did not know where to begin. One wrote a single note and closed the app.

We now open a new account with three short notes that link to each other and explain, in a few sentences, how linking works. Since the change, the share of new people who write a second note in their first week rose from 58 to 71 percent (September to October 2026).

## Lesson five: leaving is often about one thing

All six people who had left gave one specific reason, not a general one. Two missed tags. Two needed to share notes with a family member. One wanted handwriting. One simply moved to the tool her new employer used.

We cannot build everything, and we will not try. But hearing the reasons from a person, not a survey, makes them hard to forget. Sharing notes with one other person is now the problem at the top of our next spec, and it will go through the same [release checklist](/blog/release-checklist/) as everything else.

## What forty interviews will change next year

We will keep forty interviews a year as our minimum, and we will add one thing: a short follow-up call three months later with five people from each group. Watching how a habit changes over time is the part of the story a single call cannot show. If you use Tern and would like to take part, write to team@example.com. We read every reply, as we read [every support email](/blog/everyone-answers-support/).

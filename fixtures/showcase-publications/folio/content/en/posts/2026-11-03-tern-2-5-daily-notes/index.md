---
title: "Tern 2.5: daily notes and calmer reminders"
slug: tern-2-5-daily-notes
date: 2026-11-03
author: greta-voss
reviewed_by: ines-halloran
category: changelog
facts:
  - label: Version
    value: "2.5"
  - label: Changes
    value: "6"
  - label: Fixes
    value: "4"
description: "Tern 2.5 adds daily notes, one note for each day that opens with a keystroke, and makes reminders calmer: silent by default, once only and easier to move."
focus_keyword: daily notes
tags: [v2.5, reminders, daily notes]
image:
  src: lead.jpg
  alt: A young green plant with broad pointed leaves growing in a terracotta pot inside a green glazed bowl, seen from directly above on a white table
---

Tern 2.5 adds daily notes: one note for each day, created the first time you open it, ready for whatever the day brings. It also makes reminders calmer, so they arrive once, silently by default, and are easier to move to a better time. Tern 2.5 is out today and free for everyone.

This is a small release on purpose. It passed the same [release checklist](/blog/release-checklist/) as the big ones, and the name daily notes passed the test we use to [name features](/blog/naming-features/). After the long work on [offline search in Tern 2.4](/blog/tern-2-4-offline-search/), we wanted a cycle that made Tern nicer to live in every day, rather than one big new feature.

## What is new in Tern 2.5

| Change | What it does | Notes |
|---|---|---|
| Daily notes | One note per day, named with the date, created when you first open it | Press Ctrl and D, or tap **Today** |
| Daily note links | Mention a date such as "next Friday" and Tern offers to link that day's note | Works for dates in the past and the future |
| Calendar strip | A row of the last two weeks at the top of each daily note | Days with a note are marked with a dot and a line |
| Silent reminders | Reminders make no sound or vibration unless you turn it on | Set per reminder or for all |
| Remind me later | Learns the times you usually choose and offers them first | Learned on your device only |
| Reminders list | Every reminder in one place in the sidebar | Change or delete several at once |

## How daily notes work

Press Ctrl and D on a computer, or tap **Today** on a phone, and Tern opens today's note. If it does not exist yet, Tern creates it, titled with the date. It starts empty. We tried templates with headings such as "Plans" and "Done", and testers ignored them or deleted them, so we left the page blank.

Daily notes are ordinary notes. You can link to them, link from them and search them. Their real strength shows after a few weeks: a project note's backlinks become a dated [diary](https://en.wikipedia.org/wiki/Diary) of every day you worked on it.

:::callout{kind=tip}
Use quick capture with daily notes. Captured thoughts can go straight into today's note instead of the capture inbox. Turn this on under **Settings**, then **Capture**.
:::

## Calmer reminders

We changed reminders to follow five rules: they arrive once, on time, silently unless you ask for sound, about the note itself, and with no streaks. Ines explains each rule and the ideas we turned down in [why Tern's reminders stay quiet](/blog/quiet-reminders/).

If you relied on sound for a reminder, nothing changes for it: reminders you set before this release keep their sound. Only new reminders are silent by default.

## Fixes in this release

- Search filters now work with dates written in your own language.
- The first search index builds faster on phones with little free space.
- Links to notes with emoji in the title open the right note.
- The sync status light no longer blinks during a long first sync.

## Thank you

Daily notes were requested by 260 people in the last year, many with the same words: "a page for today". Thank you to everyone on the beta who used them every day for three weeks and told us, kindly and clearly, to delete the templates.

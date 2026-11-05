---
title: "Tern 2.4: offline search"
slug: tern-2-4-offline-search
date: 2026-09-22
author: ada-kerrow
reviewed_by: greta-voss
category: changelog
description: "Tern 2.4 brings offline search: every search now runs on your device, works without a connection and returns results in under 50 milliseconds."
focus_keyword: offline search
tags: [v2.4, offline, search]
image:
  src: lead.jpg
  alt: The sharp corner of a pale board-marked concrete building, its formwork lines and tie holes crisp in daylight against a clear blue sky
---

Tern 2.4 brings offline search: every search now runs on your own device, so it works on a plane, in a tunnel or with your connection switched off, and results appear in under 50 milliseconds. Search was the last part of Tern that needed a network. With this release, the whole app works offline. Tern 2.4 is out today and free for everyone.

We explained the details in [how search works on your device](/blog/how-offline-search-works/). This note covers what changes for you, how the first index is built and what else is in the release.

## What offline search changes in Tern 2.4

| Change | What it does | Notes |
|---|---|---|
| Offline search | Searches your notes on your device, with or without a connection | Replaces server search everywhere |
| Faster results | Results appear as you type, in under 50 ms for most accounts | Up to 40 ms for 50,000 notes |
| Better ranking | Titles and words side by side rank higher | The note you wanted is first 81 percent of the time |
| Typing mistakes | One wrong letter is forgiven in longer words, a simple kind of [fuzzy matching](https://en.wikipedia.org/wiki/Approximate_string_matching) | "gardne" still finds "garden" |
| Search filters | Narrow results by date changed, linked notes or capture inbox | Type a filter or pick one from the menu |
| Private by design | Searches never leave your device | Nothing is logged on our servers |

## The first index

The first time Tern 2.4 opens, it builds a search index of all your notes on your device. For most accounts this takes a few seconds. For very large accounts it can take up to a minute. Tern builds the index only while your device is charging or has more than half its battery, and you can use Tern normally while it works. Until it finishes, search still uses our servers, so nothing stops working.

:::callout{kind=note}
The index takes extra space on your device: about a quarter of the size of your notes. For an account of 10,000 notes, that is about 11 MB.
:::

## Search filters

Search filters narrow results without making you remember a special syntax. You can type them or pick them from the menu beside the search box.

- **changed:** limits results to notes changed in a period, such as `changed:week` or `changed:2026-08`.
- **links:** finds notes that link to a note, such as `links:Garden`.
- **inbox:** searches only the capture inbox.

Filters work together. `links:Garden changed:month` finds every note about the garden that you changed this month.

## Fixes in this release

- Version history no longer lists the same version twice after a sync.
- Saved searches keep their order on every device.
- The search box keeps your last search when you switch notes and come back.
- Tables with very wide columns no longer push the note's title off screen.

## What is next

Next comes a smaller, calmer release: daily notes and quieter reminders. As always, the list of what we will not build is on the same page as the plan. Read the [Tern 2.3 notes](/blog/tern-2-3-version-history/) for the release before this one.

---
title: "How offline search works on your device"
slug: how-offline-search-works
date: 2026-09-08
author: ada-kerrow
reviewed_by: ines-halloran
category: engineering
description: How Tern's offline search works on your device, from the index it builds to the ranking that puts the right note first.
focus_keyword: offline search
tags: [offline, search, engineering]
image:
  src: lead.jpg
  alt: Rows of small pale wooden drawers in a library card catalogue, each with a handwritten paper tab, one drawer pulled open to show the cards inside
---

Offline search in Tern works by keeping a full search index on your device, next to your notes, and updating it every time you save. When you search, Tern reads that index and never asks a server, so results appear in under 50 milliseconds with or without a connection. This note explains how the index is built, how results are ranked and what it costs in space and battery.

Offline search ships to everyone in Tern 2.4 on 22 September. Until now, Tern searched on our servers, which meant search was the one feature that failed on a train or a plane. That broke the promise we made when we [built Tern to work offline first](/blog/offline-first/).

::toc

## What a search index is

A [search index](https://en.wikipedia.org/wiki/Search_engine_indexing) works like the card catalogue in an old library. Instead of reading every book to find a word, you look the word up in a drawer of cards, and each card lists the books that contain it.

Tern's index does the same for your notes. For each word, it keeps a list of the notes that contain it and where in each note it appears. Searching for "garden plan" means looking up two short lists and finding the notes on both, which takes a fraction of a millisecond even with tens of thousands of notes.

## How the index is built

Tern builds the index in four steps every time you save a note.

1. **Split the text into words.** Tern breaks the note into words, keeping numbers and words joined by hyphens together, so "2026-09-08" and "follow-up" each stay one word.
2. **Make each word plain.** Each word is lower-cased and accents are folded, so "café" and "cafe" match. Common endings are trimmed, so "planning" also finds "plan" and "planned".
3. **Record where each word appears.** Each entry notes the note and the position, so Tern can tell when two words appear side by side.
4. **Write the change, not the whole index.** Only the entries for the saved note change. The rest of the index is untouched.

The index lives in the same local database as your notes and is written in the same transaction. A note and its index entries are saved together or not at all, so the index can never point to a note that is not there.

```text
word        notes (position)
garden      n12 (4, 19)   n87 (2)   n203 (11)
plan        n12 (5)       n40 (1)   n203 (30)
```

In this small example, a search for "garden plan" finds n12 and n203. Note n12 ranks higher because the two words sit next to each other, at positions 4 and 5.

## How results are ranked

Finding the notes is the easy part. Putting the right one first is harder. Tern combines four signals into one score for each note.

| Signal | What it measures | Weight |
|---|---|---|
| Match in the title | Whether the words appear in the note's title | High |
| Words close together | Whether the search words appear side by side or near each other | High |
| Rare words | Whether the matched words are rare in your notes, so they say more | Medium |
| Recently edited | How recently you changed the note | Low |

We tested the weights on 600 real searches, shared with us by people from our interview group who agreed to take part. Each person told us which note they had wanted. Version history from [Tern 2.3](/blog/tern-2-3-version-history/) let us replay each search against the notes as they were that day. With these weights, that note was the first result 81 percent of the time and in the top three 94 percent of the time (August 2026). Our old server search scored 76 and 91 percent on the same test.

:::callout{kind=note}
We left "how often you open a note" out of the ranking on purpose. It made results better for a week and worse after a month, because it kept surfacing the same few notes. It also meant storing a history of what you open, which we would rather not keep.
:::

## Searching as you type

Offline search shows results as you type, which means running a new search after every letter. To keep that fast, Tern treats the last word you are typing as the start of a word. Typing "gard" finds "garden" and "gardening". Earlier words must match in full.

If you type a word that is not in the index at all, Tern tries the closest words that are, allowing one wrong letter for words of five letters or more. That catches most typing mistakes without filling the results with noise.

:::figure{src="books-shelf.jpg" alt="Offline search makes notes as quick to scan as a shelf: the end of a dark wooden bookcase in soft focus, its shelf packed with paperbacks whose spines catch the warm light from a lamp behind"}
A shelf you can scan in a glance. An index turns tens of thousands of notes into something just as quick to look through.
:::

## What it costs

Offline search on your device is not free. It takes space, and building it takes power. We measured both on a set of test accounts sized like our largest real users.

| Notes in the account | Notes on disk | Index on disk | First build time | Search time |
|---|---|---|---|---|
| 1,000 | 4 MB | 1 MB | Under 1 second | Under 5 ms |
| 10,000 | 40 MB | 11 MB | 6 seconds | Under 15 ms |
| 50,000 | 210 MB | 58 MB | 34 seconds | Under 40 ms |

The first build happens once, after you update to Tern 2.4, and only while your device is charging or above half its battery. After that, each save updates the index in well under a millisecond, which has no measurable effect on battery life in our tests.

## Limits we accepted

The index covers the text of your notes and their titles, not attached files or the bin. Extracting text from a large file on a phone can take longer than people will wait, so that work is left for a later cycle.

## Why we built offline search this way

We could have kept server search and cached recent results on the device. That would have been less work, and it would have failed exactly when people needed it most: with no signal. Offline search is more work to build and test, but it means search behaves the same everywhere, and none of your searches ever leave your device.

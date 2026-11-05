---
title: "Tern 2.3: faster sync and version history"
slug: tern-2-3-version-history
date: 2026-08-04
author: ada-kerrow
reviewed_by: greta-voss
category: changelog
description: "Tern 2.3 syncs notes up to three times faster, keeps 90 days of version history for every note and adds saved searches."
focus_keyword: version history
tags: [v2.3, sync]
image:
  src: lead.jpg
  alt: Looking straight up a spiral staircase at an octagonal glass skylight, its iron frame dividing the light into panes, ringed by carved balustrades on ochre walls
---

Tern 2.3 brings version history to every note: Tern now keeps each saved version of a note for 90 days, and you can see, compare and restore any of them. Sync is also up to three times faster, and saved searches let you keep a search one click away. Tern 2.3 is out today for everyone, on every device.

Version history is the change we are proudest of in this release, because it removes a fear. People told us they hesitated to cut a long note down, in case they wanted the old text back. Now they can cut freely.

## What is new in Tern 2.3

| Change | What it does | Plan |
|---|---|---|
| Version history | Keeps every saved version of a note for 90 days | Free and paid |
| Compare versions | Shows what changed between two versions, line by line | Free and paid |
| Restore a version | Brings back an old version as the current one, keeping the newer one in history | Free and paid |
| Faster sync | Sends only the changed parts of a note, not the whole note | Free and paid |
| Saved searches | Keeps a search in the sidebar with its own live count | Free and paid |
| Longer history | Keeps versions for a year instead of 90 days | Paid |

## How version history works

Tern saves a version when you stop typing for 30 seconds, when you close a note and when sync brings in a change from another device. Versions that differ by only a few characters are merged, so the list stays short enough to read.

To see a note's history, open the note's menu and choose **History**. The list shows each version with its time and the device it came from. Choose two versions to compare them: removed text is struck through and new text is underlined, much like a [diff](https://en.wikipedia.org/wiki/Diff), so the difference does not rely on colour alone.

:::callout{kind=tip}
Restoring an old version never deletes the newer one. It becomes the newest entry in history, so you can always change your mind again.
:::

## Why sync is faster

Until this release, Tern sent the whole note to your other devices on every change. For a short note that was fine. For a long note, such as a year of meeting notes, it meant sending tens of kilobytes for every word typed.

Tern 2.3 sends only the parts of a note that changed. On our test set of real, anonymised note sizes, the median sync of a change fell from 1.2 seconds to 0.4 seconds, and the data sent fell by 85 percent. The gain is largest on slow phone connections, which is where people noticed sync most. We explain how Tern stores notes on your device in [why we built Tern to work offline first](/blog/offline-first/).

## Saved searches

A saved search is a search you keep in the sidebar, such as every note that links to "Garden" and was changed this month. Its count updates as your notes change. Saved searches also answer a request we have heard since [we removed tags in Tern 2.0](/blog/removed-tags/): a way to gather notes without filing them.

## Fixes in this release

- Notes with very long [tables from Tern 2.2](/blog/tern-2-2-tables/) no longer scroll slowly on older phones.
- Sync no longer pauses after a laptop wakes from sleep.
- The history button is now reachable with the keyboard alone.
- Fixed a rare case where two devices editing the same line produced a duplicate line.

## Thank you

Version history was first suggested in a support email in 2023, by a reader who had pasted over a whole chapter of notes. We wrote back then to say we would build it. It took longer than we hoped, and we are glad to finally keep that promise.

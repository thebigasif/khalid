---
title: "Faster board loading: large boards open twice as fast"
slug: faster-board-loading
date: 2026-10-28
author: tomas-ferreira
category: improved
description: "Large boards now open in about half the time. The faster board loading comes from drawing the cards you can see first."
focus_keyword: board loading
tags: [boards]
facts:
  - label: Version
    value: "3.4.0"
  - label: Platforms
    value: "Web"
  - label: Plans
    value: "All plans"
---

Board loading is now about twice as fast for large boards. A board with 600 cards took close to three seconds to open on a typical laptop. It now opens in a little over one second, and the first lists appear almost at once.

## What we changed

Before this version, Mossboard drew every card on a board before it showed any of them. That was fine for a board of 40 cards and slow for a board of 600. Now it draws the lists and cards you can see first, then the rest while you read. This is often called [lazy loading](https://en.wikipedia.org/wiki/Lazy_loading). Scrolling stays smooth, because the cards further down are ready before you reach them.

We also send less data. A card on a board only needs its title, labels, people and due date. The description, checklist and comments now load when you open the card, not with the board.

## Who will notice faster board loading

Teams with large boards will see the biggest change: support queues, content calendars and long backlogs. On a small board the difference is too small to notice, though it still uses less data, which helps on a slow phone connection or a busy train.

## How we measured it

We timed board loading on the boards of 40 teams who agreed to help, before and after the change. We counted the time until the first list showed and the time until every card was ready. Both fell by about half on boards over 300 cards. On boards under 50 cards the change was less than a tenth of a second.

## What it means for other changes

The [recurring cards](/blog/recurring-cards/) in this version open straight from the board. The [time zone fix for due dates](/blog/due-dates-time-zones/) also shipped in 3.4, so the dates you see on a fast board are the right ones too.

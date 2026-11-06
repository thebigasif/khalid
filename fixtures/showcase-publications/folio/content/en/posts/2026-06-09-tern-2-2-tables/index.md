---
title: "Tern 2.2: tables in notes"
slug: tern-2-2-tables
date: 2026-06-09
author: greta-voss
reviewed_by: ada-kerrow
category: changelog
facts:
  - label: Version
    value: "2.2"
  - label: Changes
    value: "6"
  - label: Fixes
    value: "4"
description: "Tern 2.2 adds tables to notes: type a pipe or paste from a spreadsheet, sort any column, and keep tables readable on a phone."
focus_keyword: tables
tags: [v2.2, editor]
image:
  src: lead.jpg
  alt: Two hands at work on a blank page of a spiral sketchbook, one holding a short black ruler flat, the other drawing a straight line with a silver pen
---

Tern 2.2 adds tables in notes: start a line with a pipe character, or paste cells from a spreadsheet, and Tern turns them into a table you can edit, sort and resize. Tables work offline like the rest of Tern, sync line by line, and stay readable on a phone. Tern 2.2 is out today and free for everyone.

Tables were the second most requested feature of 2025, after quick capture. People wanted them for small, practical things: a packing list with columns, a reading log, a budget for a weekend away. They did not want a spreadsheet inside their notes, and neither did we.

## What is new in Tern 2.2

| Change | How to use it | Notes |
|---|---|---|
| Create a table | Type a pipe at the start of a line, or choose **Table** from the insert menu | Starts with three columns and two rows |
| Paste a table | Copy cells from a spreadsheet and paste | Keeps the cells, drops the formulas |
| Sort a column | Click or tap a column heading | Sorts text, numbers and dates in the right order |
| Move rows and columns | Drag the handle at the edge | Or use the keyboard: Alt with an arrow key |
| Narrow screens | Tables scroll sideways inside the note | The first column stays in place |
| Export | Tables export as plain text tables | Readable in any text editor |

## What tables in notes do not do

We kept tables simple on purpose. Our [one-page spec](/blog/one-page-spec/) for this [six-week cycle](/blog/six-week-cycles/) listed what we would not build, and we held to it:

- **No formulas.** A table can hold numbers, but it will not add them up. When you need sums, you need a spreadsheet.
- **No merged cells.** They make tables hard to read on a phone and hard for screen readers to follow.
- **No colours per cell.** Colour on its own would be the only signal for some people, and it fades in dark mode.

Every one of these was asked for at least a dozen times in testing. We may add totals one day. The others are unlikely.

:::callout{kind=tip}
To turn a list into a table, select the list and choose **Table** from the insert menu. Tern splits each line at commas or tabs and puts the pieces in columns.
:::

## Tables and screen readers

Every table in Tern is a real table to assistive technology, such as a [screen reader](https://en.wikipedia.org/wiki/Screen_reader), with its first row read as headings. Screen readers announce the row and column as you move between cells. We tested with three screen readers on four platforms and with two people who use them every day, who found eleven problems in the first build. All eleven are fixed in this release.

## Tables and sync

A table syncs like any other part of a note: change by change, not as a whole. If you edit one cell on your phone and another cell on your laptop while apart, both changes survive. If you both edit the same cell, Tern keeps both values in that cell, one under the other, and marks it for you to choose.

## Fixes in this release

- Capture from the share menu now keeps the source link when you share a page.
- The capture inbox count updates straight away on every device.
- Long titles no longer overlap the sync status light on small phones.
- Pasting text with tabs into a normal paragraph no longer creates a table by surprise.

Read the [Tern 2.1 notes](/blog/tern-2-1-quick-capture/) for the release before this one. Next, we turn to sync speed and a way to see older versions of a note.

---
title: "Large attachments over 20 MB upload again"
slug: large-attachments
date: 2026-09-22
author: tomas-ferreira
category: fixed
description: "After version 3.2 some large attachments over 20 MB stopped half way. They upload again, up to the limit of your plan."
focus_keyword: large attachments
tags: [boards]
facts:
  - label: Version
    value: "3.2.1"
  - label: Platforms
    value: "Web"
  - label: Plans
    value: "All plans"
---

After version 3.2, some large attachments stopped part of the way through. A progress bar reached about half and then waited for ever. Files under 20 MB were fine. This is fixed on the web, and large attachments upload again up to the limit of your plan.

The mobile apps were never affected, because they upload files in one piece. Only the web app split files, and only files over 20 MB were cut at the wrong place. Smaller files, images pasted into comments and files sent by email to a board all worked as before.

## What went wrong

Version 3.2 changed how files are split into pieces for upload, so that an upload can carry on after a short loss of connection. For files over 20 [megabytes](https://en.wikipedia.org/wiki/Megabyte), one piece in the middle was given the wrong size, and the server waited for bytes that never came.

## How large attachments work now

Each piece is now checked before it is sent, and the server says clearly when a piece is missing, so the upload can send it again. A file that stops now shows **Try again** in place of a progress bar that never moves. We also added a test that uploads files of many sizes before each release, so this kind of fault is caught before it ships.

## Limits by plan

| Plan | Largest file |
|---|---|
| Free | 25 MB |
| Team | 250 MB |
| Business | 1 GB |

## What you need to do

If an upload failed while the bug was live, attach the file again. Nothing was half saved on the card, so there is no broken copy to remove.

The other fix in 3.2.1 stopped [digest emails going out twice](/blog/digest-emails-twice/). Files you share with clients through [guest access](/blog/guest-access/) follow the same limits, and the [boards area page](/blog/tag/boards/) lists every other change to cards and boards.

---
title: "Tern 2.1: quick capture from anywhere"
slug: tern-2-1-quick-capture
date: 2026-04-14
reviewed: 2026-10-01
author: greta-voss
reviewed_by: rafe-morrow
category: changelog
description: "Tern 2.1 adds quick capture: a small window that opens from anywhere, saves a note in one keystroke and files it in a capture inbox for later."
focus_keyword: quick capture
tags: [v2.1, capture]
image:
  src: lead.jpg
  alt: A cube of loose note paper in four soft colours, yellow, blue, pink and green, standing on a warm brown desk
---

Tern 2.1 adds quick capture: press one shortcut from any app, type a thought and press Return, and Tern saves it as a note without opening the main window. Each captured note lands in a new capture inbox, where you can link it or move on later. Tern 2.1 is out today on every platform and is free for everyone.

Quick capture is the feature people asked for most in 2025. In our support inbox, 212 people described the same moment: an idea arrives while they are doing something else, and by the time Tern is open, the idea has gone.

## What is new in Tern 2.1

| Change | Where | Notes |
|---|---|---|
| Quick capture window | Desktop | Opens with a shortcut you choose; Alt and Space by default |
| Capture from the share menu | Phone and tablet | Share text, a link or a photo into Tern without leaving the app you are in |
| Capture widget | Phone home screen | One tap opens a blank capture note |
| Capture inbox | Everywhere | Every captured note waits here until you link it or clear it |
| Keyboard-only capture | Desktop | Return saves, Escape closes, nothing else is needed |
| Faster start | Desktop | Tern opens about 40 percent faster on older laptops |

## How quick capture works

The capture window is deliberately small: one line for a title, a box for the text and nothing else. There is no formatting bar and no list of notes to link to. We tried both in testing and they slowed people down, because they made capture feel like writing.

When you press Return, Tern saves the note at once, on your device. It syncs in the background when it can, like every Tern note. If you are offline, the note is still saved; it simply syncs later. This is the same [offline-first design](/blog/offline-first/) the rest of Tern uses.

## The capture inbox

Captured notes do not vanish into your notes. They wait in the capture inbox, at the top of the sidebar, with a small count beside it. From the inbox you can:

- **Link a note** to an existing note, which moves it out of the inbox;
- **Keep it as it is**, which files it with your other notes;
- **Clear it**, which moves it to the bin for 30 days.

:::callout{kind=tip}
Try clearing your capture inbox once a day, at the same time. Most captured notes take under ten seconds to sort, and the ones you cannot sort are usually worth turning into a proper note.
:::

## Fixes in this release

- Pasting a long list no longer drops the last item on some phones.
- Links to notes with a full stop in the title now open the right note.
- Dark mode no longer shows a white flash when a note opens.
- The editor keeps your place when sync updates the same note.

## What is next

We are now working on tables in notes, the second most requested feature this year. If you capture a thought about tables, you know where to send it: team@example.com, or reply to this release email. Read the [Tern 2.0 notes](/blog/tern-2-0-linked-notes/) for what came before this release.

---
title: "Why Tern has no folders"
slug: no-folders
date: 2026-01-20
reviewed: 2026-10-12
author: ines-halloran
reviewed_by: ada-kerrow
category: essays
description: Why Tern has no folders, what we use instead to keep notes findable, and what we learned from the people who missed folders most.
focus_keyword: no folders
tags: [design, linking]
image:
  src: lead.jpg
  alt: A smooth white interior wall curving into an open, rounded window, lit by soft daylight with no corners or edges in sight
---

Tern has no folders because a note rarely belongs in only one place, and folders force you to choose one. Instead, Tern keeps every note in one place and helps you find it again with links between notes and fast search. After three years, we are more sure of this decision than when we made it, and this essay explains why.

When we started Tern in 2022, Ada and I had both spent years watching people lose notes. Not delete them: lose them. The note was there, in a folder inside a folder, named something that made sense on the day it was written. Folders looked tidy and hid things well.

## The problem with folders

A folder asks you to decide where a note belongs at the moment you write it. That is the worst moment to decide. You do not yet know what the note will become, or which other notes it will matter to.

Take a note about a conversation with a builder about a new kitchen window. Does it go in "Home", "Budget", "Kitchen" or "People"? It belongs in all four. A folder makes you pick one, and in six months you will look in one of the other three.

We counted this in early interviews. People who used folders in other apps spent, on average, 40 seconds finding a note they had written the week before. Most of that time went on opening the wrong folders first.

## What Tern does with no folders

Tern replaces folders with three simple tools. None of them asks you to decide where a note lives.

| Instead of | Tern uses | How it helps |
|---|---|---|
| Putting a note in a folder | Linking it to other notes | A note can link to as many notes as it relates to |
| Opening folders to browse | Search that works as you type | The note appears after two or three letters |
| A folder for a project | A note for the project, with links to the rest | The project note becomes a living table of contents |

The third row is the one that surprises people. In Tern, a project is just a note. You write "Kitchen" at the top, then link the builder conversation, the budget and the window measurements to it. Each of those notes can also link to "Budget" or "People". Nothing is filed; everything is connected.

## The people who missed folders

Not everyone agreed with us. In our first year, about one person in ten who tried Tern told us they missed folders. We took that seriously and interviewed 24 of them about life with no folders.

What they missed was not really folders. It was three things folders gave them:

- **A sense of order.** Seeing their notes grouped made them feel in control.
- **A place to start.** Opening the app to a short list of folders felt calmer than a long list of notes.
- **A way to hide things.** Old projects went into a folder and out of sight.

Each of these deserved an answer that did not bring folders back. Pinned notes gave people a place to start: a handful of project notes at the top of the sidebar. Archiving gave them a way to hide finished work. And the sense of order came, slowly, from links. After a month, most people's notes formed clear clusters around a few project notes, without anyone filing anything.

:::callout{kind=tip}
If you are moving to Tern from an app with folders, make one note for each of your old folders and link the imported notes to it. Tern's importer does this for you, so nothing feels lost on the first day.
:::

## What we got wrong

We were wrong about one thing: tags. In Tern 1, we kept tags as a lighter kind of folder. People used them exactly like folders, with the same problems, plus a new one: three slightly different tags for the same idea. So in Tern 2.0, out next month, every tag becomes a note and every tagged note gets a link to it. We have since written about [what we learned when we removed tags](/blog/removed-tags/).

## Why we still think no folders is right

The best evidence is quiet. People who stay with Tern for three months find their notes faster than they did in their old app, by their own account and by our measurements. They also write more links each month, which tells us the links are useful, not a chore.

Folders feel like order because they look like a [filing cabinet](https://en.wikipedia.org/wiki/Filing_cabinet). But a filing cabinet works because a clerk files every paper in one agreed place. Most of us are not clerks, and our notes are not papers. They are thoughts, and thoughts connect to more than one thing. That is why Tern has no folders, and why we expect it never will.

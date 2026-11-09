---
title: "Why we still sketch on paper before we design"
slug: sketch-on-paper
date: 2026-05-26
author: ines-halloran
reviewed_by: greta-voss
category: essays
description: Why every screen in Tern starts as a pencil sketch on paper, what paper is better at than a design tool, and how we move from sketch to screen.
focus_keyword: sketch on paper
tags: [design, process]
image:
  src: lead.jpg
  alt: A hand in a grey knitted sleeve uses a blue mechanical pencil to sketch on paper, drawing rows of boxes for a screen layout, with a white mug and a spiral notebook beside it on a wooden table
---

We sketch on paper before we design because paper is fast, cheap and honest about how unfinished an idea is. A pencil sketch takes two minutes, invites people to change it, and cannot be mistaken for a decision. Every screen in Tern, including the ones you use every day, started as a drawing on paper that took less time to make than to describe.

I have designed software for twelve years, and for most of them I opened a design tool first instead of choosing to sketch on paper. I still use one every day. But I have stopped starting there, and this essay is about why.

::toc

## Why we sketch on paper first

A design tool is very good at the final details of a screen: spacing, type, colour, the exact state of a button. It is poor at the big early questions, where the question is not "how should this look?" but "should this exist, and where?"

Paper is the opposite, and that is why we sketch on paper before anything else. It is useless for the final details and excellent for the big questions. When I sketch on paper, I can draw six versions of a screen in the time it takes to set up one frame in a design tool. Six rough answers to a question are worth far more than one polished answer, because the first answer is rarely the best one.

Paper is also good at three quieter things:

- **It hides the details.** A sketch cannot show the font or the exact grey, so nobody argues about them too early.
- **It looks unfinished.** People feel free to cross things out, draw arrows and write "why?" in the margin.
- **It is easy to throw away.** Nobody grieves for a sketch. A screen that took a day to build in a design tool is much harder to let go.

## How a sketch changed Tern

The clearest example is the capture inbox in [Tern 2.1](/blog/tern-2-1-quick-capture/). Our first idea, in the spec, was to file captured notes straight into your other notes, with a small marker. It seemed simple.

I drew it on paper, then drew what the notes list would look like after a busy week with 30 captured thoughts mixed into it. The second drawing made the problem obvious at once: the list was a mess, and the marker did not help. The inbox came from that second sketch, drawn in about four minutes on the back of a printed agenda.

In a design tool, I would have designed the first idea carefully and well, and we might have shipped it.

:::figure{src="shavings.jpg" alt="A yellow pencil and a small metal sharpener on an open lined notebook, ready to sketch on paper, with curled pencil shavings scattered across the blank page"}
A sharp pencil and a blank page. The best sketches come from the first page of a fresh notebook, before anything on it feels precious.
:::

## Our sketching rules

Over three years, the team has settled on a few rules for how we sketch on paper. None of them is clever, and all of them help.

| Rule | Why it helps |
|---|---|
| Use a thick pen or a soft pencil | It stops you drawing small details you should not decide yet |
| Draw at least three versions | The first idea is usually the obvious one, not the best one |
| One screen per sheet | Sheets can be laid out on a table and moved around as a flow |
| Write the user's goal at the top | Every sketch is judged against the same question |
| Photograph sketches into Tern | They live beside the spec and anyone can find them later |
| Never redraw a sketch neatly | A neat sketch is a design pretending to be a sketch |

The thick pen rule is the one people resist most. Engineers in particular like a fine pen, because they want to be precise. That is exactly the problem. A sketch should be too rough to be precise.

## From sketch to screen

A sketch does not go straight into a design tool. It goes through three steps first.

### Step one: the sketch review

We lay the sketches on a table, or post photos of them in the spec, and anyone on the team can comment. The question is always the same: does this help the person in the spec reach their goal? We usually pick one sketch and steal one or two ideas from the others.

### Step two: the paper flow

Next, I draw the chosen screen in every state it can be in: empty, full, loading, offline and broken. This is where most surprises appear. The offline state of a screen is often the one that shapes the design, because Tern [works offline first](/blog/offline-first/), and an offline screen has to make sense without any help from the server.

### Step three: the first build

Then an engineer and I build the roughest working version of the screen, often in a day. It uses the real app and real notes, and it looks plain. Only when it works with real content do I move to a design tool for the final details.

:::callout{kind=note}
We skip the design tool entirely more often than you would think. Several screens in [Tern 2.0](/blog/tern-2-0-linked-notes/) went from sketch to first build to shipped, with the details set straight in code against our design tokens.
:::

## When paper is the wrong tool

Paper has limits, and pretending otherwise would be dishonest. It is poor at motion, so we prototype animation in code. It cannot test colour contrast, so every colour choice goes through a contrast checker. And it is hard to share with people who are not in the room, which matters more each year as more of the team works from home.

For remote work, we photograph sketches with a phone and add them to the spec, and we keep a shared folder of scanned sketches going back to 2022. It is one of the most useful things we own. When someone asks why a screen works the way it does, the answer is often a photo of a page from a notebook.

## What I would tell a new designer

Sketch on paper first, even if it feels slow or old-fashioned. [Paper prototyping](https://en.wikipedia.org/wiki/Paper_prototyping) is older than most software, and it still works. Sketch on paper badly and quickly. Draw more versions than you think you need, and show them to people before you like them too much. The design tool will still be there when you know what you are designing.

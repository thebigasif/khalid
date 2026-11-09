---
title: "How we write a one-page spec"
slug: one-page-spec
date: 2026-03-03
reviewed: 2026-10-08
author: greta-voss
reviewed_by: ada-kerrow
category: how-we-work
description: How the Tern team writes a one-page spec, the six parts every spec has, and why we stop writing when the page is full.
focus_keyword: one-page spec
tags: [planning, writing]
image:
  src: lead.jpg
  alt: A closed notebook with a grey cloth cover lying on a pale wooden desk, a silver pen resting across the top of it
---

Every spec we write is a one-page spec: six short parts, in plain sentences, that anyone on the team can read in five minutes. If the idea does not fit on one page, it is too big for one cycle, and we split it before anyone starts work. The limit is the point. It forces us to decide what matters before we argue about how to build it.

We are a team of nine. We do not have time for thirty-page documents that nobody reads twice, and we have learned that a long spec hides its open questions in the middle. A one-page spec puts them at the top, where they get answered.

## The six parts of a one-page spec

Every spec at Tern has the same six parts, in the same order. The order matters: the first three say why, and only the last three say what.

| Part | What it answers | Typical length |
|---|---|---|
| The problem | What goes wrong today, for whom, and how we know | Three to five sentences |
| Who it is for | The kind of person who will notice the change | One or two sentences |
| What good looks like | How we will know, a month later, that it worked | Two or three measurable lines |
| The shape of the answer | The idea, in words and one rough sketch | A short paragraph and a drawing |
| What we will not do | The tempting parts we are leaving out | A short list |
| Open questions | What we do not know yet, and who will find out | A short list with names |

The problem comes from real evidence: support emails, interview notes or numbers from the app. "People want X" is not evidence. "Rafe answered 41 emails in March from people who lost a note they had typed on the train" is.

## Why the sketch is drawn by hand

The shape of the answer always includes one sketch, and we draw it by hand on paper, then photograph it. A rough drawing invites changes. A polished [mock-up](https://en.wikipedia.org/wiki/Mockup) invites approval, and approval too early is the most expensive mistake a small team can make. Ines explains this at length in [why we sketch on paper first](/blog/sketch-on-paper/).

:::figure{src="pencil-notes.jpg" alt="A hand holding a yellow pencil drafts a one-page spec on a lined spiral notebook, with fresh pencil shavings scattered across the open page"}
The sketch in a spec is allowed to be ugly. It only has to show the idea well enough for someone to disagree with it.
:::

## The part people skip: what we will not do

"What we will not do" is the most useful part of a one-page spec, and the one new team members leave empty. It names the ideas that came up while writing and were left out on purpose. It stops the build from growing quietly, one reasonable extra at a time.

When we planned offline search, this list said we would not search inside attached files, would not rank results by how often a note is opened, and would not add search to the quick capture window. All three were good ideas. None of them fitted in six weeks. Two of them are now in later specs.

## How we review a spec

A spec is ready for review when its author can read it aloud in under five minutes. We review it in writing first: everyone adds comments over one day, and the author answers each one or turns it into an open question. Then we meet for twenty minutes, only to settle what the comments did not.

We check four things in review:

- **Is the problem real?** Is there evidence, and is it recent?
- **Is the success line measurable?** Could we tell in a month whether it worked?
- **Is the "will not do" list honest?** Does it name the parts someone will ask for?
- **Does every open question have a name beside it?** A question with no owner stays open.

:::callout{kind=tip}
If two people in review describe the problem in different words, the spec is not ready. Rewrite the first part until everyone tells the same story, then look at the rest.
:::

## What happens after the spec

An approved spec goes into the next [six-week cycle](/blog/six-week-cycles/) and stays on one page for the life of the work. When we learn something that changes the plan, we edit the page and add a dated line at the bottom saying what changed and why. At the end of the cycle, the page becomes the first draft of the release note.

We have written 31 specs this way since the start of 2025. The longest was still one page, in a slightly smaller font, and we have since agreed that does not count.

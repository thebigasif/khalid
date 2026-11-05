---
title: "How we name features"
slug: naming-features
date: 2026-08-25
author: ines-halloran
reviewed_by: rafe-morrow
category: essays
description: How the Tern team names features with plain words people already use, the test every name must pass and the names we threw away.
focus_keyword: name features
tags: [design, writing]
image:
  src: lead.jpg
  alt: Dozens of small origami cranes and folded paper shapes in red, blue, yellow and green scattered along a long wooden table by a window
---

We name features with the plainest words that describe what they do, in the words people already use for them. A name has to pass one test: someone who has never used Tern should guess what the feature does from its name alone. That is why Tern has "quick capture", "version history" and "daily notes", and none of the clever names we were tempted by.

Naming sounds like a small job. It is not. A feature's name appears in menus, help pages, support emails and release notes for years. A bad name costs a little every time someone reads it and does not understand.

## The test we use to name features

Every proposed name goes through the same test. Rafe shows it, with no other context, to five people who do not use Tern, and asks one question: "What do you think this does?" A name passes if at least four of the five give an answer close enough to the truth.

| Name we tried | What people guessed | Result |
|---|---|---|
| Spark | A notification, a fire, a way to start a chat | Failed |
| Inbox | Email, messages from other people | Failed |
| Quick capture | Saving something fast, a screenshot | Passed |
| Rewind | Video controls, a music player, undo | Failed |
| Version history | Old versions of a note, undo | Passed |
| Today page | A calendar, the weather, news | Failed |
| Daily notes | A note for each day, a diary | Passed |

"Capture inbox" is the one place we kept "inbox". On its own, the word made people think of messages. Beside "capture", it made sense to four people out of five, and we accepted that.

## Words people already use

The best source of names is our support inbox. When people describe a feature in their own words, those words are the name. Nobody wrote to ask for "temporal note states". Dozens wrote to ask how to "get back the old version" of a note. The feature became version history.

We keep a short list of these words in a shared note, and anyone [answering support email that week](/blog/everyone-answers-support/) can add to it. When we name features, we look there first.

## Rules we follow

- **Use a verb or a plain noun.** "Link a note" is clearer than "Linkage".
- **Avoid metaphors.** A [metaphor](https://en.wikipedia.org/wiki/Metaphor) needs explaining, and explaining is the job the name was meant to do.
- **Avoid our own jargon.** Engineers call the search index "the shard". Users see "search". The two must never meet.
- **Keep names short enough for a button.** If a name does not fit on a phone button, it will be cut, and the cut version will be worse.
- **Never rename without a strong reason.** Every rename breaks help pages, habits and screenshots on the web.

:::callout{kind=warning}
Be careful with names that are fun. Fun names are easy to remember and hard to understand. They also age badly: a joke that delights in the first week can feel tired by the fifth year.
:::

## Names we got wrong

We have renamed two features since 2022, and both taught us to name features more carefully. "Threads" became "linked notes" in [Tern 2.0](/blog/tern-2-0-linked-notes/), the same release in which [we removed tags](/blog/removed-tags/), because people thought it meant comments. And "Snooze", for hiding a reminder, became "remind me later", because people thought snoozing a note would hide the note itself.

Both renames were right, and both were expensive. We rewrote 40 help pages, answered weeks of confused email and lost an old screenshot or two on review sites. The guessing test exists so we do not have to do that again.

## What naming teaches us about design

A feature that is hard to name is often hard to use. When we cannot find a plain name, it is usually because the feature does two things, or does one thing in an unusual way. More than once, trying to name a feature has sent us back to [sketch it again on paper](/blog/sketch-on-paper/). A good name does not make a feature better, but a missing one tells you something is wrong.

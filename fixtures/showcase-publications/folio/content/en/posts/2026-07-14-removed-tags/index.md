---
title: "What we learned when we removed tags"
slug: removed-tags
date: 2026-07-14
author: ines-halloran
reviewed_by: rafe-morrow
category: essays
description: What we learned when we removed tags from Tern in version 2.0, who missed them, what we got right and what we would do differently.
focus_keyword: removed tags
tags: [design, linking]
image:
  src: lead.jpg
  alt: A small black block plane resting on a worn wooden bench beside two curls of pale wood shaving, a chisel blurred in the background
---

When we removed tags from Tern in February, most people did not notice, about one in twelve missed them, and a few left. Five months on, we think removing them was right, but we did it a little too fast and explained it a little too late. This essay is about what we learned, because removing a feature is harder than adding one, and few teams write about it.

[Tags](https://en.wikipedia.org/wiki/Tag_(metadata)) were in Tern from the first version. In [Tern 2.0](/blog/tern-2-0-linked-notes/) we turned every tag into a note and every tagged note into a link to it. Nothing was deleted, but the tag box was gone, and with it a habit.

## Why we removed tags

We had three reasons, and all three came from our own data and our support inbox.

- **Tags were folders by another name.** People used them to file notes in one place, with the problems I described in [why Tern has no folders](/blog/no-folders/).
- **Tags drifted.** The median account had 41 tags, and a third of them were near duplicates: "garden" and "gardening", "work" and "Work", "todo" and "to do".
- **Tags and links did the same job.** A tag says "this note is about Garden". A link to a note called Garden says the same, and the Garden note can hold your thoughts about the garden too.

## Who missed them

In the first month after Tern 2.0, Rafe tracked every support email about tags. There were 386 of them. They fell into four groups.

| What people said | Share of emails, February to March 2026 | What we did |
|---|---|---|
| "Where did my tags go?" | 52 percent | Clearer note on first open, help page, reply |
| "I liked typing a hash to tag" | 21 percent | Typing a hash now offers to link a note |
| "I need to see all notes with a tag" | 18 percent | Saved searches, coming in Tern 2.3 |
| "I am leaving" | 9 percent | Export help and a personal reply from us |

The first group was a failure of explanation, not design. We had written a long post about the change, but it was not where people looked. They looked in the app, and the app said almost nothing. That is the mistake I most regret.

The second group taught us about habits. People did not love tags; they loved typing a hash in the middle of a sentence without stopping. So we kept the gesture and changed what it does: typing a hash now opens the same note list as two square brackets.

:::callout{kind=note}
The 9 percent who said they were leaving were about 35 people. We wrote back to each of them personally, helped those who wanted to export, and asked what would bring them back. Eleven have since returned.
:::

## What we would do differently

Now that we have removed tags, we know what we would change if we removed a feature again.

1. **Explain it where people meet it.** A short message in the app, at the moment someone reaches for the old feature, is worth more than any blog post.
2. **Keep the gesture.** Remove the concept if you must, but keep the muscle memory working wherever you can.
3. **Ship the replacement first.** Saved searches answer the most serious complaint. They should have shipped with Tern 2.0, not two releases later, in August.

## Why it was still right

After five months, accounts that used tags heavily now write 30 percent (2026) more links than before, and they find notes faster in our search tests. The tag clean-up alone removed thousands of near duplicates. Most telling, new people who never knew Tern with tags do not ask for them at all.

Removing a feature feels like taking something away, and for a few people it is. But every feature has a cost, paid by everyone, every day, in a busier screen and one more decision. Some features are worth that cost. Tags in Tern were not. We removed tags, and saying so out loud has made the app simpler for everyone who came after.

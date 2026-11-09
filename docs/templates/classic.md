---
title: "Classic"
order: 5
---

# Classic

The default look: a lead story and a grid of cards on list pages, a readable column for posts and the contents beside long posts.

```toml
[template]
name = "classic"
```

Without a `[template]` table, your blog uses Classic.

## Who it suits

Classic suits any blog that wants to start writing today and choose a look later. It has no options to set. It takes your colours, fonts and logo from your theme tokens, such as the ones theme sync writes, so it looks like your brand from the first build.

## What each page shows

**Front page**
- A wide frame, 72rem at most, with the blog's name and quiet menu links in one row at the top.
- The announcement, when you set one, as a light tint of your accent colour across the full width above the header, with the words in your text colour.
- The blog's name in large type.
- The newest post as a large lead story: its picture beside its topic, author, date, title and summary.
- The other posts as a grid of cards, three across on a wide screen and one on a phone. Each card has its picture in one 16:9 shape, the topic in small capitals above the title, the author and date, and the summary.

**A post**
- The title in large type, over a column of readable width. A short post's column sits in the middle of the frame. A long post with its contents beside it uses the whole frame from 1024 px wide: the text starts under the blog's name and the contents end at the header's right edge.
- The byline: each author's photo, name and role, then the reviewer, the date, the reading time and the topic in one row.
- The lead image under the byline.
- On a wide screen, a long post's contents in a quiet column beside the text, with the section you are reading marked. On a phone, the contents fold into a box above the text.
- After the body: the author box and the related posts.

**List pages**, such as a topic, a tag or an author's posts, open with their own lead story and grid, as the front page does.

Everything reads with scripts off. Only your theme tokens set colours and type, so a synced theme recolours all of it.

## Options

Classic takes no options. To change the layout of a page part, such as where the contents list sits, choose another template in [`docs/templates.md`](../templates.md).

---
title: "Brief"
order: 86
---

# Brief

Brief is a template for a short digest newsletter, sent daily or weekly. Each issue is a handful of tight items: a headline, a line or two on what happened, and a link to the source. Every page is one narrow sheet of paper on a tinted desk, so the site reads like the email, and a reader finishes an issue in about two minutes.

```toml
[template]
name = "brief"
```

## Who it suits

- A curator who sends a daily or weekly round-up of links on one subject.
- A team, club or trade group that sends members a short digest of news.
- A writer who shares what they read, each link with their own one-line take.

It suits long essays less well. A long post still reads well, but the front page is made for short issues.

## Issues are posts

Brief needs nothing new in your posts. Each post is an issue. Write each item as a `##` heading, then one short paragraph that ends on a link to the source:

```markdown
---
title: "Seal pups return to the sandbanks, and the ferry gets a winter timetable"
description: "A record count of seal pups, the winter ferry times and a new tide gauge."
category: morning-brief
facts:
  - label: Issue
    value: "No. 126"
  - label: Items
    value: "5"
---

Five things from the coast this Tuesday.

## Seal pups return to the sandbanks

Counters found 212 grey seal pups this autumn, the most in ten years. [Read the count](https://www.example.org/seal-count)

## The ferry's winter timetable starts on Monday
...
```

- Each `##` heading is an item. Brief numbers the items 01, 02 and on, in large figures in the margin.
- The last link in the paragraph under a heading gets an arrow, so the source is easy to spot. Put the link at the end of that paragraph.
- A short opening line before the first item and a closing line after the last read as plain text.

Facts are optional, and Brief reads them by their place in the list, not by their label:

1. **The issue number,** such as `126` or `No. 126`. It shows as "No. 126" in the stamp line over the issue.
2. **The number of items,** such as `5`. It shows as "5 items" beside one short tick per item.

Brief reads only the digits of each value, so `No. 126` and `126` are the same. A value with no digits shows nothing. See [Facts on a post](../templates.md#facts-on-a-post) for the limits. A post without facts, such as a note from the editor, shows in the same lists without a number or ticks, and it does not end on the sign-off line.

Categories are your editions, such as Morning brief and Week in review. Tags are topics.

## What the front page shows

1. **The masthead:** the newsletter's name, large, and the `tagline` under it, with the sign-up form beside them on a wide screen.
2. **The latest issue:** the newest post with facts (or, when no post has any, the newest post), with its lead picture short and wide, the stamp line (edition, issue number and date), its title and summary, the item ticks with the reading time, and a button to read it.
3. **Past issues:** every other post on the page as a ruled row: the date as a small stamp of weekday, day and month, then the title, the summary, and the edition, issue number, items and reading time.
4. **Editions:** every category as a row of chips.
5. Numbered page links and your call to action as a band.

Later pages show the rows only.

## What a post shows

1. The stamp line: the edition, the issue number and the date.
2. The title, the description as a one-line summary, the writer, and the item ticks with the reading time. A post without an item count shows its reading time in the stamp line.
3. The lead picture, short and wide, if the post has one.
4. The items, each numbered in the margin, in one column of about 68 characters a line. On a phone the number sits over its heading.
5. For an issue, the sign-off line: "That's the brief. See you next issue."
6. The tags, the share links, the author box and the sign-up form.
7. Related issues as rows.

## Other pages

- **Lists** (an edition, a tag, the archive): the list's name and intro, then the issues as rows.
- **Writers** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A writer's page:** their photo or mark, role, experience and bio, then their issues as rows, and the other writers.

## The sheet

Every page is one sheet about 44rem wide, centred on a desk in your surface colour, with a heavy ink edge along its top. The header, the page and the footer all sit on the sheet. On a screen narrower than the sheet, the desk goes and the sheet fills the screen. With the contents beside a long post (`contents = "side-end"` or `"side-start"`), the contents sit on the desk next to the sheet when the window has room, and fold into a box above the post when it does not.

## Your brand

Brief brings its own colours: white paper on a pale grey-green desk, near-black ink and a brick red (`#9c3418`) for links and the item numbers. Your colours, fonts and logo in `theme/tokens.toml` replace them. Your `surface` colour is the desk and the boxes, and your `accent` draws the item ticks and the line over the sign-up form.

Brief's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Brief's default |
|---|---|
| `[fonts] heading` | Helvetica Neue, then Arial and the system's sans |
| `[fonts] body` | Charter, then Georgia and other serifs |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `1.875rem` to `2.5rem` |
| `[type] h2` | `1.25rem` to `1.375rem`, the size of each item's heading |
| `[type] h3` | `1.125rem` to `1.25rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0px`, square |
| `[shape] measure` | `36rem` |
| `[space] section` | `3rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the newsletter's name on the front page, such as what it covers and how often it goes out. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related issues show, from `3` to `10`. |
| `contents` | `"none"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. An issue's items are short, so Brief shows no contents unless you ask. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["email", "bluesky", "linkedin"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the sign-up form shows: `none`, `post`, `home` or `both`. On the front page it sits beside the name. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |

## The demo

The demo is Tideline, a made-up weekday digest about the Selwick coast, a made-up stretch of coast, and the sea beyond it. Every place, group and story in it is invented, and every link goes to a reserved example address.

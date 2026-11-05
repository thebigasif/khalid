---
title: "Ledger"
order: 70
---

# Ledger

A calm, text-led blog with no pictures in its lists, for studies and long how-tos.

```toml
[template]
name = "ledger"
```

## Who it suits

Ledger suits a team that publishes research, data studies and long guides, where the words and the topics matter more than pictures. A reader lands on a front page that reads like a table of contents: the blog's name and what it is about, a row of topics, then the newest posts one under another, each with its author, date and topic. A post opens with its topic, its title and who wrote it, then an open list of its sections.

Pick another template if your posts lead with photos, or if you want a large featured post at the top of the front page.

## What each page shows

**Front page**
- The blog's name in large type under a short accent bar, your tagline, and up to three short facts such as "12,000 readers".
- A row of topics, one for each category your posts use, as small tinted labels called pills.
- The latest posts as a list split by thin lines, numbered 01, 02 and on in the margin: author, date and topic above each title. No pictures and no summaries, so a reader scans many titles at once.
- When `[[reader.home_sections]]` lists sections, five posts for each, with a "View all" link.
- A newsletter band across the full width at the end, when your site takes sign-ups.

**A post**
- The post's topic as a pill above the title.
- The byline: each author's photo, name and role.
- One date: the day it was updated, or the day it was published if it never was. Then the reading time.
- The contents as an open box after the title, without numbers.
- The body. Tips and further reading show as ruled notes, a `summary` callout shows as a closing "bottom line" under a heavy rule, how-to steps get large numbers, and FAQ questions read as plain questions with their answers always shown.
- After the body: the author box, then five related posts in the same ruled list, then the newsletter form.
- The lead image under the byline, in a light frame, unless you turn it off. No share links.

**Lists and author pages** show posts as the same ruled list, each with its summary.

Everything reads with scripts off. Ledger brings its own look: ink green on warm paper, with serif headings. Your theme tokens, such as the ones theme sync writes, replace any of those colours and fonts, and the logo comes from them too.

## Options

Set these under `[template.options]`. Each line shows the default.

| Option | Default | What it changes |
|---|---|---|
| `hero_stats` | none | Up to four numbers under the front page's title, each written `number \| what it counts`. Nothing shows until you write them. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. They work with scripts off. |
| `video_section` | `true` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `true` | A row of the newest posts with a podcast or other sound player, when there are any. |
| `tagline` | empty | One line under the blog's name on the front page, up to 140 characters. |
| `stats` | none | Up to three short facts beside the tagline, such as `["12,000 readers", "Weekly since 2024"]`. |
| `lead_image` | `true` | Whether a post shows its lead image under its title. |
| `logo` | `true` | Whether the header shows your logo from `[brand] logo` in place of the blog's name. |
| `updated_note` | `false` | Whether an updated post says why it was updated, under its date. |
| `cards` | `"text"` | The shape of the post cards on list and author pages: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `false` | Whether each card shows how long the post takes to read. |
| `pagination` | `"numbered"` | The links under a long list: `pages` for newer and older, or `numbered` for a link to each page. |
| `contents` | `"inline"` | Where a post's contents go: `inline`, `inline-folded`, `side-start`, `side-end` or `none`. |
| `reading_progress` | `"bar"` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` or `none`. |
| `related` | `"list"` | How the related posts are laid out: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `5` | How many related posts show, from 3 to 10. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. It needs `[mail]`, so a static site shows none. |
| `cta_band` | `"none"` | Where the band with your `[template.cta]` shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | Whether the header shows your `[template.cta]` as a button. |

A full example:

```toml
[template]
name = "ledger"
[template.options]
tagline = "Studies and long how-tos on search"
stats = ["12,000 readers", "Weekly since 2024"]
related_count = 3
```

## Sizes and spacing

Ledger sets these in place of classic's. A value in your own `theme/tokens.toml` wins.

```toml
[colors]
background = "#fcfbf7"
surface = "#f3efe3"
text = "#1b2420"
muted = "#55605a"
link = "#0d6b4c"
accent = "#0d6b4c"
border = "#dcd6c6"

[fonts]
heading = "\"Iowan Old Style\", \"Palatino Linotype\", Palatino, \"Book Antiqua\", Georgia, serif"

[shape]
radius = "0.375rem"
measure = "42rem"

[type]
body = "1.0625rem 1.125rem"
h1 = "2.25rem 3.25rem"
h2 = "1.5rem 2rem"
h3 = "1.25rem"
small = "0.9375rem"
line_height = 1.65

[space]
unit = "0.5rem"
section = "3rem"
```

## Reader features

Ledger keeps every `[reader]` feature as it is. Turn the topics row off with `topic_links = false`, and comments stay off unless your site runs as a server with comments on.

## Design

This is Ledger's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A team that publishes research, data studies and long how-tos, where the words and the numbers matter more than pictures.

**Reference blogs**, by kind. Their pictures are kept with the design notes, outside the repository.

- A large nonpartisan research centre. Serif headlines over small capital labels with the kind of report and its date, ruled rows of publications with a short summary, a numbered "most read" rail, and a frame near 75rem on a laptop.
- A data publication about the world's large problems. Serif titles over a plain sans text in a narrow column, each chart framed with its title, its key in coloured words and a "Data source" line, and the topic of a page as a small pill.

| Part | Ledger |
|---|---|
| Type pairing | A strong gothic sans for headings over a sturdy text serif. With no theme, the pack uses the system sans for headings and the system serif (Charter, Georgia) for text. |
| Colour | Near-white ground, ink text and one deep, serious accent (petrol or ink green). Panels are a very faint grey. Tables use hairlines, never fills, except the header row. |
| Spacing rhythm | A base step of 0.5rem. Rows in lists 1.25rem apart, separated by hairlines. Sections 4rem apart. |
| Frame at 1440 px and wider | At least **75rem**: the narrowest of the ten, because Ledger leads with text. |
| Cards and lists | No boxes around list items. Each post is a ruled row: a number in the margin, then author, date and topic, then the title, then (on lists) the summary. |
| Phone header and menu | The logo and the menu on one row; the menu wraps under it. |
| Tabs and indexes at 390 px | Topic pills wrap onto lines. The contents box stays open as a plain list. |
| Link, card and focus states | Links underlined in the accent; on hover the underline thickens. A row's title is the target and underlines on hover. Focus is a 2 px accent ring. |
| Signature details | Numbers 01, 02, 03 in the margin of every list. The "bottom line" summary under a heavy rule at the end of a post. Tables with tabular figures, right-aligned numbers and hairline rows. |

**Sample brand.** The demo uses `fixtures/showcase-brands/ledger/theme/`: petrol (`#0d5c66` links, `#0f6c78` accent) on near white (`#fdfdfc`), Libre Franklin for headings and PT Serif for text, and a bar-chart mark.

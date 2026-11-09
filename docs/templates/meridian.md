---
title: "Meridian"
order: 80
---

# Meridian

Meridian is a calm company journal. It suits a product or services company that publishes a few considered posts a month, often written by more than one person, and keeps a handful of long guides that it updates over time.

Pick it when you want the blog to feel quiet and trustworthy: lots of room, large serif titles over a plain sans text, one column to read, hairlines instead of boxes, and one accent colour used only where a reader can act. If you publish many short news items a day, another template will suit you better.

## How to switch it on

```toml
# site.toml
[template]
name = "meridian"
```

Build or restart the blog. Meridian brings its own palette (ink on a warm paper colour, one deep indigo accent) and serif titles. Any value in your own `theme/tokens.toml` wins, so a blog synced with its main site keeps that site's colours, fonts, logo and corners.

## What readers see

**The front page**

- Your logo in the header, the menu at its end and your call to action as an outlined button, on one line on a wide screen. On a smaller screen the menu takes its own row; on a phone it is one row that scrolls, and the button gives way to the band further down.
- The blog's name as a very large serif title over a heavy ink rule, like a journal's masthead, with the intro from your front page's `_index.md` beside it when you write one.
- A row of tabs under the rule: "All", then each category your posts use. On a phone the tabs become small rounded buttons that wrap.
- The newest post as the lead story: a large picture beside a large title, its summary and its author.
- The next two posts side by side with large pictures and titles, when the page has at least five posts.
- Editor's picks: up to three posts you marked `featured: true`, numbered 01, 02, 03 in the serif, as short text cards on a soft band across the page.
- Every other post as a wide row, like an index: on a wide screen the category in small capitals, the date and the author with their role sit in a narrow column at the start, then the title and the summary, with the post's picture cropped to 3 by 2 on the far side. On a phone the picture sits above the text.
- When `[[reader.home_sections]]` lists sections, they take the place of these rows. Each section opens like a section of a magazine: its name set very large in the serif over a thin ink line, then its posts and a "View all" link.
- Tabs by topic, when `topic_tabs` is on: one tab per topic, each with its four newest posts. Each panel opens the same way as a section, with the topic's name set very large in the serif over a thin ink line. Names of one word, such as "Housing" or "Transit", suit this best. A long word breaks across lines rather than running off a phone screen.
- A newsletter form, when your blog takes sign-ups (see below), and numbered page links under long lists.

**A journal post** (every post, unless it asks for the reference layout)

- Breadcrumbs as one small label: the blog and the post's category.
- A large serif title, then the authors with their round photo, name and role on one side and the dates on the other, between two hairlines.
- On a wide screen a post with a cover opens split across the page: the title and byline on one side, the cover on the other, over a heavy ink rule. On a smaller screen the cover sits under the byline, a little wider than the text.
- Then one quiet column of text. Callouts are soft panels or a thin rule, quotes are centred, steps are numbered in circles and questions open and close with a plus.
- After the post: the author box, the call-to-action band, the newsletter form, the posts that link here, then three related posts with their pictures across the page.

**A reference guide** (a post with `template: reference` in its front matter)

Use this for long guides you keep up to date.

- Breadcrumbs and the title. No byline at the top: a guide speaks for the company.
- One line with "Last updated" and its date, and your call to action as a button.
- The contents, open, numbered 1, 2, 3 for sections and 1.1, 1.2 for their parts. A guide needs at least three sections (`##` headings) for the contents to show.
- After the guide: three related articles, then a closing band with two buttons: your call to action and "All articles".

Three blocks fit a guide well:

```markdown
:::callout{kind=summary}
- What the guide answers, in one line each.
:::

::::columns
:::column{title="Free plan"}
For one blog.
:::
:::column{title="Team plan"}
For a team with several authors.
:::
::::

:::callout{kind=note}
A disclaimer, or anything a reader should check first.
:::
```

**An author page** shows the author's round photo beside their name and role, their bio and profile links, then their posts as the same wide rows.

## Options

Set any of these under `[template.options]` in `site.toml`. A line you leave out keeps Meridian's default.

| Option | Meridian's default | What it changes |
|---|---|---|
| `hero_stats` | none | Up to four numbers under the front page's title, each written `number \| what it counts`. Nothing shows until you write them. |
| `topic_tabs` | `true` | Tabs on the front page, one per topic, each with its four newest posts. They work with scripts off. |
| `video_section` | `true` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `true` | A row of the newest posts with a podcast or other sound player, when there are any. |
| `logo` | `true` | The header shows your logo from `[brand] logo`. Without a logo, the blog's name shows. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the front page to it. |
| `dek` | `false` | A post shows its `description` as a subtitle under its title. |
| `cards` | `row` | The shape of post cards in lists: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_author_role` | `true` | Each card shows its author's role after their name. |
| `category_tabs` | `true` | The front page shows the row of category tabs. |
| `picks` | `3` | How many posts marked `featured: true` the front page shows as editor's picks, from 0 for none to 8. |
| `pagination` | `numbered` | The links under a long list: `pages` for newer and older, or `numbered` for a link to each page. |
| `related` | `scroll` | How related posts are laid out under journal posts: `grid`, `list`, `row`, `scroll` or `links`. Meridian shows `scroll` as three cards side by side on a wide screen. |
| `related_count` | `3` | How many related posts show, from 3 to 10. |
| `contents` | `none` | Where a journal post's contents go: `side-end`, `side-start`, `inline`, `inline-folded` or `none`. A reference guide always shows its contents open. |
| `reading_progress` | `bar` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `button` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `contents_numbers` | `nested` | How a journal post's contents are numbered when you turn them on: `numbered`, `nested` or `none`. |
| `share` | `none` | Where a post shows share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["linkedin", "x", "email"]` | Which networks the share links go to, in order. |
| `newsletter` | `both` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `post` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | Whether the header shows your call to action as a button. |

For example, to turn the tabs off and show five editor's picks:

```toml
[template.options]
category_tabs = false
picks = 5
```

## Reader features Meridian changes

Meridian turns `reading_time` off by default, so the byline stays short. Write it in `[reader]` to turn it back on. Your line always wins. The author box and related posts stay on. A reference guide shows its three related articles either way.

## Your call to action and newsletter

The header button, the band after each post and the guide's buttons all show the call to action you write once:

```toml
[template.cta]
label = "Book a demo"
href = "https://example.com/demo/"
note = "See it on your own site."
```

Without `[template.cta]`, none of them shows. The newsletter form shows only on a blog that can take sign-ups: `mode = "server"` with a `[mail]` table.

## Sizes and spacing

Meridian sets these in place of the shared defaults. A value in your own `theme/tokens.toml` wins.

| Token | Meridian's value |
|---|---|
| `[fonts] heading` | Iowan Old Style, Charter, Sitka Text, Cambria or Georgia: a serif every computer already has, so nothing is downloaded |
| `[fonts] body` | The computer's own sans: system-ui, Segoe UI, Roboto, Helvetica Neue or Arial |
| `[type] body` | 1.0625rem on a phone to 1.1875rem on a wide screen |
| `[type] h1` | 2.75rem to 5rem |
| `[type] h2` | 1.75rem to 2.25rem |
| `[type] h3` | 1.25rem to 1.5rem |
| `[type] h4` | 1.125rem to 1.25rem |
| `[type] small` | 0.875rem |
| `[type] line_height` | 1.7 |
| `[type] heading_weight` | 600 |
| `[space] section` | 6rem between sections |
| `[shape] radius` | 0.375rem corners |
| `[shape] measure` | 42rem, about 70 letters a line |

The page itself is up to 80rem wide, so rows have room; the text of a post keeps to the 42rem measure. The blog's name on the front page grows to 7.5rem on a wide screen, and the lead story's title to 4rem.

## Colours

| Token | Meridian's value |
|---|---|
| `background` | `#fbf9f4`, a warm ivory |
| `surface` | `#f2eee4`, for bands and panels |
| `text` | `#16181d`, ink |
| `muted` | `#57585c` |
| `link` and `accent` | `#2b3c96`, a deep indigo |
| `border` | `#e0d9cb`, a warm grey |

The accent appears only on links, small labels, buttons, the current tab and the short rule before each label. Buttons pick black or white text, whichever reads better on the accent. Every pair passes WCAG 2.2 AA.

## What Meridian does not do yet

- Each post has one call to action. Several different banners per site are a later step.
- The footer shows the footer menus only; it cannot show the blog's name or logo without a change to the shared page frame.

## Design

This is Meridian's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A product or services company that publishes a few considered posts a month and keeps some long guides up to date.

**Reference blogs**, by kind. Their pictures are kept with the design notes, outside the repository.

- A team software company's journal. A large lead story beside a two by two grid of smaller stories, small "article in" labels with the topic as an outlined pill, "Latest by topic" as a row of pill tabs, and a sign-up card placed inside the grid.
- A file-sharing company's journal. A lead story with a very tall title beside a large picture, then an editorial grid whose rhythm changes row by row: two large stories, then three smaller ones set at different heights, each with its topics as one quiet line under the title. Its dark lead band is not part of this brief.

**The one idea.** A journal's masthead. You can name Meridian from a thumbnail of its front page: the blog's name set very large in the serif, from 2.375rem on a phone to 7.5rem on a wide screen, with tight letter spacing, over a heavy ink rule. Every list and post repeats that ink rule, and the rows below read as the journal's index.

| Part | Meridian |
|---|---|
| Type pairing | A high-contrast display serif for titles over a clean sans for text. With no theme, the pack uses the system's book serif for titles (Iowan Old Style, Charter, Georgia) and the system sans for text. |
| Colour | A warm ivory ground, ink text and one deep accent (plum or indigo) used only where a reader can act. Hairlines in a warm grey. |
| Spacing rhythm | A base step of 0.5rem. Very generous: 6rem between front page sections, 2.5rem between rows. |
| Frame at 1440 px and wider | At least **80rem**: wide rows need the room. |
| Front page structure | The masthead, the tabs, the lead story split beside its picture, two large stories side by side, the numbered editor's picks on a soft band, six rows, then every other post in a text-only index: the topic, the serif title and the date on one quiet line. The index lists every post, since the front page is the path to each one. The rhythm changes from one part to the next, so the page never reads as one long list. |
| Cards and lists | Wide rows, not cards: on a wide screen a rail at the start holds the category in small capitals, the date and the author with their role, then a large serif title and the summary, and the picture cropped to 3 by 2 on the far side. Rows sit between hairlines. Topic, video and podcast rows are grids of picture cards. |
| Phone header and menu | The logo and the menu, the menu as one row that scrolls. The outlined button gives way to the band further down. |
| Tabs and indexes at 390 px | The category tabs become small rounded buttons that wrap. A row is compact: a small square picture beside the title and the date, with the summary left for the post. The lead story keeps its large picture. |
| Link, card and focus states | Links underlined in the accent. A row's serif title takes the underline on hover. Focus is a 2 px accent ring. |
| Post page at 1440 px and wider | The text grows to 1.25rem on a 44rem measure, about 70 characters a line, and charts, pictures and quotes step out to a 60rem column, so the window never reads as half empty. |
| Signature details | The masthead over its ink rule. Each topic panel and home section opens on its one-word name, set from 2.75rem on a phone to 9.5rem on a wide screen, over a hairline. The lead story with a large picture beside a large serif title, and a post's split header the same way. Editor's picks numbered 01 to 03 in the serif on a soft band. The reference guide layout with "Last updated" and numbered contents. |

**Sample brand.** The demo uses `fixtures/showcase-brands/meridian/theme/`: plum (`#7b2d6a` links, `#8a3378` accent) on warm ivory (`#fbf8f5`), DM Serif Display for titles and Work Sans for text, and a globe mark.

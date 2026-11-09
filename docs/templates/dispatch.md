---
title: "Dispatch"
order: 87
---

# Dispatch

Dispatch is a template for an email newsletter whose website has one first job: to win sign-ups. The front page opens on the sign-up, with your promise and the form, beside the latest issue. Every issue carries its number in a small postage stamp, and a sign-up box follows every issue and every list.

```toml
[template]
name = "dispatch"
```

## Who it suits

- A writer or a small team who sends a weekly or monthly newsletter and keeps every issue on the web.
- A club, shop or local group whose site mostly exists to grow a mailing list.
- Anyone who measures their site by new subscribers, not by page views.

It suits a magazine or a news site less well. For those, Atlas or Wire read better. For a newsletter about markets with figures and charts, Ticker fits better.

## Issues are posts

Dispatch needs nothing new in your posts. Each post is an issue. Give it one fact, its number, and the number shows in the issue's stamp:

```yaml
---
title: "Garlic goes in now: twelve cloves in a balcony box"
description: "Why autumn is the time to plant garlic, and how to grow twelve bulbs in one balcony box."
category: growing-food
facts:
  - label: Issue
    value: "178"
image:
  src: lead.jpg
  alt: A basket of white garlic bulbs
---
```

Only the first fact shows, so keep its value short, such as `178` or `No. 12`. An issue without facts shows the month and day it was sent in its stamp. See [Facts on a post](../templates.md#facts-on-a-post) for the limits.

Categories are your topics, such as growing food or reader questions. Tags are smaller themes.

## The sign-up

On a served site with `[mail]` in `site.toml`, every sign-up box holds Khalid's own email field and button, and new readers join your list. See [Newsletters](../newsletters.md).

A static site cannot take a sign-up. There, each box shows your call to action as its button. Point it at your newsletter's page or at the sign-up page of the service you send with:

```toml
[template.cta]
label = "Subscribe free"
href = "https://www.example.com/newsletter/"
note = "Free, every Saturday. Leave any time."
```

Under the button, each box shows your call to action's `note`, or without one, when the letter goes out from `cadence`. It also links to the feed unless `feed_link` is `false`.

## What the front page shows

1. **The sign-up first.** When the letter goes out (`cadence`) as a small label, your newsletter's name large, its promise (`tagline`), up to three lines on what each issue holds (`promises`), then the reply slip: the sign-up box, a ruled card with a dashed cut line around it.
2. Up to four proof numbers from `hero_stats`, such as readers and issues sent. Nothing shows until you write them.
3. **The latest issue** beside the sign-up on a wide screen and under it on a phone: its picture, its stamp, the date and topic, the title, the summary and a link to read it.
4. **Recent issues** as ruled rows: the stamp, the date and topic, the title, the summary and a small picture.
5. Numbered page links. Later pages keep the sign-up at the top and list their issues.
6. **Topics:** every category as a plain link.
7. A second sign-up box at the end, for readers who scrolled through the issues first. `newsletter` sets where the boxes after the top one show.

## What an issue shows

1. The stamp with the issue's number, "Sent on" and the date, and the topic.
2. The title, the post's description as a one-line summary, and the byline.
3. The lead picture, then the issue in one column of about 70 characters a line.
4. **The sign-up box** straight after the text: "Get the next issue by email".
5. The tags, the share links and the author box.
6. More issues as rows with their stamps.

## Other pages

- **Lists** (a topic, a tag, the archive): the list's name and intro, the issues as rows with their stamps, then the sign-up box.
- **Writers** (`/authors/`): everyone in a grid with their mark or photo, role and how many issues they wrote.
- **A writer's page:** their mark or photo, role and bio, then their issues as rows, and the other writers.

## Your brand

Dispatch brings its own colours: a postal red (`#b02a1f`) for the stamps, buttons and links, ink on warm letter paper, and a deeper paper surface for the sign-up box. Your colours, fonts and logo in `theme/tokens.toml` replace them. A strong serif for headings suits it best.

Dispatch's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Dispatch's default |
|---|---|
| `[fonts] heading` | Iowan Old Style, then Palatino and other book serifs |
| `[fonts] body` | The system's own sans, for the text and the form |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.25rem` |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.25rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `400` |
| `[shape] radius` | `0.25rem` |
| `[shape] measure` | `36rem` |
| `[space] section` | `4rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | The newsletter's promise in one line, under its name on the front page. |
| `cadence` | none | When the letter goes out, such as "Every Saturday morning". It shows above the name, and in every sign-up box when your call to action has no `note`. |
| `promises` | none | Up to three short lines on what each issue holds, each with a tick, under the promise. |
| `feed_link` | `true` | Every sign-up box also links to the feed. |
| `hero_stats` | none | Up to four numbers under the sign-up, each written `number \| what it counts`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | An issue shows its description under its title. |
| `updated_note` | `false` | An updated issue says why it was updated, from its `updated_note`. |
| `cards` | `"row"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many more issues show under an issue, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long issue's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where an issue shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["email", "bluesky", "linkedin"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the sign-up boxes after the top one show: `none`, `post` after each issue, `home` at the end of the front page, or `both`. Lists show one unless it is `none`. |
| `cta_band` | `"none"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. The sign-up boxes already carry it. |
| `cta_button` | `true` | The header shows your call to action as a button. |

## The demo

The demo shows The Saturday Gardener, a made-up weekly letter about growing food and flowers in small spaces, with ten issues, two writers and an about page. Its photographs come from the WordPress photo directory under CC0, and its photo credits page lists each one.

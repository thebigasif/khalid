---
title: "Ticker"
order: 87
---

# Ticker

Ticker is a template for a markets newsletter. It leads with numbers: a dark tape of figures across the front page, each issue's moves as a bar chart, and every list as a data table shaded by how far each figure moved. Every chart is HTML and CSS, so it draws with scripts off and reads as a list of numbers to a screen reader.

```toml
[template]
name = "ticker"
```

## Who it suits

- A weekly or daily markets letter on stocks, rates, currencies or commodities.
- A finance writer who explains how markets work, with tables in the posts.
- A trade body or research desk that publishes a regular round-up of figures.

It suits a blog without numbers less well. For a newsletter led by words, Bulletin or Edition read better.

## Issues are posts with facts

Ticker needs nothing new in your posts. A post with `facts` is an issue. Each fact is one market and its move:

```yaml
---
title: "Factory orders lift shares in a strong week for small firms"
description: "Factory orders rose for a third month, and shares had their best week since June."
facts:
  - label: Stocks index
    value: "+1.8%"
  - label: Small firms
    value: "+2.6%"
  - label: The currency
    value: "+0.3%"
  - label: Copper
    value: "+1.1%"
---
```

- A value that is a plain number draws a bar. It may start with `+` or `-` and may hold `%`, `bp`, `pts`, a currency sign or thousands commas, such as `+1.8%`, `-12bp` or `1,204`. Any other value shows as text, without a bar.
- A value starting with `+` counts as up and one starting with `-` as down.
- Keep the same labels in the same order in every issue. The issue table then lines up week by week, one column per market. Its column headings come from the newest post with facts on the page.
- Give every fact the same unit, such as a change in per cent. Bars are scaled to the largest value in the chart, so a figure in points beside figures in per cent draws out of scale.

See [Facts on a post](../templates.md#facts-on-a-post) for the limits. A post without facts, such as an explainer, shows in the same lists with empty figure cells and has no chart.

Categories are your desks, such as Weekly wrap, Stocks or Rates. Tags are topics.

## What the front page shows

1. **The tape:** a dark band of up to four figures, each with a drawn mark, a triangle up, a triangle down or a bar for flat. It shows your `hero_stats` lines, written `+1.8% | Stocks index`, or, without them, the latest issue's facts.
2. Your newsletter's name, large, and the `tagline` under it, with the sign-up form beside them on wide screens.
3. **The latest issue:** the newest post with facts, with its desk, title, summary, date and writer, beside its facts as a bar chart.
4. **Recent issues:** every other post on the page as a data table: title and desk, date, then one column per fact. Each figure cell is shaded by how far it moved beside the largest move in its column, up in your accent colour and down in ink.
5. **Desks:** every category as a row of links.
6. Numbered page links, your call to action as a band, and the `disclaimer` line.

Later pages show the table only.

## What a post shows

1. The trail from the home page to the post's desk.
2. The desk as a small label, the title, the description as a one-line summary, the writer and the date.
3. **This issue in numbers:** the facts as a bar chart. Up bars are filled with your accent colour. Down bars are hatched in ink and grow the other way from a centre line, so a reader can tell them apart without colour. The `disclaimer` line sits under it.
4. The lead picture, then the post in one column about 36rem wide.
5. Tables in the post are data tables: a heavy rule over the head, figures in even-width numbers at the end of each cell, and a first column that stays in view while a wide table scrolls.
6. The tags, the share links and the author box.
7. Related posts as a small issue table.

## Other pages

- **Lists** (a desk, a tag, the archive): the list's name and intro, then the posts as the issue table.
- **Writers** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A writer's page:** their photo or mark, role, experience and bio, then their posts as the issue table, and the other writers.

## A note on advice

If your newsletter talks about investments, the law where you live may ask you to say that it is not advice. Set the `disclaimer` option, and Ticker puts that line under every chart and at the foot of the front page.

## Your brand

Ticker brings its own colours: a deep green accent (`#0b6b4f`) for moves up, ink for moves down, on a warm off-white page. Your colours, fonts and logo in `theme/tokens.toml` replace them. Pick an accent that reads on your background, since up bars and shaded cells use it. A monospace in `[fonts] mono` sets every figure; one with even-width digits keeps columns straight.

Ticker's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Ticker's default |
|---|---|
| `[fonts] heading` | Helvetica Neue, then Arial and the system's sans |
| `[fonts] body` | Charter, then Georgia and other serifs |
| `[fonts] mono` | The system's monospace |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2rem` to `3rem` |
| `[type] h2` | `1.375rem` to `1.75rem` |
| `[type] h3` | `1.125rem` to `1.25rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0px`, square |
| `[shape] measure` | `36rem`, about 68 characters a line |
| `[space] section` | `4rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the newsletter's name on the front page. |
| `disclaimer` | none | One line under every chart and at the foot of the front page, such as "Nothing here is investment advice." |
| `hero_stats` | none | Up to four figures for the tape, each written `figure \| what it is`. Without them the tape shows the latest issue's facts. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `5` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["bluesky", "linkedin", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. On the front page it sits beside the name. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |

## The demo

The demo is Tallyline, a made-up weekly letter about the markets of Varnholm, a made-up country. Every index, company, price and move in it is invented, and nothing in it is investment advice.

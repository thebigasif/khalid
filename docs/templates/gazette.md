---
title: "Gazette"
order: 85
---

# Gazette

Gazette is a template for a local news site with many sections, such as council, schools, sport, business and what's on. The front page shows every section at once, each in its own block under a bold section flag, with the newest stories running down a rail beside them. Headlines are packed close, and each one can carry the street or village it happened in.

```toml
[template]
name = "gazette"
```

## Who it suits

- A town paper or a local news site that publishes every day across many sections.
- A small newsroom where each reporter covers one or two beats.
- A parish, district or neighbourhood site that posts notices, results and events as well as news.

It suits a site led by its columnists less well. For that, Wire reads better.

## Sections and places

Gazette needs nothing new in your posts.

- **A section is a category.** List your categories in `categories.toml`. The front page shows a block for up to six sections, in the order of their newest story.
- **A story's place is its first fact.** Give the street, village or ward as the first fact, and it shows as a small label before the headline everywhere, and as the dateline at the start of the story. A story without facts shows no place.

```yaml
---
title: Mill Lane bridge to close for six weeks of repairs
category: council
facts:
  - label: Where
    value: Mill Lane
---
```

- **The top story** is the newest post with `featured: true`. With none, it is the newest post.

## What the front page shows

1. **The edition bar:** the front page's title, then the date of the newest story and your `tagline` at the end of the same line.
2. **The section index:** one link per section, each jumping to its block further down. On a phone the row scrolls sideways.
3. **The top story:** its picture beside its place or section, the headline large, the summary, the writer, the date and the reading time.
4. **The section blocks,** two across on a wide screen. Each opens with the section flag: the section's name in reverse on a heavy rule, linked to the section, with an "All Sport stories" link at the rule's end. Then the section's newest story with a small picture and summary, and up to three more as headlines with their places. The top story is not repeated in its section.
5. **The latest news rail,** beside the top story and the blocks on a wide screen and under the top story on a phone: the newest stories (option `rail_count`), grouped under the day they came out, each a place and a headline. The newsletter form closes the rail.
6. The numbered page links and the call to action band.

Later pages of the front page list their stories as headlines grouped by day, each with its place and summary.

When `site.toml` sets home sections (`[[reader.home_sections]]`), each one is a block in place of the categories.

## What a story shows

1. The section flag, linked to the section.
2. The headline and its subtitle, from the post's `description`.
3. The writer's row: photo, name and role, the date, and when it was updated.
4. The lead picture with its caption under it as the credit line.
5. The dateline: the place in small capitals, opening the text.
6. The text in a reading column of 60 to 75 characters a line.
7. The tags, the share links and the author box.
8. **More news:** related stories as headlines with their places and summaries.

## List pages

A section, tag or archive page shows its name under a heavy rule with its intro, the newest story as a lead with its picture, then the rest as headlines grouped by day, in two columns on a wide screen. A reporter's page shows their photo, role and bio, then their stories grouped by day.

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line in the edition bar on the front page, such as the towns you cover. |
| `rail_count` | `10` | How many of the newest stories the latest news rail lists, from `4` to `20`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A story shows its description under its headline. |
| `updated_note` | `false` | An updated story says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `featured` | `true` | The newest featured post is the top story. Off, the newest post is. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `5` | How many related stories show under More news, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long story's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a story shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["facebook", "bluesky", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. On the front page it closes the rail. |
| `cta_band` | `"home"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |

## Design

The demo is The Marlbeck Courier, the made-up paper of Marlbeck, a made-up market town on a river. Its town, council, schools, clubs, businesses and people are all made up. Its sample brand is a deep green (`#17603e`) on white, Domine for headlines and Mulish for the text, labels and flags. Both fonts are under the SIL Open Font License.

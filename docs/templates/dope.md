---
title: "Dope"
order: 34
---

# Dope

Dope is a template for a blog arranged by topic. Its front page opens on a shelf of large topic tiles, so a reader picks a subject before a post. Lists show wide rows with a big picture beside each summary, and a post opens on its picture from edge to edge.

Based on Ghost's Dope theme (MIT).

```toml
[template]
name = "dope"
```

## Who it suits

- A magazine or studio blog with a few strong topics, each with good pictures.
- A blog whose readers come to browse a subject, not to read the newest post.
- Posts with a lead picture worth showing large.

It suits a blog with many small topics less well, since each topic takes a whole tile. For that, Atlas reads better.

## What the front page shows

1. **The header:** your logo or the blog's name on the start side. Search and a square menu button sit on the end side. The menu button opens a white panel from the end side with your menu, over a dark veil.
2. **The topic shelf:** one tall tile per topic, as tall as the window under the header. Each tile shows the newest picture of that topic, with a band in ink under it that holds the topic's name in heavy type and the title of its newest post. The whole tile links to the topic. A short line grows before the name when a reader points at a tile. The shelf slides sideways and snaps to each tile: one tile on a phone, two from 64rem, three from 120rem and four from 160rem. It shows the six topics with the newest posts.
3. **Latest posts:** a large heading, the `intro` of your home page, then the newest posts as wide rows. Each row puts the picture on the start side and, beside it, one line of bold grey meta in lower case (the writer with a round photo, the date, the reading time and the topic), the title in heavy type and the summary.
4. Numbered page links, then each home section, when you have them.

A later page of the front page, or a blog with no topics, shows the blog's name as a large heading over the same rows.

## What a post shows

1. The lead picture, from edge to edge and up to 560 px tall.
2. The topic as a round pill in your accent colour.
3. The title, large and heavy.
4. One line in bold: the writers, the date, when it was updated and the reading time, split by dots.
5. The post in one column of about 65 letters a line. Links are underlined in your link colour.
6. The share links, as a row of round outlined buttons.
7. The author box: a round photo beside the name and bio, with no panel.
8. The newsletter form on a hairline, the call to action band, when it is on, and comments.
9. Three related posts as rows, each with a small picture beside its title.

A post without a picture opens on its topic pill and title.

## List pages

A topic, tag or archive page shows its name large and heavy, the description under it, then its posts in the same wide rows as the front page. An author page puts the round photo beside the name, then the bio and the writer's posts.

## Your brand

Dope takes your colours, fonts and logo from `theme/tokens.toml`, so it looks like your main site. The band under each topic tile is set in your text colour with your page colour for its words. Headings, meta, menus and buttons use your heading font; posts and summaries use your body font.

Dope's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Dope's default |
|---|---|
| `[colors]` | white page, near-black text, a light grey placeholder behind pictures and a soft periwinkle accent `#7f7aa3` |
| `[fonts] heading` and `body` | Raleway when your theme loads it, else the reader's own sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.5rem` |
| `[type] heading_weight` | `800` |
| `[type] line_height` | `1.6` |
| `[space] section` | `5.5rem` |
| `[shape] radius` | `0rem`: square corners; the topic pill and share buttons are round |
| `[shape] measure` | `38rem`, the width of a post's words |

Dope downloads no fonts itself. Raleway shows only when your theme ships it, as `khalid theme sync` does when your main site uses it.

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"row"` | The shape of the cards on list pages: `row`, `image`, `text`, `compact` or `ranked`. |
| `card_reading_time` | `true` | Each card shows how long the post takes to read. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `contents` | `"none"` | Where a long post's contents go: `none`, `inline`, `inline-folded`, `side-end` or `side-start`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` for 1, 1.1, or `none`. |
| `related` | `"row"` | How related posts are laid out: `row`, `list`, `grid`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | Which share links show, in this order. |
| `newsletter` | `"post"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"none"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a square button. |
| `hero_stats` | none | Up to four numbers under the "Latest posts" heading, each written `number \| what it counts`. |
| `topic_tabs` | `false` | Tabs under the latest posts, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `false` | A row of the newest posts with a podcast or other sound player, when there are any. |

The call to action comes from `[template.cta]`; see [the templates guide](../templates.md#your-call-to-action).

For example, a site that wants the contents beside long posts and no share links:

```toml
[template]
name = "dope"
[template.options]
contents = "side-end"
share = "none"
```

## Good to know

- Everything works with scripts off. Dope adds no script: the shelf scrolls and snaps with CSS, and the menu opens with a plain button.
- A topic tile takes the picture of the topic's newest post. Give each topic's newest post a lead picture, or the tile shows a plain grey area over its band.
- Ghost's topic descriptions are not part of Dope. Each tile names the topic's newest post in their place.
- Ghost's members, paid tiers, comments, recommendations and search are not part of Dope. Khalid's own newsletter form, comments and search take their places.
- Ghost loads more posts as a reader scrolls. Dope uses numbered pages, and the archive at `/archive/` lists every post.
- The menu panel holds your menu only. Ghost's panel also listed recent posts and social links; in Dope the front page and your footer carry those.

## Design

This is Dope's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A magazine or studio blog with a few strong, picture-led topics. The first goal is that a reader picks a topic; the second is reading a post in it.

**Reference.** Ghost's Dope theme: a front page made only of topic tiles as tall as the window, topic pages of wide rows with a 500 px picture beside each summary, and a post that opens on a full-width picture.

| Part | Dope |
|---|---|
| Type pairing | One heavy geometric sans (weight 800) for headings, meta, menus and buttons. The pack sets it for text too; a sample brand may pair it with a serif for text, as Dope's "Elegant serif" setting does. |
| Colour | White ground, near-black ink, a light grey placeholder behind every picture, and one soft accent on the topic pills. The band under each tile is in ink. Meta is bold grey. |
| Spacing rhythm | A base step of 0.5rem. Rows 75 px apart on a wide screen, list heads 4rem above and below. |
| Frame at 1440 px and wider | The header across the whole window. Lists in **75rem** (1200 px, the frame the layout check holds), posts in the measure. The shelf and a post's lead picture run from edge to edge. |
| Cards and lists | Wide rows, not cards: a 500 px picture in a 8 to 5 frame with square corners, then the meta, a heavy 28 px title and the summary. Related rows use a 300 px picture and no summary. |
| Phone header and menu | The logo, search and the square menu button on one row, 80 px tall. The menu opens as a panel from the end side. |
| Tabs and indexes at 390 px | One tile per screen in the shelf; rows stack, the picture over the text. |
| Link, card and focus states | Links and titles fade a little on hover. A tile's picture grows a little and a line grows before its name. Focus is a 3 px accent ring, inside the tile on the shelf. |
| Signature details | The tall topic shelf with its ink bands. The square two-line menu button and the end-side panel. Bold lower-case meta split by dots. The round accent topic pill above a post's title. |

**Sample brand.** The demo uses `fixtures/showcase-brands/dope/theme/`: deep plum (`#8a2f6b` links, `#9c3a7b` accent) on white with a faint plum surface (`#f7f2f5`), Raleway for headings and PT Serif for text, and a mark of four squares.

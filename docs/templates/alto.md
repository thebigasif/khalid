---
title: "Alto"
order: 15
---

# Alto

Alto is a template for a clean, minimal blog or personal journal. The front page opens with a strip of square cards for the posts marked featured, which scrolls sideways. Then every post follows, one under another, each with a wide picture, a small meta line, the title and a short summary. Each post has a centred header, a wide lead picture and one calm column, with the author box and three more posts after it.

Based on Ghost's Alto theme (MIT). Khalid's copy is rewritten for Khalid's templates and keeps the theme's licence beside its files.

```toml
[template]
name = "alto"
```

## Who it suits

- A writer who wants a quiet, picture-led personal blog.
- A journal where each post has a good picture and a short summary.
- A blog with a few posts worth pointing to, marked `featured`, for the strip at the top.
- A blog moving from Ghost that wants to keep Alto's look.

## What the front page shows

1. The header: the logo or name at the start, the menu right after it, then the search link and your call to action as a pill at the end.
2. On the first page, the featured strip: up to eight posts marked `featured`, newest first. Each card shows a square picture, its topic as a small pill in the accent colour, the title and the date. Four cards fit across a wide screen, three on a tablet and nearly one on a phone. The strip scrolls sideways and snaps to each card. It needs no script. `picks` sets how many cards show; `0` leaves the strip out.
3. The newest posts, one under another. Each shows its picture at 960 px and 2 by 1, then, in a 720 px column under it, a small meta line (the date, the reading time and the topic, split by slashes), the title and the summary.
4. The page links: newer posts at the start, older posts at the end and the page count in a pill between them. Set `pagination` to `numbered` for a link to each page.

## What a post shows

1. A centred header: the meta line with the date, the reading time and the topic, then the title, large and heavy.
2. The subtitle, when `dek` is on, and the date of the last update and who reviewed the post.
3. The lead picture, wider than the text.
4. The contents, folded under one line.
5. The post in one column, 18 px text, with a bar in the accent colour beside each quote.
6. A **Share this post** pill that opens the share links in a small menu. It needs no script.
7. The tags as pills.
8. The author box: the author's round photo beside their name, role and bio.
9. Three more posts under a small capital heading, each with a picture, the date and the title.

## List pages

A topic's or a tag's page shows its name in the 960 px column, its intro, then its posts in the same feed as the front page. An author's page shows the author's round photo above their name, role, profiles and bio, then their posts in the same feed.

## Your brand

Alto brings its own colours: soft charcoal ink on white, light grey panels and lines, and a calm blue as the accent. Your colours, fonts and logo in `theme/tokens.toml`, such as the ones theme sync writes, replace them, so it looks like your main site.

Alto's two font settings map to your font tokens. `[fonts] heading` sets the titles, as Alto's title font. `[fonts] body` sets every other word, as its body font. Set the heading to a serif for Alto's "Elegant serif" titles.

Alto's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Alto's default |
|---|---|
| `[colors] accent` | `#1f7ab8`, a calm blue, deep enough for light text on it |
| `[fonts] heading` | Inter, then the system's sans |
| `[fonts] body` | Inter, then the system's sans |
| `[type] body` | `1rem`; a post's text is 18 px |
| `[type] h1` | `2.25rem` on a phone to `3rem` on a wide screen |
| `[type] h2` | `1.5rem` to `1.875rem` |
| `[type] h3` | `1.25rem` |
| `[type] small` | `0.75rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `800` |
| `[shape] radius` | `0px`: pictures have square corners |
| `[shape] measure` | `38rem`, the width of a post's text |
| `[space] section` | `4rem` |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `false` | A post shows its description as a subtitle under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards in lists: `image` (Alto's feed), `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `true` | Each card's meta line shows how long the post takes to read. |
| `picks` | `8` | How many featured posts the strip on the front page shows, from `0` for no strip to `8`. |
| `pagination` | `"pages"` | `pages` shows newer, older and the page count; `numbered` links to each page near this one. |
| `related` | `"grid"` | How the posts after a post are laid out: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `3` | How many posts show after a post, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline-folded`, `inline`, `side-end`, `side-start` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` for 1, 1.1, or `none`. |
| `reading_progress` | `"bar"` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `share` | `"end"` | Where a post shows its Share pill: `end` after the post, `top` in the header, `both`, or `none`. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"none"` | Where the sign-up box shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"none"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a pill, as Alto's Subscribe button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its newest posts. They work with scripts off. |
| `video_section` | `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `false` | A row of the newest posts with a podcast or other sound player, when there are any. |

The call to action comes from `[template.cta]`; see [the templates guide](../templates.md#your-call-to-action). Without it, the header pill and the band do not show.

For example, a blog with four featured cards, the Share pill at the top of each post and the sign-up box after it:

```toml
[template]
name = "alto"
[template.options]
picks = 4
share = "top"
newsletter = "post"
```

## What Alto leaves out

Alto's Ghost features that a Khalid blog does not have are mapped or left out:

- The featured carousel and its arrows: a strip of cards that scrolls sideways with snap points, with no script.
- The dark mode and its switch: Alto here is light only.
- Members, paid tiers and the sign-in link: your call to action takes the place of the Subscribe button, and Khalid's newsletter form can show after posts or on the front page.
- Ghost's comments, search and share window: Khalid's own comments, search page and share links take their place.
- The logo position settings: the logo sits at the start, as Alto's default.
- The topic label laid over each featured picture: the label sits under the picture, so no words cover it.
- The gallery lightbox. Khalid's own enlarge feature can open pictures.
- Social icons and "Powered by Ghost" in the footer: the footer shows your footer links.

## Good to know

- Everything reads with scripts off. Alto adds no script.
- The front page's intro is not shown, as in Alto; it still serves as the page's description.
- A featured post with no picture shows a plain grey square in the strip.
- The sign-up box shows only when your blog can take sign-ups and `newsletter` asks for it: `mode = "server"` with a `[mail]` table.

## Design

This is Alto's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A writer who wants Ghost's Alto look: a calm, white page, big pictures and heavy titles, with nothing else in the way.

**Reference.** Ghost's Alto theme, its templates and its stylesheet.

| Part | Alto |
|---|---|
| Type pairing | One sans, Inter, at weight 800 for titles and 400 for the words; a serif for titles when the brand sets one. |
| Colour | White ground, soft charcoal ink, light grey panels and lines. One accent for the featured pills, buttons and the bar beside quotes. |
| Spacing rhythm | 100 px between posts in the feed, 64 px above the featured strip, 80 px before the footer. |
| Frame at 1440 px and wider | The header and the featured strip at **75rem** (1200 px); list pages at **60rem** (960 px), the frame the layout check holds. |
| Cards and lists | No box. Featured cards: a square picture, a pill, the title and the date. Feed cards: a 960 px picture at 2 by 1, then a 720 px column with the meta line, the title and the summary. |
| Phone header and menu | The logo at the start and a Menu button at the end; the call to action moves into the menu. |
| Tabs and indexes at 390 px | The strip shows nearly one card with the next peeking in; feed pictures turn to 1.6 by 1. |
| Link, card and focus states | A card's title fades a little on hover. Links in a post are underlined in the link colour. Focus is a 3 px accent ring. |
| Signature details | The sideways strip of square featured cards. The small capital meta line with slashes. The centred post header. The Share pill after the post. The round author photo in the author box. |

**Sample brand.** The demo uses `fixtures/showcase-brands/alto/theme/`: pine green (`#0b6b5e` links, `#0f7b6c` accent) on white with a pale stone panel (`#f3f3f1`), Lora for titles as Alto's "Elegant serif" setting, Inter for every other word, and a round mark with a hill.

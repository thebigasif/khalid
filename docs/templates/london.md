---
title: "London"
order: 65
---

# London

London is a template for a blog or portfolio that leads with large pictures. Its front page opens with one sentence set large in the middle of the page. Every post follows as a tall picture tile that runs from edge to edge.

Based on Ghost's London theme (MIT).

```toml
[template]
name = "london"
```

## Who it suits

- A studio, photographer, illustrator or designer whose posts each have a strong lead picture.
- A blog that wants few words on its front page: a picture and a title per post.
- Posts that read as one calm column under a large picture.

It suits a blog whose posts have no pictures less well. Those posts become dark tiles with a large number, which works for a few but not for most. For a text-led blog, Ledger or Bulletin reads better.

## What the front page shows

1. **The header:** your logo or the blog's name in spaced capitals, the menu beside it, then search and your call to action as a round ink button on the end side.
2. **The statement:** the `intro` of your home page, or the blog's description when there is none, set large, heavy and centred, with wide space above and below (option `statement`).
3. **The tiles:** every post as a picture tile, each as tall as a third of the window is wide. Two tiles sit side by side, then one runs across the whole width, and the pattern repeats. The pictures touch, with no gap between them. Under each picture sits the title, heavy and centred, then the writer, the date and the topic in small capitals. The whole tile is one link, and the picture dims a little under the pointer.
4. **Posts without a picture:** a dark tile with the title in the page colour at the top and the post's number in large faint type at the bottom.
5. Numbered page links, then each home section, the newsletter form and the call to action band, when they are on.

On a phone every tile runs the whole width. A later page of the front page shows the blog's name and that page's tiles.

## What a post shows

1. The title, large, heavy and centred.
2. The subtitle, from the post's `description`, in grey.
3. One quiet line: "By" and the writers, the topic, the date, when it was updated and the reading time.
4. The lead picture, wider than the text, up to 75rem.
5. The post in one centred column of about 65 letters a line. Links are underlined in your link colour.
6. The share links, centred, as a row of round buttons.
7. The author box, when it is on.
8. The newsletter form, centred on a hairline, and the call to action band, when they are on.
9. Comments, then three related posts as London's picture tiles.

## List pages

A topic or tag page shows its name large and centred, its description in grey under it, then its posts as the same picture tiles. An author page shows a round photo, the name, the role and the bio, and the writer's profiles as one line of names split by slashes, then their posts as tiles.

## Your brand

London takes your colours, fonts and logo from `theme/tokens.toml`, so it looks like your main site. The pictures carry most of the colour; your link colour marks links in posts. Titles use your heading font, set heavy.

London's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | London's default |
|---|---|
| `[colors]` | white page, near-black text `#131313`, light grey lines and a deep blue link colour `#1f6fae` |
| `[fonts] heading` and `body` | the reader's own sans: San Francisco, Segoe UI, Helvetica or Arial |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.75rem` |
| `[type] heading_weight` | `800` |
| `[type] line_height` | `1.7` |
| `[shape] radius` | `0px`; buttons are round |
| `[shape] measure` | `40rem`, the width of a post's text |

Nothing is downloaded for the fonts: each reader sees the sans their own device has.

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `statement` | `true` | The front page opens with your home page's `intro`, or the blog's description, set large in the middle. With `false` it shows the blog's name in its place. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `dek` | `true` | A post shows its `description` as a subtitle under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards in the tiles: `image`, `text`, `row`, `compact` or `ranked`. With `text`, every tile is a dark numbered tile. |
| `card_reading_time` | `false` | Each tile shows how long the post takes to read. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `contents` | `"none"` | Where a long post's contents go: `none`, `inline`, `inline-folded`, `side-end` or `side-start`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` for 1, 1.1, or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `related` | `"grid"` | How related posts are laid out: `grid` shows London's tiles, or `list`, `row`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | Which share links show, in this order. |
| `newsletter` | `"post"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"none"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a round button. |
| `hero_stats` | none | Up to four numbers under the statement, each written `number \| what it counts`. |
| `topic_tabs` | `false` | Tabs under the tiles, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `false` | A row of the newest posts with a podcast or other sound player, when there are any. |

The call to action comes from `[template.cta]`; see [the templates guide](../templates.md#your-call-to-action).

For example, a site that shows its name in place of the statement and its contents beside long posts:

```toml
[template]
name = "london"
[template.options]
statement = false
contents = "side-end"
```

## Good to know

- Everything reads with scripts off. London adds no script.
- No text sits on a picture. Ghost's London shows a post's title over its picture when the pointer is on it; here the title sits under the picture, so it reads on every screen.
- Ghost's members, paid tiers, comments and search are not part of London. Khalid's own newsletter form, comments and search take their places.
- Ghost loads more posts as a reader scrolls. London uses numbered pages.
- Ghost's picture viewer in posts is left out. Khalid's own `enlarge_images` feature can open a picture larger.

## Design

This is London's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A studio or a person whose work is best shown in large pictures. The first goal is to look at the work; the second is to read about one piece.

**Reference.** Ghost's London theme: a large centred statement, then a strip of tall picture tiles from edge to edge, two side by side and every third across the whole width, with dark numbered tiles for posts without a picture.

| Part | London |
|---|---|
| Type pairing | One sans for everything, heavy (weight 800) for titles and the statement, regular for text. With no theme, the pack uses the system sans. |
| Colour | White ground, near-black ink, light grey lines. The pictures carry the colour. Buttons are ink. Posts without a picture are ink tiles. |
| Spacing rhythm | A base step of 0.5rem. The page keeps a thin margin of 3% of the window, 6% on a phone. The statement has 12% of the window's width above and below. |
| Frame at 1440 px and wider | The header, the tiles and the footer run the whole window inside the thin margin, **1344 px** or more at 1440 px. A post's text keeps to **40rem**, its title to 56rem and its lead picture to 75rem. |
| Cards and lists | Picture tiles with no gap between them, each 35% of the window's width tall (70% on a phone), the title and a small line of capitals under each picture. |
| Phone header and menu | The logo and the menu button on one row. Every tile runs the whole width. |
| Tabs and indexes at 390 px | One column everywhere. The statement keeps its weight at about 34 px. |
| Link, card and focus states | A tile's picture dims a little and its title is underlined under the pointer. Links in a post are underlined. Focus is a 3 px accent ring. |
| Signature details | The large centred statement. The two-then-one rhythm of tiles from edge to edge. The dark tiles with a large faint number. The name in spaced capitals. |

**How it differs from Edge.** Both lead with pictures. London sets one sentence large at the top and shows tall tiles in a fixed two-then-one rhythm with each title under its picture. Edge is a masonry wall of pictures at their own shapes.

**Sample brand.** The demo uses `fixtures/showcase-brands/london/theme/`: charcoal ink (`#161616`) on white with a warm stone surface (`#f3f1ee`), burnt orange for links (`#9a3412`) and the accent (`#b8461b`), Mulish for everything, and a square mark beside the name in spaced capitals.

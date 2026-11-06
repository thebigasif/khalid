---
title: "Casper"
order: 25
---

# Casper

Casper is a template for a classic blog. The front page opens with a cover band: the blog's name on the accent colour, with a picture under it. A feed of picture cards follows, with one large card, two half cards and then three across. Each post reads in one serif column under a wide picture, with a sign-up box and three more posts after it.

Based on Ghost's Casper theme (MIT). Khalid's copy is rewritten for Khalid's templates and keeps the theme's licence beside its files.

```toml
[template]
name = "casper"
```

## Who it suits

- A writer or a small team that wants a familiar, picture-led blog.
- A blog with a strong picture or brand colour for its cover.
- Readers who skim: every card shows its topic, title, a short summary, the date and the reading time.
- A blog moving from Ghost that wants to keep Casper's look.

## What the front page shows

1. The header, a bar in the accent colour that runs straight into the cover. With the logo on the cover (the default `navigation`), the header shows only the menu, the search link and your call to action as a pill in the page's colour.
2. The cover, picked with `cover`:
   - **Picture**, the default: the blog's name set large and its intro on a band in the accent colour, then the newest featured post's picture as a wide strip under the band, tinted by the accent colour. No words sit on the picture. A blog with no featured post with a picture gets the band alone.
   - **Accent**: the accent colour behind the name and intro.
   - **Plain**: the name and intro on the page's own colour.
   - **None**: no cover; the feed starts at the top.
   With the logo in the middle or stacked, the cover shows the intro alone, set large, as Casper does. The intro is the front page's intro, or the blog's description when it has none. `cover_align` sets the words in the middle or at the start.
3. The feed, picked with `feed`:
   - **Classic**, the default: the newest post as a large card, its picture taking two thirds of the width and its words beside it. The next two posts take half the width each. Every other post is a card a third of the width.
   - **Grid**: every post a third of the width, each picture at its own shape.
   - **List**: every post as a large card, the picture on alternate sides.
4. Numbered page links. Later pages show the grid of thirds.

Each card shows its picture, its topic, a **Featured** mark when the post is featured, the title, two or three lines of summary, then the date and the reading time.

## What a post shows

1. The post's topic in small grey type.
2. The title, large and heavy, and the subtitle under it.
3. One row with the authors' round photos overlapping, their names, the date and the reading time, and, under it, the date of the last update and who reviewed the post. At the end of the row, a **Share this post** pill opens the share links in a small menu. It needs no script.
4. The lead picture, as wide as `post_image` says: wider than the text (the default), the whole window, as wide as the text, or hidden.
5. The contents, folded under one line.
6. The post in one serif column 720 px wide, 20 px text, with a bar in the accent colour beside each quote.
7. The tags as pills.
8. The sign-up box: a large title, then the field with the button inside its end, when your blog takes sign-ups.
9. Three more posts as cards, across the whole width.

Casper turns the author box off. Turn it on in `[reader]` with `author_box = true` if you want it.

## List pages

A topic's or a tag's page shows its name large, its intro, then its posts as cards three across. An author's page shows the author's round photo above their name, role, profiles and bio, then their posts in the same grid.

## The header

`navigation` picks where the logo sits:

- **Cover**, the default: the logo or name at the start, the menu after it, then the search link and your call to action at the end. On the front page, the blog's name moves onto the cover.
- **Middle**: the menu at the start, the logo in the middle and the rest at the end.
- **Stacked**: the logo in the middle of the top row, with the search link at the start and the call to action at the end, then the menu centred on a row of its own between two thin lines.

While the blog has a picture or accent cover, the header on every other page is a bar in the accent colour, with the logo, the links and the button turned to light. A long menu scrolls sideways inside the header. On a phone, the menu opens from a Menu button, with no script.

## Your brand

Casper brings its own colours: near-black ink on white, light grey lines and Ghost's pink as the accent. Your colours, fonts and logo in `theme/tokens.toml`, such as the ones theme sync writes, replace them, so it looks like your main site.

Casper's two font settings map to your font tokens. `[fonts] heading` sets the titles and the page's own words, as Casper's title font. `[fonts] body` sets a post's text, as Casper's body font. Set both to a sans for Casper's "Modern sans-serif" body, or both to a serif for its "Elegant serif" titles.

On the accent bar and the cover, your logo shows in one light colour.

Casper's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Casper's default |
|---|---|
| `[colors] accent` | `#e6005f`, Ghost's pink, deep enough for light text on it |
| `[fonts] heading` | The system's sans |
| `[fonts] body` | Georgia, a serif |
| `[type] body` | `1rem`; a post's text is 20 px |
| `[type] h1` | `2rem` on a phone to `3.25rem` on a wide screen |
| `[type] h2` | `1.4375rem` to `1.75rem` |
| `[type] h3` | `1.25rem` to `1.5rem` |
| `[type] small` | `0.8125rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `800` |
| `[shape] radius` | `0px`: pictures have square corners |
| `[shape] measure` | `45rem`, the width of a post's text |
| `[space] section` | `4rem` |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `navigation` | `"cover"` | Where the header puts the logo: `cover`, `middle` or `stacked`. |
| `cover` | `"picture"` | How the front page opens: `picture`, `accent`, `plain` or `none`. |
| `cover_align` | `"center"` | Whether the cover's words are centred (`center`) or set at the start (`start`). |
| `feed` | `"classic"` | The front page's posts: `classic`, `grid` or `list`. |
| `post_image` | `"wide"` | How wide a post's lead picture is: `wide`, `full`, `small` or `hidden`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description as a subtitle under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on topic, tag and author pages: `image` (Casper's grid), `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `true` | Each card shows how long the post takes to read. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related` | `"grid"` | How the posts after a post are laid out: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `3` | How many posts show after a post, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline-folded`, `inline`, `side-end`, `side-start` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` for 1, 1.1, or `none`. |
| `reading_progress` | `"bar"` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `share` | `"top"` | Where a post shows its share links: `top` as the Share pill in the byline row, `end` as a row after the post, `both`, or `none`. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"post"` | Where the sign-up box shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"none"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a pill, as Casper's Subscribe button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its newest posts. They work with scripts off. |
| `video_section` | `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `false` | A row of the newest posts with a podcast or other sound player, when there are any. |

The call to action comes from `[template.cta]`; see [the templates guide](../templates.md#your-call-to-action). Without it, the header pill and the band do not show.

For example, a blog with the logo in the middle, a plain cover set at the start and a list feed:

```toml
[template]
name = "casper"
[template.options]
navigation = "middle"
cover = "plain"
cover_align = "start"
feed = "list"
```

## What Casper leaves out

Casper's Ghost features that a Khalid blog does not have are mapped or left out:

- Members, paid tiers and the sign-in link: Casper shows Khalid's newsletter form where Ghost shows its sign-up box, and your call to action where Ghost shows its Subscribe button.
- Ghost's comments, search and share window: Khalid's own comments, search page and share links take their place.
- Infinite scroll: numbered page links.
- The site cover image setting: the cover takes the newest featured post's picture.
- The sign-up text setting: the sign-up box uses Khalid's own words.
- The dark and auto colour schemes: Casper here is light only.
- The members-only marks on cards and the gallery lightbox. Khalid's own enlarge feature can open pictures.
- Social icons and "Powered by Ghost" in the footer: the footer shows your footer links.

## Good to know

- Everything reads with scripts off. Casper adds no script.
- The sign-up box shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table. It signs readers up to your default list.
- The cover's picture is the same as the featured post's own picture, so pick a featured post whose picture works large.

## Design

This is Casper's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A writer or a small team that wants Ghost's classic look: a coloured cover, big picture cards and a calm serif post.

**Reference.** Ghost's Casper theme, its templates and its stylesheet.

| Part | Casper |
|---|---|
| Type pairing | The system's sans at weight 800 for titles and the page's own words; Georgia for a post's text. |
| Colour | White ground, near-black ink, light grey lines. One accent for the cover, the header bar on other pages, buttons and the bar beside quotes. A dark footer in the ink. |
| Spacing rhythm | A gap of 4 per cent of the screen between columns and 4.8 per cent between rows. 64 px or more before the footer. |
| Frame at 1440 px and wider | At least **75rem** (1200 px). |
| Cards and lists | No box: a picture at 100 by 55, the topic, the title, two or three lines of summary and a small meta line. A six-column grid: a large card across all six, half cards across three, the rest across two. |
| Phone header and menu | The logo at the start and a Menu button at the end; the call to action moves into the menu. |
| Tabs and indexes at 390 px | Every card takes the whole width; the large card stacks its picture over its words. |
| Link, card and focus states | A card's title fades a little on hover. Links in a post are underlined in the link colour. Focus is a 3 px accent ring. |
| Signature details | The cover band with the name set large in the middle. The accent header bar with a pill button in the page's colour. The overlapping round author photos beside the Share pill. The serif post column. The dark footer with dots between the links. |

**Sample brand.** The demo uses `fixtures/showcase-brands/casper/theme/`: sky blue (`#1468a3` links, `#1672b0` accent) on white with a cool grey panel (`#f4f6f8`), the system's sans for titles and Georgia for a post's text, as Casper sets them, and a round mark with a peak.

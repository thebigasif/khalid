---
title: "Edition"
order: 38
---

# Edition

A newsletter with a big cover: a picture from edge to edge with your pitch and the sign-up form, then featured posts and a quiet list of the latest.

Based on Ghost's Edition theme (MIT).

```toml
[template]
name = "edition"
```

## Who it suits

Edition suits a writer or a small team whose blog is also a newsletter. A visitor lands on one large picture with a single sentence about what you write and a field for their email address. Under it sit a few featured posts, then every post in one calm column, newest first.

Pick another template if you publish many topics at once and want a grid of pictures, or if your posts rarely have pictures and the cover would stay plain.

## What each page shows

**Front page**
- The cover. It takes the picture of the newest featured post, or else of the newest post with a picture, and lays it across the whole width of the screen. The cover starts with a dark band under the header. In its middle, set large in light type: your `cover_text`, or else the front page's intro, or else the blog's name. Under it, the sign-up form in one white box, the email field with the button inside it, when your site takes sign-ups. The picture runs under the band to the foot of the screen, with no text on it, so every word stays clear of the picture. An arrow at the foot of a full cover links down to the latest posts. With `cover = "half"` the cover fills about half the screen. With no picture to use, the cover is plain text on the page. With `cover = "none"` the front page starts with its title.
- "Featured": up to `picks` posts marked `featured: true`, three across a wide screen in one row that scrolls sideways, each a wide picture over a bold title. The whole card opens the post.
- "Latest posts": every post in a 45rem column under a small capital heading with a hairline, in the layout `feed_layout` names:
  - `expanded`: a picture at the start, then the bold title, two lines of the summary and a line with the date, the reading time and the topic. On a phone the picture sits above.
  - `thumbnail-end`: the same, with the picture at the end.
  - `text-only`: the same without pictures, with a hairline between posts.
  - `minimal`: one line a post, with a date block of day and month in your link colour, the title, the reading time and an arrow.
- Numbered page links, the call to action when `cta_band` asks for it, each home section's four newest posts, and the topics.
- Pages 2 and on show the blog's name as a plain, centred title over the list.

**A post**
- A centred header: the date, the reading time and the topic in small capitals, then a large title and the post's description.
- The byline: each author's photo, name and role, and the date it was updated, if it was.
- The lead picture, wider than the text on a wide screen.
- A folded contents list for long posts, then the body.
- After the body: the share links, the tags and the author box, centred between two hairlines. After the post: the newsletter and the call to action, each between an accent rule and a hairline, comments when they are open, and four related posts on a grey band across the window.

**Lists and author pages** show posts as the front page's expanded rows. A list's title is centred. An author's page has a round photo over the centred name, role and bio.

Everything reads with scripts off. Edition brings its own look: near-black ink on white, light grey hairlines, one warm orange for buttons, and small rounded corners. Your theme tokens, such as the ones theme sync writes, replace any of those colours and fonts, and the logo comes from them too.

## Options

Set these under `[template.options]`. Each line shows the default.

| Option | Default | What it changes |
|---|---|---|
| `cover` | `"full"` | How tall the front page's cover is when a post has a picture for it: `full` fills the screen, `half` fills about half, `none` leaves the cover out. |
| `cover_text` | empty | The line set large on the cover, up to 160 characters, such as what the newsletter is about. Without it the cover shows the front page's intro. |
| `feed_layout` | `"expanded"` | How the latest posts are listed on the front page: `expanded`, `thumbnail-end`, `text-only` or `minimal`. |
| `picks` | `6` | How many featured posts the "Featured" row shows, from `0` for none to `8`. |
| `logo` | `true` | Whether the header shows your logo from `[brand] logo` in place of the blog's name. |
| `dek` | `true` | Whether a post shows its `description` under its title. |
| `updated_note` | `false` | Whether an updated post says why it was updated, under its byline. |
| `cards` | `"row"` | The shape of the post cards in lists: `image`, `text`, `row`, `compact` or `ranked`. Only `row` gives Edition's rows with a picture. |
| `card_reading_time` | `true` | Whether each card shows how long the post takes to read. |
| `pagination` | `"numbered"` | The links under a long list: `pages` for newer and older, or `numbered` for a link to each page. |
| `related` | `"row"` | How the related posts are laid out: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `4` | How many related posts show, from 3 to 10. |
| `contents` | `"inline-folded"` | Where a post's contents go: `inline`, `inline-folded`, `side-start`, `side-end` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | The networks the share links go to, in the order they show. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. On the front page it sits in the cover. It needs `[mail]`, so a static site shows none. |
| `cta_band` | `"post"` | Where the box with your `[template.cta]` shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | Whether the header shows your `[template.cta]` as a button. |

A full example:

```toml
[template]
name = "edition"
[template.options]
cover = "half"
cover_text = "A weekly letter on writing for search, and for the people who read it."
feed_layout = "minimal"
picks = 3
```

## Sizes and spacing

Edition sets these in place of classic's. A value in your own `theme/tokens.toml` wins.

```toml
[colors]
background = "#ffffff"
surface = "#f6f6f6"
text = "#1a1a1a"
muted = "#666666"
link = "#b8350f"
accent = "#cf3f14"
border = "#e6e6e6"

[fonts]
heading = "system-ui, -apple-system, \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, sans-serif"
body = "system-ui, -apple-system, \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, sans-serif"

[shape]
radius = "0.375rem"
measure = "45rem"

[type]
body = "1rem 1.0625rem"
h1 = "2.125rem 2.5rem"
h2 = "1.5rem 1.875rem"
h3 = "1.125rem 1.25rem"
h4 = "1rem 1.0625rem"
small = "0.8125rem"
line_height = 1.6
heading_weight = 800

[space]
unit = "0.5rem"
section = "4.5rem"
```

The stylesheet adds its own sizes on top of these: the cover's line grows to 2.25rem on a wide screen, as do a post's title, a list's title and an author's name. Section headings are small capitals at 0.9375rem over a hairline. The header, the featured row and a post's lead picture use 75rem, as Ghost's Edition sets them. Lists keep to the 45rem measure, and a list's title sits over a hairline as wide as the list. A post's text keeps to 32.5rem at 18 px on a wide screen, about 70 characters a line, while its title and description use 45rem. With the contents beside it, a post grows by 18rem so the text keeps that measure.

## Reader features

Edition keeps every `[reader]` feature as it is. Comments stay off unless your site runs as a server with comments on.

## From Ghost's Edition theme

Edition is a rewrite of Ghost's Edition theme for Khalid's templates and tokens. Some of Ghost's parts work differently here:

- Ghost's cover uses the site's cover image. Khalid has no such image, so Edition takes the picture of the newest featured post, or else of the newest post with a picture. Ghost sets the line, the form and the header on the picture. Edition sets them on a dark band above it, so no text sits on a picture whose colours it cannot know. Ghost's `email_signup_text` is the `cover_text` option, and `publication_cover_style` is `cover`.
- Ghost's featured posts slide in a carousel with arrows and a script. Here they sit in a row that scrolls sideways with no script.
- Ghost's "Load more" button is Khalid's numbered pages.
- Ghost's members sign-up is Khalid's newsletter form. Paid tiers and the star on paid posts are left out. Ghost's comments and search are Khalid's own.
- Ghost's "Published by" avatars under a post are Khalid's author box. Ghost's links to the previous and next post are left out.
- Ghost's three post templates by picture width are one post page whose lead picture is wider than the text.
- Ghost's title and body font settings come from your theme tokens. Ghost's navigation layouts are one: the name at the start, the menu in the middle, search and the button at the end.
- Ghost's gallery lightbox and its scripts are left out. A post page runs no script of its own.

## Design

This is Edition's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A newsletter that is also a blog, led by one writer or a small team.

**Reference.** Ghost's Edition theme: a full-screen cover picture under a dark veil with one large line and an email field, a carousel of three featured posts, then a narrow "Latest" feed of rows with a small picture, a bold title, two lines of summary and the date.

| Part | Edition |
|---|---|
| Type pairing | One heavy sans for headings and a plain sans for text. With no theme, the pack uses the system sans for both, headings at weight 800. |
| Colour | White, near-black ink, light grey hairlines and one warm accent for buttons and the accent rule over the sign-up. The cover's band is the ink colour with the page colour for its text. |
| Spacing rhythm | A base step of 0.5rem. Rows 2.5rem apart. Sections 3 to 4.5rem apart. |
| Frame at 1440 px and wider | **45rem** for lists, the frame the layout check holds; **75rem** for the header, the featured row and the lead picture; **32.5rem** for post text. The cover runs the full width of the screen. |
| Cards and lists | One column of rows with no boxes: a 10rem picture at 16 by 10 at the start, a bold title, two lines of summary, then the date, reading time and topic split by dots. The whole row is the link. |
| Phone header and menu | The logo, search and the menu button on one row. The cover's line drops to 1.6875rem and the form spans the width. |
| Link, card and focus states | Titles fade a little on hover. Focus is a 3 px accent ring. |
| The one idea | The front page opens like a magazine cover with a sign-up on it. Someone shown only the thumbnail should name it by the screen-wide cover: a dark band with one large light line and a white email box, over a picture to the foot of the screen. |
| Type scale | The cover's line 1.6875 to 2.25rem, post and page titles 1.75 to 2.5rem, row titles 1.125rem, section labels 0.9375rem in capitals. |
| Signature details | The screen-wide cover of a dark band over a picture. The small capital section labels over a hairline. The featured row of three wide pictures. The minimal feed's day and month block. The accent rule over the sign-up and the call to action. |

**Sample brand.** The demo uses `fixtures/showcase-brands/edition/theme/`: deep teal (`#0d6b63` links, `#0f766e` accent) on white with a pale green-grey field (`#f3f7f6`), Nunito for headings and Source Sans 3 for text, and a letter in a square for the logo.

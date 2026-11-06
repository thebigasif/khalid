---
title: "Dawn"
order: 33
---

# Dawn

A calm, text-led blog and newsletter: a small centred cover with the sign-up, a row of featured posts, then every post as one line with a day and month block, the title and the reading time.

Based on Ghost's Dawn theme (MIT).

```toml
[template]
name = "dawn"
```

## Who it suits

Dawn suits a writer or a small team whose blog is also a newsletter, and whose readers want to scan many titles fast. A visitor lands on one short sentence about the blog and a field for their email address. Under it sit a few featured posts with pictures, then every post as a plain list, newest first.

Pick another template if your posts lead with pictures, or if readers need a summary of each post before they choose one.

## What each page shows

**Front page**
- The cover, small and centred: your site's icon from `[brand] icon`, if you set one, then the front page's intro, or else the site's description. Under it, the sign-up form, the email field beside Dawn's small capital Subscribe button, when your site takes sign-ups. Without sign-ups the cover shows your `[template.cta]` as a button instead.
- The featured row: up to `picks` posts marked `featured: true`, under a small capital heading, `featured_title` or else "Featured". Three sit across a wide screen in one row that scrolls sideways, each a picture twice as wide as tall over a bold title. The whole card opens the post.
- The feed: every post in a 45rem column, one line a post. A day and month block in your link colour, the title in the text weight, cut short with an ellipsis when it is long, a star on a featured post, the reading time, and an arrow that slides in on hover. The whole line opens the post. On a phone the reading time and the arrow are left out.
- Numbered page links, the call to action when `cta_band` asks for it, each home section's five newest posts as the same lines, and the topics.
- Pages 2 and on show the blog's name as a centred title over the list.

**A post**
- A centred header: the date, the reading time and the topic in small capitals, the topic in your link colour. Then a large title, the post's description, and the date it was updated, if it was.
- The share links as small capital buttons, then the lead picture, wider than the text on a wide screen.
- A folded contents list for long posts, then the body.
- After the body: the tags, then "Written by" over each author's round photo and name, and the author box. After the post: the call to action between an accent rule and a hairline, comments when they are open, and up to `related_count` related posts as the feed's lines on a grey band across the window.

**Lists** show the name centred and large in your link colour, a tag's with a # before it, then the posts as the front page's lines. A series keeps its numbered cards.

**An author's page** has a small round photo over the centred name, the role and post count in small capitals, the bio, then every post as the front page's lines.

Everything reads with scripts off. Dawn brings its own look: near-black ink on white, light grey hairlines, one warm orange for the date blocks, the buttons and the accent rules, and small square corners. Your theme tokens, such as the ones theme sync writes, replace any of those colours and fonts, and the logo comes from them too.

## Options

Set these under `[template.options]`. Each line shows the default.

| Option | Default | What it changes |
|---|---|---|
| `featured_title` | empty | The heading over the featured row, up to 60 characters. Without it the row is headed "Featured". |
| `picks` | `6` | How many featured posts the featured row shows, from `0` for none to `8`. |
| `logo` | `true` | Whether the header shows your logo from `[brand] logo` in place of the blog's name. |
| `dek` | `true` | Whether a post shows its `description` under its title. |
| `updated_note` | `false` | Whether an updated post says why it was updated, under its title. |
| `cards` | `"text"` | The shape of the post cards on pages Dawn does not draw itself, such as search results and a series: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `true` | Whether each line in the feed shows how long the post takes to read. |
| `pagination` | `"numbered"` | The links under a long list: `pages` for newer and older, or `numbered` for a link to each page. |
| `related_count` | `5` | How many related posts show, from 3 to 10. |
| `contents` | `"inline-folded"` | Where a post's contents go: `inline`, `inline-folded`, `side-start`, `side-end` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `share` | `"top"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | The networks the share links go to, in the order they show. |
| `newsletter` | `"home"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. On the front page it sits in the cover. It needs `[mail]`, so a static site shows a note in its place. |
| `cta_band` | `"post"` | Where the box with your `[template.cta]` shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | Whether the header shows your `[template.cta]` as a button. |

A full example:

```toml
[template]
name = "dawn"
[template.options]
featured_title = "Start here"
picks = 3
newsletter = "both"
```

## Sizes and spacing

Dawn sets these in place of classic's. A value in your own `theme/tokens.toml` wins.

```toml
[colors]
background = "#ffffff"
surface = "#f6f6f6"
text = "#1a1a1a"
muted = "#6b6b6b"
link = "#c2370f"
accent = "#e8491f"
border = "#e6e6e6"

[fonts]
heading = "-apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif"
body = "-apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif"

[shape]
radius = "0.1875rem"
measure = "45rem"

[type]
body = "1rem 1.0625rem"
h1 = "2.125rem 2.5rem"
h2 = "1.5rem 1.75rem"
h3 = "1.125rem 1.25rem"
h4 = "1rem 1.0625rem"
small = "0.8125rem"
line_height = 1.6
heading_weight = 800

[space]
unit = "0.5rem"
section = "4.5rem"
```

The stylesheet adds its own sizes on top of these. The header and the featured row use 75rem, as Ghost's Dawn sets them. Lists keep to the 45rem measure. A post's title and description use 45rem too, while its text keeps to 38rem at 18 px, about 70 characters a line. With the contents beside it, a post grows by 18rem. A post's title grows to 2.5rem on a wide screen, a list's title to 1.875rem. Feed lines are 1rem titles with 0.75rem date blocks; section labels are capitals at 0.9375rem over a hairline.

## Reader features

Dawn keeps every `[reader]` feature as it is. Comments stay off unless your site runs as a server with comments on.

## From Ghost's Dawn theme

Dawn is a rewrite of Ghost's Dawn theme for Khalid's templates and tokens. Some of Ghost's parts work differently here:

- Ghost's featured posts slide in a carousel with arrows and a script. Here they sit in a row that scrolls sideways with no script. Ghost's `featured_title` setting is the `featured_title` option, and `show_featured_posts` is `picks`, with `0` for none.
- Ghost's "Load more" button is Khalid's numbered pages.
- Ghost's cover shows the site's description with Subscribe and Login buttons. Here it shows the front page's intro, or else the site's description, with Khalid's newsletter form. There are no member logins, so the Login button is left out.
- Ghost's star marks a post for paying members. Khalid has no paid tiers, so the star marks a featured post.
- Ghost's comments and search are Khalid's own. Ghost's share pop-up is a row of share links.
- Ghost's "Published by" photos under a post are "Written by" with each author's photo and name, followed by the author box. Ghost's arrows to the previous and next post are left out.
- Ghost's three post templates by picture width are one post page whose lead picture is wider than the text. Ghost's `show_author` setting is the `byline` reader feature, and `show_related_posts` is the `related_posts` reader feature.
- Ghost's title and body font settings come from your theme tokens. Ghost's navigation layouts are one: the name at the start, the menu after it, search and the button at the end.
- Ghost's light and dark colour schemes are one light scheme. Ghost's gallery lightbox and its scripts are left out. A post page runs no script of its own.

## Design

This is Dawn's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A writer or a small team with a newsletter, whose readers scan titles.

**Reference.** Ghost's Dawn theme: a small centred cover with the site icon, description and Subscribe button, a carousel of three featured posts with wide pictures, then a narrow list of one-line rows, each with a day and month block in the brand colour, the title, the reading time and an arrow.

| Part | Dawn |
|---|---|
| Type pairing | One open sans for headings and text, headings at weight 800 and feed titles at the text weight. With no theme, the pack uses the system sans. |
| Colour | White, near-black ink, light grey hairlines and one warm orange for the date blocks, buttons, the star and the accent rules. The related posts sit on the grey surface. |
| Spacing rhythm | A base step of 0.5rem. Feed lines 0.9375rem above and below. Sections 2.5 to 4rem apart. |
| Frame at 1440 px and wider | **45rem** for lists and posts, the frame the layout check holds (720 px); **75rem** for the header, the featured row and the lead picture; **38rem** for post text. |
| Cards and lists | No boxes. One line a post with a hairline between lines: date block, title with an ellipsis, star, reading time, arrow on hover. Featured cards are a 2 to 1 picture over a bold title. |
| Phone header and menu | The logo, search and the menu button on one row. The featured row shows one card and a slice of the next. Feed lines keep the date and the title. |
| Link, card and focus states | Titles fade a little on hover, and the feed's arrow slides in. Focus is a 3 px accent ring. |
| The one idea | A blog you read as a list. Someone shown only the thumbnail should name it by the column of one-line rows, each opened by a small coloured day and month. |
| Type scale | Post titles 1.625 to 2.5rem, list titles 1.75 to 1.875rem, featured titles 1.125rem, feed titles 1rem, date blocks and small capitals 0.75rem. |
| Signature details | The day and month blocks in the link colour. The small centred cover with the sign-up. The three wide featured pictures. A tag's name in colour with a # before it. The accent rule over the call to action. Related posts on a grey band. |

**Sample brand.** The demo uses `fixtures/showcase-brands/dawn/theme/`: a clear early-sky blue (`#2f4db3` links, `#3d5bd9` accent) on white with a pale blue-grey field (`#f3f5fa`), Mulish for headings and text as Dawn sets it, and a rising sun beside the name for the logo.

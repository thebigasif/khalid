---
title: "Episode"
order: 35
---

# Episode

A podcast's blog: the newest episode and its player first, then every episode in a bold box.

Based on Ghost's Episode theme (MIT).

```toml
[template]
name = "episode"
```

## Who it suits

Episode suits a podcast, or any show that publishes episodes with a sound file and written notes. A visitor lands on a dark band with the show's name, a line about it and the places to listen. Under it, the newest episode waits in a large box with its player, so they can press play without leaving the front page. Then every episode follows, one under another, each in a box with a square picture.

Pick another template if most of your posts have no sound, or if you want a grid of pictures on the front page.

## What each page shows

**Front page**
- A dark band across the full width with your blog's name in large type at its foot, and your `tagline` under it.
- A second dark band with the places to listen from `listen_on`, such as Spotify or Apple Podcasts, each a round play mark with "Listen on" over the service's name. Without `listen_on` this band does not show.
- The newest episode: the newest post that carries a sound player, from an `::audio` block or an embedded player from Spotify or SoundCloud. It shows large in a box with a thick edge: its picture, date, title and summary, then the player itself and a "View episode" button. The player plays from the front page. A post without a sound player never shows here, and without any, the front page goes straight to the list.
- "Latest episodes": the other posts, each a box with a thick edge, its square picture beside the words on a wide screen and above them on a phone, the date and topic over the title, two lines of its summary and a play mark in your accent colour. The whole box opens the post. Numbered page links follow.
- When `[[reader.home_sections]]` lists sections, three posts for each, with a "View all" link.
- The newsletter, centred between two thick rules, with a large heading over one row of email field and button, when your site takes sign-ups. Then the topics.
- Pages 2 and on show the blog's name as a plain title over the list.

**A post**
- The topic, the date and the reading time on one line, then a large title and the post's description under it.
- The byline: each author's photo, name and role, and the date it was updated, if it was.
- No lead image unless you set `lead_image = true`, so the sound player in the post is the first thing a listener meets. The player sits in a box with a thick edge.
- A folded contents list for long show notes, then the body.
- After the body: the post's tags, the share links and the author box. After the post: the newsletter, comments when they are open, and three related episodes in the same boxes as the front page.

**Lists and author pages** show posts in the same boxes, under a large heading.

Everything reads with scripts off. Episode brings its own look: near-black ink on a light grey ground with one strong pink, square corners and heavy headings. Your theme tokens, such as the ones theme sync writes, replace any of those colours and fonts, and the logo comes from them too.

## Options

Set these under `[template.options]`. Each line shows the default.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | empty | One line under the show's name on the front page, up to 140 characters. |
| `listen_on` | none | Up to four places to listen, each written `name \| address`, such as `"Spotify \| https://open.spotify.com/show/1"`. Only an address that starts with `https://` or `http://` shows. Nothing shows until you write them. |
| `podcast_section` | `true` | Whether the front page leads with the newest episode and its player. With `false`, the list starts under the bands. |
| `lead_image` | `false` | Whether a post shows its lead image under its title. Lists still show it. |
| `logo` | `true` | Whether the header shows your logo from `[brand] logo` in place of the blog's name. |
| `dek` | `true` | Whether a post shows its `description` under its title. |
| `updated_note` | `false` | Whether an updated post says why it was updated, under its byline. |
| `cards` | `"row"` | The shape of the post cards in lists: `image`, `text`, `row`, `compact` or `ranked`. Only `row` gives the boxes with a square picture. |
| `card_reading_time` | `false` | Whether each card shows how long the post takes to read. |
| `pagination` | `"numbered"` | The links under a long list: `pages` for newer and older, or `numbered` for a link to each page. |
| `related` | `"row"` | How the related posts are laid out: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show, from 3 to 10. |
| `contents` | `"inline-folded"` | Where a post's contents go: `inline`, `inline-folded`, `side-start`, `side-end` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` or `none`. |
| `reading_progress` | `"bar"` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | The networks the share links go to, in the order they show. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. It needs `[mail]`, so a static site shows none. |
| `cta_band` | `"none"` | Where the band with your `[template.cta]` shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | Whether the header shows your `[template.cta]` as a button. |

A full example:

```toml
[template]
name = "episode"
[template.options]
tagline = "A weekly show about writing for search, and for the people who read it."
listen_on = [
  "Apple Podcasts | https://podcasts.apple.com/podcast/id1",
  "Spotify | https://open.spotify.com/show/1",
]
```

To give an episode its player, put an audio block in the post, with the sound file in the post's folder or at a full address:

```markdown
::audio{src="episode-12.mp3" title="Episode 12: answers first"}
```

## Sizes and spacing

Episode sets these in place of classic's. A value in your own `theme/tokens.toml` wins.

```toml
[colors]
background = "#ebebeb"
surface = "#f6f6f6"
text = "#15171a"
muted = "#4b4f56"
link = "#b3004f"
accent = "#d1005f"
border = "#c9c9c9"

[fonts]
heading = "system-ui, -apple-system, \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, sans-serif"
body = "system-ui, -apple-system, \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, sans-serif"

[shape]
radius = "0rem"
measure = "42rem"

[type]
body = "1.0625rem 1.1875rem"
h1 = "2.25rem 3.25rem"
h2 = "1.625rem 2.25rem"
h3 = "1.25rem 1.5rem"
h4 = "1.0625rem 1.125rem"
small = "0.875rem"
line_height = 1.6
heading_weight = 800

[space]
unit = "0.5rem"
section = "4rem"
```

The stylesheet adds its own display sizes on top of these: the show's name on the front page grows to 5.5rem on a wide screen, a post's title and a list page's title to 4rem, and the newest episode's title to 2.75rem. The frame is 60rem, as Ghost's Episode sets it. A post keeps to the 42rem measure.

## Reader features

Episode keeps every `[reader]` feature as it is. Comments stay off unless your site runs as a server with comments on.

## From Ghost's Episode theme

Episode is a rewrite of Ghost's Episode theme for Khalid's templates and tokens. Some of Ghost's parts work differently here:

- The links to Apple Podcasts, Spotify and YouTube Music are the `listen_on` option, for any service. Each shows a plain play mark, not the service's own logo.
- Ghost's home page shows the three latest episodes and a link to the rest. Episode leads with the newest episode and its player, then lists the others with numbered pages.
- Ghost's members sign-up is Khalid's newsletter form. Ghost's comments and search are Khalid's own.
- Ghost's cover image behind the header is a dark band in your text colour with a soft glow in your accent.
- Ghost's background colour and typography settings come from your theme tokens.
- Ghost's gallery lightbox and its scripts are left out. A post page runs no script of its own.

## Design

This is Episode's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A podcast, or any show whose posts each carry an episode to play.

**Reference.** Ghost's Episode theme: a tall header with large white type over a dark picture, a black strip of listening services, boxed episodes with a thick dark edge and a square picture, and a sign-up between two thick rules.

| Part | Episode |
|---|---|
| Type pairing | One heavy, wide sans for headings and a plain sans for text. With no theme, the pack uses the system sans for both, headings at weight 800. |
| Colour | A light grey ground, near-black ink for the bands and the box edges, and one strong accent for the play marks and buttons, with the ground's colour inside them. |
| Spacing rhythm | A base step of 0.5rem. Boxes 1.5rem apart with 1.5rem inside. Sections 3 to 5.5rem apart. |
| Frame at 1440 px and wider | **60rem**, the narrowest of the templates, as Ghost's Episode sets it. The two bands run the full width of the screen. |
| Cards and lists | Every list is one column of boxes with a 4px ink edge and square corners: a square picture of 16.25rem beside the words on a wide screen, the date over the title, two lines of summary, then a 3rem play mark. The whole box is the link. |
| Phone header and menu | The logo and the menu button on one row. The listening services wrap onto two columns. |
| Link, card and focus states | A box turns to the surface colour on hover. Title links underline on hover. Focus is a 3 px accent ring. |
| The one idea | The front page is a show's page: a dark band with the name, the places to listen, and the newest episode ready to play. Someone shown only the thumbnail should name it by the dark bands over a stack of thick-edged boxes, each with a square picture and an accent play mark. |
| Type scale | The show's name 2.5 to 5.5rem, page and post titles 2.25 to 4rem, section headings 1.875 to 3rem, episode titles in lists 1.375 to 2rem. Display type is tight, 1 to 1.1 line height. |
| Signature details | The "Listen on" band with round play marks. The newest episode's player on the front page. The accent play mark in every box. The sign-up between two thick rules. |

**Sample brand.** The demo uses `fixtures/showcase-brands/episode/theme/`: violet (`#4c1fb0` links, `#5a27d0` accent) on a cool grey (`#e9e8ee`), Libre Franklin for headings and Public Sans for text, and a play mark in a square.

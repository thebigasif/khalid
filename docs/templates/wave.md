---
title: "Wave"
order: 88
---

# Wave

An interview show's blog: the newest episode in a slim player card, then every episode as a row with square artwork and a round play mark.

Based on Ghost's Wave theme (MIT).

```toml
[template]
name = "wave"
```

## Who it suits

Wave suits an interview show: a podcast where each episode is a conversation with a guest, with a sound file and written notes. The page is quiet and white, in one narrow column, so the guests and the episode titles carry it. A visitor sees the show's name and the places to listen, then the newest conversation in a slim player card, ready to play. Every other episode follows as a row: square artwork with a round play mark, the date and topic, the title and a short summary.

Wave and Episode are both for shows. Pick Episode for a louder look, with dark bands and thick boxes. Pick Wave for a calm, light page that keeps the attention on each guest. Pick another template if most of your posts have no sound.

## What each page shows

**Front page**
- Your blog's name as the page title, and the front page's intro under it, in grey.
- The places to listen from `listen_on`, such as Spotify or Apple Podcasts, in one row, each with a small headphones mark in your accent colour. Without `listen_on` this row does not show.
- "Latest episode": the newest post that carries a sound player, from an `::audio` block or an embedded player from Spotify or SoundCloud. It shows in a slim card with a hairline edge: its square artwork, its date and topic, its title, then the player itself. The player plays from the front page. A post without a sound player never shows here, and without any, the front page goes straight to the list.
- "Latest episodes": the other posts, one row each, with square artwork of 12.5rem and a round play mark in your accent colour, the date and topic in small capitals, the title and a short summary. A post without a picture gets a grey square with the play mark. The whole row opens the post. Numbered page links follow.
- When `[[reader.home_sections]]` lists sections, three posts for each, with a "View all" link.
- The newsletter in a soft grey panel, when your site takes sign-ups. Then the topics.
- Pages 2 and on show the blog's name as a plain title over the list.

**A post**
- A player card as the post's header: the episode's square artwork at the start on a wide screen, then the date, the topic and the reading time, the title, the post's description and the hosts with their photos.
- A folded contents list for long show notes, then the body in a narrow column. Put the sound player at the top of the body, so it comes straight after the header. It sits in a soft grey panel with the episode's title over the controls.
- After the body: the post's tags, the share links and the author box in a soft grey panel. After the post: the newsletter, comments when they are open, and three related episodes in the same rows as the front page.

**Lists and author pages** show posts in the same rows, under a title in Wave's size.

On a phone the artwork in the player card hides, as in Wave, and each row shrinks to small artwork beside the date and title, without the summary.

Everything reads with scripts off. Wave brings its own look: near-black titles over slate grey text on a white page, with one orange for the play marks, soft 6px corners and Poppins when the reader has it. Your theme tokens, such as the ones theme sync writes, replace any of those colours and fonts, and the logo comes from them too.

## Options

Set these under `[template.options]`. Each line shows the default.

| Option | Default | What it changes |
|---|---|---|
| `listen_on` | none | Up to four places to listen, each written `name \| address`, such as `"Spotify \| https://open.spotify.com/show/1"`. Only an address that starts with `https://` or `http://` shows. Nothing shows until you write them. |
| `podcast_section` | `true` | Whether the front page leads with the newest episode and its player. With `false`, the list starts under the title. |
| `logo` | `true` | Whether the header shows your logo from `[brand] logo` in place of the blog's name. |
| `dek` | `true` | Whether a post shows its `description` under its title. |
| `updated_note` | `false` | Whether an updated post says why it was updated, under its byline. |
| `cards` | `"row"` | The shape of the post cards in lists: `image`, `text`, `row`, `compact` or `ranked`. Only `row` gives the rows with square artwork. |
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
name = "wave"
[template.options]
listen_on = [
  "Apple Podcasts | https://podcasts.apple.com/podcast/id1",
  "Spotify | https://open.spotify.com/show/1",
  "YouTube Music | https://music.youtube.com/playlist?list=1",
]
```

To give an episode its player, put an audio block at the top of the post, with the sound file in the post's folder or at a full address:

```markdown
::audio{src="episode-12.mp3" title="Episode 12: a conversation with Maren Holt"}
```

## Sizes and spacing

Wave sets these in place of classic's. A value in your own `theme/tokens.toml` wins.

```toml
[colors]
background = "#ffffff"
surface = "#f6f6f6"
text = "#1a1a1a"
muted = "#5a656b"
link = "#b05000"
accent = "#c45a00"
border = "#e6e6e6"

[fonts]
heading = "Poppins, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif"
body = "Poppins, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif"

[shape]
radius = "0.375rem"
measure = "37rem"

[type]
body = "1.0625rem 1.125rem"
h1 = "2rem 2.5rem"
h2 = "1.5rem 1.75rem"
h3 = "1.25rem 1.5rem"
h4 = "1.0625rem 1.125rem"
small = "0.8125rem"
line_height = 1.65
heading_weight = 600

[space]
unit = "0.5rem"
section = "4rem"
```

The stylesheet adds its own sizes on top of these: the front page's and list pages' titles grow to 2.5rem on a wide screen, a post's title to 2.125rem, and an episode's title in a list to 1.75rem, as Wave sets them. The frame is 52.5rem, the width of Wave's list of episodes. A post keeps to the 37rem measure, about 72 characters a line; with the contents beside it, the post grows by 18rem so the text keeps that measure.

Wave's orange is darkened here, from `#fc7e0f` to `#c45a00`, so the play marks and buttons keep their contrast on white.

## Reader features

Wave keeps every `[reader]` feature as it is. Comments stay off unless your site runs as a server with comments on.

## From Ghost's Wave theme

Wave is a rewrite of Ghost's Wave theme for Khalid's templates and tokens. Some of Ghost's parts work differently here:

- Ghost's Wave plays an episode in a player bar that slides up from the foot of the page, with a script. Here the newest episode's player sits in its card on the front page, and each post has its own player at the top of its notes. Both are the browser's own sound player, so they work with scripts off.
- The play mark on each row opens the episode's page, where the player is, instead of playing in place.
- Ghost's Wave takes the sound file's address from a post's social description. Here the sound comes from the post's own `::audio` block or embedded player.
- The links to Apple Podcasts, Spotify, YouTube Music and RSS in Ghost's footer are the `listen_on` option on the front page, for any service. Each shows a headphones mark, not the service's own logo.
- Ghost's Wave writes the title in huge grey letters where a post has no picture. Here a plain grey square holds the place.
- Ghost's members sign-up is Khalid's newsletter form. Ghost's comments and search are Khalid's own.
- Ghost's cover image behind the site's description is left out, since Khalid sets no text over pictures.
- Ghost's "load more" is numbered pages. Ghost's typography settings come from your theme tokens.

## Design

This is Wave's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** An interview show, or any podcast that wants a calm, light page.

**Reference.** Ghost's Wave theme: one narrow column on white, each episode a row with square rounded artwork and a round orange play button over it, small grey capitals for the date over a medium title, and a slim white player card with a hairline edge.

| Part | Wave |
|---|---|
| Type pairing | One round geometric sans for headings and text, headings at weight 600. With no theme, Poppins where the reader has it, then the system sans. |
| Colour | A white page, near-black titles, slate grey for text and summaries, a light grey for panels and placeholders, and one orange for the play marks, buttons and topic links. |
| Spacing rhythm | A base step of 0.5rem. Rows 4rem apart on a wide screen and 2rem on a phone. Sections 3 to 5rem apart. |
| Frame at 1440 px and wider | **52.5rem**, the width of Ghost's Wave list of episodes. Header, list and footer share it. |
| Cards and lists | One column of rows: square artwork of 12.5rem with 6px corners and a 3rem round play mark at its centre, then the date and topic in small capitals, the title and a short summary. The whole row is the link. |
| Phone header and menu | The logo and the menu button on one row. Rows shrink to 5rem artwork beside the date and title. |
| Link, card and focus states | Titles and links turn to the link colour on hover. Focus is a 3 px accent ring. |
| The one idea | A quiet show page where each episode is a square of artwork with a round play mark. Someone shown only the thumbnail should name it by the stack of square artworks with orange play marks beside plain titles on white. |
| Type scale | Page and list titles 1.75 to 2.5rem, a post's title 1.75 to 2.125rem, episode titles in lists 1.0625 to 1.75rem, labels and dates 0.8125 to 0.875rem in capitals. |
| Signature details | The round play mark on every artwork. The slim player card with a hairline edge. The small grey capitals over each title. Soft 6px corners. |

**Sample brand.** The demo uses `fixtures/showcase-brands/wave/theme/`: deep orange (`#a34600` links, `#c25400` accent) on a warm off-white (`#fdfcfa`), DM Sans for headings and text, close to Wave's Poppins, and a round play mark beside the name.

---
title: "Solo"
order: 85
---

# Solo

A personal site for one writer or creator: a large headline with a sign-up field, then a feed of posts with large titles.

```toml
[template]
name = "solo"
```

Based on Ghost's Solo theme (MIT).

## Who it suits

Solo suits one person who writes, such as a writer, a consultant, a researcher or a maker who wants a home for their work. A reader lands on a front page that starts with you: your photo, one large line about who you are, and a field to sign up for your posts. Below it, every post has a title set nearly as large, so a few strong titles carry the page.

Pick another template if several people write for your blog under a company name, or if you post many short items a day. For a team, Keystone or Folio fits better. For news, Wire does.

## What each page shows

**Front page**
- A round photo of the newest post's author, when that author has one. Turn it off with `profile_photo = false`.
- The headline, set very large. It is your `headline` option, or the blog's name when you leave it empty.
- The front page's intro under it, when there is one, and up to four numbers from `hero_stats`.
- A sign-up field shaped like a pill, with the button inside it, when your site takes sign-ups and `newsletter` is `home` or `both`.
- The feed of posts, in the layout `feed` names:
  - `classic`: each post's title beside its picture, the picture on the end side, with the date, the reading time and the category under the title. On a phone the picture comes first.
  - `typographic`: very large titles and their summaries, with no pictures, far apart.
  - `parallax`: each post as a tall card. Its picture fills the card under a dark shade, and the title sits at the bottom in the page colour. Ghost's theme moves the picture as the reader scrolls. Here the picture stays still, so the page needs no script.
- When `[[reader.home_sections]]` lists sections, they take the place of the feed, as on every front page.
- Numbered page links when the list is long, the topics as outlined pills, and the band with your `[template.cta]` when `cta_band` asks for it.

**A post**
- The title, very large, across three quarters of the page, then the summary (`description`) under it.
- The lead picture at the full width of the page.
- On a wide screen, the byline sits in a column on the start side and stays in view as the text scrolls: each author's photo above their name and role, one date (the update when there is one), the reading time and the category as an outlined pill. On a phone the byline comes before the text.
- The contents as a folded box at the top of a long post.
- The body in one readable column with large, heavy headings.
- After the body: the share links, the tags, the author box, then three related posts as large titles, the call to action and the comments.

**Lists and author pages** set the list's name as large as a post title. Each post shows its title beside its picture on a wide screen, with the date, the reading time and the category under it, then its summary. An author's page shows the photo over the name, then the bio in large type.

Everything reads with scripts off. Solo brings its own look: near-black ink on white, one modern sans set heavy and tight in headings, square pictures and pill-shaped buttons and labels. Your theme tokens, such as the ones theme sync writes, replace any of those colours and fonts, and the logo comes from them too.

## Options

Set these under `[template.options]`. Each line shows the default.

| Option | Default | What it changes |
|---|---|---|
| `feed` | `"classic"` | How the front page lists posts: `classic`, `typographic` or `parallax`. |
| `headline` | empty | The large line at the top of the front page, up to 140 characters, such as who you are. Empty shows the blog's name. |
| `profile_photo` | `true` | Whether the front page shows a round photo of the newest post's author above the headline. |
| `logo` | `true` | Whether the header shows your logo from `[brand] logo` in place of the blog's name. |
| `dek` | `true` | Whether a post shows its `description` as a summary under its title. |
| `updated_note` | `false` | Whether an updated post says why it was updated, under its date. |
| `cards` | `"image"` | The shape of the post cards on list and author pages: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `true` | Whether each post in the feed and on lists shows how long it takes to read. |
| `pagination` | `"numbered"` | The links under a long list: `pages` for newer and older, or `numbered` for a link to each page. |
| `contents` | `"inline-folded"` | Where a post's contents go: `inline-folded`, `inline`, `side-start`, `side-end` or `none`. Both side placements put the contents in the byline column. |
| `reading_progress` | `"bar"` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` or `none`. |
| `related` | `"list"` | How the related posts are laid out: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show, from 3 to 10. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "linkedin", "bluesky", "email"]` | The networks the share links go to, in the order they show. |
| `newsletter` | `"home"` | Where the sign-up form shows: `none`, `post`, `home` or `both`. On the front page it is the pill under the headline. It needs `[mail]`, so a static site shows none. |
| `cta_band` | `"none"` | Where the band with your `[template.cta]` shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | Whether the header shows your `[template.cta]` as a button. |
| `hero_stats` | none | Up to four numbers under the headline, each written `number \| what it counts`. Nothing shows until you write them. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. They work with scripts off. |
| `video_section` | `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `false` | A row of the newest posts with a podcast or other sound player, when there are any. |

A full example:

```toml
[template]
name = "solo"
[template.options]
feed = "typographic"
headline = "I write about small gardens and the people who keep them."
newsletter = "both"
```

## Sizes and spacing

Solo sets these in place of classic's. A value in your own `theme/tokens.toml` wins.

```toml
[colors]
background = "#ffffff"
surface = "#f2f2f2"
text = "#15171a"
muted = "#5b5f66"
link = "#15171a"
accent = "#15171a"
border = "#e3e3e3"

[fonts]
heading = "Manrope, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif"
body = "Manrope, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif"
mono = "\"JetBrains Mono\", ui-monospace, Menlo, Consolas, \"Liberation Mono\", monospace"

[type]
body = "1.0625rem 1.25rem"
h1 = "2.5rem 4.5rem"
h2 = "1.75rem 2.5rem"
h3 = "1.375rem 1.75rem"
h4 = "1.125rem 1.25rem"
small = "0.875rem"
line_height = 1.7
heading_weight = 800

[space]
unit = "0.5rem"
section = "5rem"

[shape]
radius = "0px"
measure = "42rem"
```

The fonts name Manrope first, but Solo ships no font file. A reader who has Manrope sees it; anyone else sees the system's own sans. To serve Manrope or another font to every reader, add it to your `theme/tokens.toml`, as theme sync does.

The stylesheet adds its own display sizes on top of these: the headline grows to 3.625rem, a feed title to 3.25rem (4rem in the typographic and parallax feeds) and a post or list title to 4.5rem. The frame is 84rem. On a wide screen a post's text takes the middle half of the frame, about 39rem, with the byline in the quarter on the start side.

## Reader features

Solo keeps every `[reader]` feature as it is. Turn the topics row off with `topic_links = false`, and comments stay off unless your site runs as a server with comments on.

## What Solo leaves out

Ghost's Solo theme has a few parts that need Ghost itself. Here is what each became:

- Members and paid tiers: the sign-up pill uses Khalid's own newsletter form. There are no paid tiers.
- Ghost comments: Khalid's own comments show under a post when your site has them on.
- Ghost search: the header uses Khalid's own search, on a served blog.
- The parallax effect: the picture stays still under the title.
- Infinite scroll: numbered page links.
- The three header layouts (side by side, large background, typographic profile) and the background colour setting: Solo uses one layout, with a round photo above the headline. Your background colour comes from your tokens.
- Previous and next links under a post: Solo shows related posts instead.

## Design

This is Solo's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** One person who writes and wants their name and their titles to lead.

**Reference.** Ghost's Solo theme, the theme this template is based on: a large about headline with a subscribe field, then a feed in a classic, typographic or parallax layout.

| Part | Solo |
|---|---|
| Type pairing | One modern geometric sans for headings and text, headings at weight 800 with tight tracking. With no theme, the pack names Manrope and falls back to the system sans. |
| Colour | A plain background with near-black ink for text, links and buttons. A faint grey panel for the sign-up pill, the contents box and the call to action. Hairlines are rare. |
| Spacing rhythm | A base step of 0.5rem. Feed items 3 to 5rem apart (5 to 10rem in the typographic feed). Sections 5rem apart. |
| Frame at 1440 px and wider | **84rem**, the main part 1344 px wide at 1440 px, with a side gutter of 1.5 to 3rem. |
| Cards and lists | No boxes. In the classic feed and on lists, the title and its meta line sit beside a 16:9 picture on the end side, centred on it. The whole post is the link. |
| Phone header and menu | The logo and a round Menu button on one row; the menu folds open below. |
| Link, card and focus states | Body links in ink, underlined. A feed title fades to a lighter ink on hover. Focus is a 3 px accent ring; on a dark parallax card the ring is in the page colour. |
| The one idea | One voice at full volume: a headline set as large as the screen allows over a pill-shaped sign-up field, then titles nearly as large down the page. Someone shown only the thumbnail should name it by the size of the type and the space around it. |
| Type scale | Display sizes sit well above the token scale: the headline 2 to 3.625rem at line height 1.2, feed titles 2 to 3.25rem (2 to 4rem in the typographic and parallax feeds) at 1.1, post and list titles 2.5 to 4.5rem at 1. |
| Signature details | The sign-up pill with its button inside. Meta lines with small dots between the date, the reading time and the category. The byline column that stays in view beside the text. The category as an outlined pill. Square pictures. |

**Sample brand.** The demo uses `fixtures/showcase-brands/solo/theme/`: ink (`#1b1a17`) on warm paper (`#f6f3ee`), links in burnt orange (`#8f3a17`), Manrope for headings and DM Sans for text, and a round monogram mark.

## Credit

Based on Ghost's Solo theme (MIT). Ghost's licence text is kept beside the template's files in `crates/khalid-theme/packs/solo/LICENSE-ghost.txt`.

---
title: "Edge"
order: 62
---

# Edge

A picture-led site for creative work: a wall of lead pictures on the front page, and posts that open with a wide picture above one calm reading column.

```toml
[template]
name = "edge"
```

Based on Ghost's Edge theme (MIT).

## Who it suits

Edge suits photographers, illustrators, designers and small studios who want the work itself to be the front page. A reader lands on a wall of pictures with no text between them. Each picture is a link to its post, and each keeps its own shape, so tall and wide pictures pack together like a pinboard.

Pick another template if your posts have no strong lead picture, or if readers come for the words first. For a text-led blog, Ledger fits better. For a company blog with pictures and summaries, Keystone does.

## What each page shows

**Front page**
- A light header: your logo or name in the middle, search on the start side, and the menu centred on its own row between two hairlines. On a phone: the name, a search icon and a Menu button.
- The front page's intro, small and centred, when there is one, and up to four numbers from `hero_stats`.
- The wall: every post's lead picture, newest first, reading across the rows. Two columns on a phone, three on a tablet and a laptop, four from 1350 px and five from 1680 px. Each picture spans grid rows in proportion to its height, so the wall packs like masonry with no script. A post with no picture shows its title in the middle of a grey square.
- With `captions = "below"`, each picture shows its title, category and date under it, and the wall becomes columns, read down each column. With the default, `hidden`, the titles are still there for screen readers.
- When `[[reader.home_sections]]` lists sections, they take the place of the wall, as on every front page.
- Numbered page links, the topics as outlined pills, the newsletter and the band with your `[template.cta]` when their options ask for them.

**A post**
- The lead picture first, up to 75rem wide.
- The title, at a modest size, and the summary (`description`) under it in grey.
- One quiet line with the authors, the date (the update when there is one), the reading time and the category, with dots between them. The reviewers and the update note follow on their own lines.
- The share links as pills at the end of the column, when `share` is `top` or `both`.
- The contents as a folded box, then the body in one 40rem column.
- After the body: the share links when `share` is `end` or `both`, the tags, the author box, the newsletter, the call to action, the posts that link here and the comments, all in the same column.
- The related posts as the same wall of pictures as the front page.

**Lists and author pages** set the list's name as a plain heading over columns of pictures, each with its title and date under it. A post with no picture shows its title and summary. An author's page shows the photo beside the name, then the bio in the reading column.

Everything reads with scripts off. Edge brings its own look: near-black ink on white, no accent colour, square pictures, one modern sans and pill-shaped buttons. Your theme tokens, such as the ones theme sync writes, replace any of those colours and fonts, and the logo comes from them too.

## Options

Set these under `[template.options]`. Each line shows the default.

| Option | Default | What it changes |
|---|---|---|
| `captions` | `"hidden"` | Whether the picture wall shows each post's title, category and date under its picture (`below`) or shows the pictures alone (`hidden`). |
| `logo` | `true` | Whether the header shows your logo from `[brand] logo` in place of the blog's name. |
| `dek` | `true` | Whether a post shows its `description` as a summary under its title. |
| `updated_note` | `false` | Whether an updated post says why it was updated, under its date. |
| `cards` | `"image"` | The shape of the post cards on list and author pages: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | The links under a long list: `pages` for newer and older, or `numbered` for a link to each page. |
| `related_count` | `6` | How many related posts show under a post, from 3 to 10. |
| `contents` | `"inline-folded"` | Where a post's contents go: `inline-folded`, `inline`, `side-start`, `side-end` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` or `none`. |
| `reading_progress` | `"bar"` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `share` | `"top"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | The networks the share links go to, in the order they show. |
| `newsletter` | `"post"` | Where the sign-up form shows: `none`, `post`, `home` or `both`. It needs `[mail]`, so a static site shows none. |
| `cta_band` | `"none"` | Where the band with your `[template.cta]` shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | Whether the header shows your `[template.cta]` as a button, on the end side. |
| `hero_stats` | none | Up to four numbers under the front page's hidden title, each written `number \| what it counts`. Nothing shows until you write them. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. They work with scripts off. |
| `video_section` | `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `false` | A row of the newest posts with a podcast or other sound player, when there are any. |

A full example:

```toml
[template]
name = "edge"
[template.options]
captions = "below"
related_count = 8
cta_button = true
```

## Sizes and spacing

Edge sets these in place of classic's. A value in your own `theme/tokens.toml` wins.

```toml
[colors]
background = "#ffffff"
surface = "#f6f6f6"
text = "#15171a"
muted = "#5f6368"
link = "#15171a"
accent = "#15171a"
border = "#e6e6e6"

[fonts]
heading = "Inter, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif"
body = "Inter, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif"
mono = "Menlo, Consolas, Monaco, \"Liberation Mono\", \"Lucida Console\", monospace"

[type]
body = "1.125rem 1.1875rem"
h1 = "1.75rem 2rem"
h2 = "1.375rem 1.625rem"
h3 = "1.1875rem 1.375rem"
h4 = "1.0625rem 1.125rem"
small = "0.875rem"
line_height = 1.6
heading_weight = 600

[space]
unit = "0.5rem"
section = "6rem"

[shape]
radius = "0px"
measure = "40rem"
```

The fonts name Inter first, but Edge ships no font file. A reader who has Inter sees it; anyone else sees the system's own sans. To serve Inter or another font to every reader, add it to your `theme/tokens.toml`, as theme sync does.

The frame follows the wall: 61.875rem (990 px) from 1024 px, 82.5rem (1320 px) from 1350 px and 103.125rem (1650 px) from 1680 px, with a side gutter of 1 to 2.25rem. A post's lead picture is up to 75rem wide and its text 40rem.

## Reader features

Edge keeps every `[reader]` feature as it is. Turn the topics row off with `topic_links = false`, and comments stay off unless your site runs as a server with comments on.

## What Edge leaves out

Ghost's Edge theme has a few parts that need Ghost itself or a script. Here is what each became:

- The masonry script: a CSS grid. Each picture spans rows in proportion to its height, so the wall still reads across in date order. With captions below, the wall is CSS columns instead.
- The lightbox on the wall: each picture opens its post. Pictures in a post open at full size when your blog has `enlarge_images` on.
- Infinite scroll: numbered page links.
- Members, sign-in and paid tiers: the sign-up form uses Khalid's own newsletter. There are no paid tiers.
- Ghost comments: Khalid's own comments show under a post when your site has them on.
- Ghost search: the header uses Khalid's own search, on a served blog.
- The three header layouts: Edge uses the stacked one, with the menu on its own row, so a long menu never crowds the name.
- The title and body font settings: your fonts come from your tokens.

## Design

This is Edge's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A photographer, illustrator or studio whose pictures should do the talking.

**Reference.** Ghost's Edge theme, the theme this template is based on: a masonry wall of feature images under a light, centred header, and a simple post with the feature image on top.

| Part | Edge |
|---|---|
| Type pairing | One modern sans for everything, headings at weight 600 with tight tracking. With no theme, the pack names Inter and falls back to the system sans. |
| Colour | A plain background with near-black ink for text, links and buttons, and no accent colour. A faint grey for tiles with no picture, the contents box and panels. Hairlines only around the menu. |
| Spacing rhythm | A base step of 0.5rem. About 32 px between pictures on a wide screen and 8 px on a phone. Sections 6rem apart. |
| Frame at 1440 px and wider | **82.5rem** (1320 px) at 1440 px, the width of a four-column wall; 103.125rem from 1680 px. |
| Cards and lists | No boxes. The wall is pictures alone, each a link. Lists show columns of pictures with a small title and date under each. |
| Phone header and menu | The name, a search icon and a Menu button on one row; the menu folds open below. |
| Link, card and focus states | Body links in ink, underlined. A picture dims slightly on hover. Focus is a 3 px accent ring. |
| The one idea | The work is the front page: a wall of pictures with nothing between them. Someone shown only the thumbnail should name it by the packed pictures and the empty white around them. |
| Type scale | Small and calm. A post title 1.75 to 2rem, the body 18 to 19px, wall titles 0.875 to 1.125rem when shown. |
| Signature details | The name centred in the header with the menu on its own row. The packed picture wall. The lead picture above a short title. Pill-shaped share links. |

**Sample brand.** The demo uses `fixtures/showcase-brands/edge/theme/`: ink (`#161616`) on white, a warm grey (`#f4f3f1`) for tiles and panels, links in a deep fern green (`#2f4a3a`), Inter for headings and text, and a small framed mark.

## Credit

Based on Ghost's Edge theme (MIT). Ghost's licence text is kept beside the template's files in `crates/khalid-theme/packs/edge/LICENSE-ghost.txt`.

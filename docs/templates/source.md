---
title: "Source"
order: 85
---

# Source

Source is a template for a general publication: a blog, a magazine or a newsletter's website. It has the logo in the middle of the header, a front page that opens with one of five headers, a list or a grid of posts with thin lines between them, and a calm column for each post under a wide picture.

Based on Ghost's Source theme (MIT). Khalid's copy is rewritten for Khalid's templates and keeps the theme's licence beside its files.

```toml
[template]
name = "source"
```

## Who it suits

- A writer or a small team that wants a familiar, flexible look and may change it as the blog grows.
- A new blog: the landing header puts the blog's name and the sign-up form first.
- A busy blog: the highlight and magazine headers show seven to ten posts at the top of the front page.
- Readers who skim: every list has thin lines between posts and short summaries.

## What the front page shows

The `header_style` option picks how the first page opens:

1. **Landing**, the default: the blog's title large and centred, its intro under it (or its description when it has no intro), then the sign-up form as a grey pill with the button inside its end. With no sign-ups, the title and intro stand alone.
2. **Search**: the same, with a search field in place of the sign-up form, on a blog that answers searches. A blog without search shows the landing header.
3. **Highlight**: the newest post large, with its topic over its title, then a column of the next three posts, then a column of small rows with square pictures: the editor's picks when `picks` is above 0 and posts are marked featured, or else the next six posts.
4. **Magazine**: the newest post large in the middle, the next three posts in a column before it and three more after it. Then the editor's picks as a row of four small rows with square pictures, under a **Featured** label, when `picks` is above 0.
5. **Off**: no header; the list starts at the top.

Highlight and magazine then show the newsletter form as a grey band with a large title, when your blog takes sign-ups. Under the header comes every post it did not show, under **Latest posts**. The `cards` option sets their shape: `row` is Source's list, with a picture 220 px wide beside each post; `image` is Source's grid, with columns at least 248 px wide; `text` is the list without pictures. With highlight or magazine, the grid uses the whole width.

With `about_sidebar` on, a grey column beside the list shows the blog's name, its intro and your call to action as a button. It shows on screens 1200 px wide and wider.

A post shows once on the page. An editor's pick that is already at the top is left out of the picks. Later pages of the front page show only the list, with numbered page links under it.

## What a post shows

1. The post's topic in small capitals, in your link colour.
2. The title, large, and the subtitle under it.
3. One row with the authors' round photos overlapping, their names, the date (the date of the last update when there is one), the reading time and, under it, the date it was first published and who reviewed it. At the end of the row, a **Share this post** pill opens the share links in a small menu. It needs no script.
4. The lead picture, wider than the text: up to 1120 px.
5. The contents, folded under one line.
6. The post in one column 720 px wide, 17 px text. With `drop_cap` on, its first letter is set large over the first lines.
7. The tags as pills.
8. The newsletter form as a grey band, then the posts that link here and the comments.
9. Four related posts as a row of picture cards, across the whole width, under **Related posts**.

Source turns the author box off. Turn it on in `[reader]` with `author_box = true` if you want it.

## List pages

A topic's or a tag's page shows its name large, its intro, a thin line, then the list in the same shape as the front page. An author's page shows the author's round photo beside their name, role and profiles, their bio, then their posts.

## The header

The header has your menu links at the start, the logo or the blog's name in the middle and your call to action as a pill at the end. With six menu links or more, the header stacks, as Source's stacked layout does: the logo in the middle of the top row and the links centred under it between two thin lines. On a phone, the logo sits at the start and the menu opens from a Menu button, with no script.

## Your brand

Source brings its own colours: near-black ink on white, grey lines and a grey field for the forms and bands, and an ink accent for buttons. Your colours, fonts and logo in `theme/tokens.toml`, such as the ones theme sync writes, replace them, so it looks like your main site. Buttons and forms are pills in the accent colour; pictures have square corners.

Source's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Source's default |
|---|---|
| `[fonts] heading` and `body` | Inter where the reader has it, else the system's sans |
| `[type] body` | `1rem`; a post's text is 17 px |
| `[type] h1` | `2.125rem` on a phone to `2.875rem` on a wide screen |
| `[type] h2` | `1.5rem` to `1.75rem` |
| `[type] h3` | `1.25rem` to `1.375rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0px`: pictures and bands have square corners |
| `[shape] measure` | `45rem`, the width of a post's text |
| `[space] section` | `4rem` |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `header_style` | `"landing"` | How the front page opens: `landing`, `highlight`, `magazine`, `search` or `off`. |
| `about_sidebar` | `false` | A column beside the front page's list with the blog's name, its intro and your call to action. |
| `drop_cap` | `false` | The first letter of each post is set large over its first lines. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description as a subtitle under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"row"` | The shape of the cards in every list: `row` (Source's list), `image` (Source's grid), `text`, `compact` or `ranked`. |
| `card_reading_time` | `false` | Each card shows how long the post takes to read. |
| `picks` | `0` | How many posts marked featured the highlight and magazine headers show as editor's picks, from `0` to `8`. The magazine row shows up to four. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related` | `"grid"` | How related posts are laid out: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `4` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline-folded`, `inline`, `side-end`, `side-start` or `none`. |
| `reading_progress` | `"bar"` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `contents_numbers` | `"numbered"` | How the contents are numbered: `numbered`, `nested` for 1, 1.1, or `none`. |
| `share` | `"top"` | Where a post shows its share links: `top` as the Share menu in the byline row, `end` as a row after the post, `both`, or `none`. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. On the front page it sits in the landing header, or as a band under the highlight and magazine headers. |
| `cta_band` | `"none"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a pill. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its newest posts. They work with scripts off. |
| `video_section` | `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `false` | A row of the newest posts with a podcast or other sound player, when there are any. |

The call to action comes from `[template.cta]`; see [the templates guide](../templates.md#your-call-to-action). Without it, the header pill, the sidebar button and the band do not show.

For example, a busy blog with a magazine header, four editor's picks and a grid:

```toml
[template]
name = "source"
[template.options]
header_style = "magazine"
picks = 4
cards = "image"
```

## What Source leaves out

Source's Ghost features that a Khalid blog does not have are mapped or left out:

- Members, paid tiers and the sign-in link: Source shows Khalid's newsletter form where Ghost shows its sign-up form, and the call to action where Ghost shows its Subscribe button.
- Ghost's comments, search and share window: Khalid's own comments, search page and share links take their place.
- Infinite scroll and "See all": numbered page links.
- The header's cover picture, the site background colour setting, the accent-coloured header and footer, the serif and mono title fonts, and the logo-on-the-left layout: set colours and fonts in `theme/tokens.toml` instead.
- The recommendations list in the sidebar.

## Good to know

- Everything reads with scripts off. Source adds no script.
- The newsletter form shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table. It signs readers up to your default list.
- The front page shows the form once. One form per page keeps every field label unique.

## Design

This is Source's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A writer or a small team that wants Ghost's familiar default look, with room to grow from a simple landing page to a busy magazine front page.

**Reference.** Ghost's Source theme, its templates and its stylesheet.

| Part | Source |
|---|---|
| Type pairing | Inter for headings and text, weight 700 for titles and 550 for the menu. With no theme, the system's sans. |
| Colour | White ground, near-black ink, grey hairlines and a grey field for forms and bands. One accent for buttons and the topic over a post's title. |
| Spacing rhythm | A 42 px gap between columns and between posts, with a hairline in the middle of each gap. 120 px before the footer. |
| Frame at 1440 px and wider | At least **82.5rem** (1320 px). |
| Cards and lists | No box: a picture, the title, two lines of summary and a small meta line. Lists put a 220 px picture beside the words; grids put it on top. |
| Phone header and menu | The logo at the start and a Menu button at the end; the call to action moves into the menu. |
| Tabs and indexes at 390 px | Every column stacks; the small picks rows keep their square picture at the end. |
| Link, card and focus states | A card's title fades on hover and its link covers the whole card. Links in a post are underlined in the link colour. Focus is a 3 px accent ring. |
| Signature details | The logo in the middle of the header. Small capital labels on a hairline over each list. Pill buttons and the pill sign-up field with the button inside it. The overlapping author photos beside the Share pill. |

**Sample brand.** The demo uses `fixtures/showcase-brands/source/theme/`: a magenta accent (`#c8175a`, links `#b0134c`) on white with a warm grey field (`#f5f1ed`), Inter for headings and text, and a round mark with the showcase's initial.

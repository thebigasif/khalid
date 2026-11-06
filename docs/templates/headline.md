---
title: "Headline"
order: 87
---

# Headline

Headline is a template for local news, or for any site that covers many topics at once. The logo sits over the menu. The front page shows the latest stories, then a section for each topic with a lead story and small rows, then short lists of headlines for more topics. Posts read in one calm column with the byline in the margin, and the footer is a dark band.

Based on Ghost's Headline theme (MIT). Khalid's copy is rewritten for Khalid's templates and keeps the theme's licence beside its files.

```toml
[template]
name = "headline"
```

## Who it suits

- A local newsroom, a community paper or a trade publication with several beats.
- A blog with many categories, where readers want to see every topic on one page.
- Readers who skim: every section shows one story large and the rest as short rows.

## What the front page shows

1. **Latest posts**: the newest posts, as many as `latest_count`. The first is large, with its picture beside its title, summary, author and date. The rest are small rows two across: a small picture beside the title, the author and the date. Under them, an **All posts** link to the archive when there are more posts.
2. **A section per topic**: the same shape for each topic, under the topic's name in capitals, with a link to all of its posts under a thin line. Each shows the topic's seven newest posts. The topics are the sections set in `[[reader.home_sections]]`, or else the topics whose newest posts are newest. `topic_sections` sets how many.
3. **Short lists**: more topics, three across, each with its name, a **More** link and its five newest headlines. `topic_lists` sets how many. A topic that already has a section is not listed again.
4. The topic links, the topic tabs and the video and podcast rows, when they are on.
5. The newsletter form as a dark band that runs into the dark footer, when your blog takes sign-ups.

A post can show in Latest and again in its topic's section, as in Ghost's Headline.

## What a post shows

1. The post's topic in small capitals, in your link colour.
2. The title, large, and the subtitle under it.
3. The byline: the authors' round photos, their names in bold, the date (the date of the last update when there is one), the reading time, who reviewed it and a **Share this post** button that opens the share links with no script. On a screen 1200 px wide or wider, the byline sits in the margin beside the lead picture. On a smaller screen it is a row under the subtitle.
4. The lead picture, as wide as the text. With `post_image = "wide"`, as wide as the page.
5. The post in one column, 18 px text in the body font, 60 to 75 characters a line.
6. The tags as grey pills.
7. The newsletter form, the posts that link here and the comments.
8. Three related posts as small rows, under **Related posts** with a thin line after it.

Headline turns the author box off. Turn it on in `[reader]` with `author_box = true` if you want it.

## List pages

A topic's or a tag's page shows its name in capitals over a thin line, its intro, then its posts as rows in a centred column: a picture 160 px wide beside the title, two lines of summary, the author and the date. An author's page shows a small round photo beside the name, role and profiles, the bio, then the posts under a label with a thin line after it.

## The header

The header is stacked: the logo or the blog's name in the middle, the search link at the start and your call to action as a pill at the end. Under them, the menu is centred between two thin lines that run across the whole screen. On a phone, the logo sits at the start and the menu opens from a Menu button, with no script.

## Your brand

Headline brings its own colours: near-black ink on white, light grey lines, a grey field for tags and boxes, an ink accent for buttons, and a dark footer in the ink colour. Your colours, fonts and logo in `theme/tokens.toml`, such as the ones theme sync writes, replace them, so it looks like your main site. The footer always takes your text colour as its ground and your page colour as its text.

Headline's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Headline's default |
|---|---|
| `[fonts] heading` | Inter where the reader has it, else the system's sans |
| `[fonts] body` | Lora where the reader has it, else Georgia |
| `[type] body` | `1rem`; a post's text is 18 px |
| `[type] h1` | `2.25rem` on a phone to `2.75rem` on a wide screen |
| `[type] h2` | `1.5rem` to `1.75rem` |
| `[type] h3` | `1.25rem` to `1.5rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0px`: pictures have square corners |
| `[shape] measure` | `37rem`, the width of a post's text |
| `[space] section` | `5rem` |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `latest_count` | `7` | How many of the newest posts show under Latest, from `3` to `13`. |
| `topic_sections` | `3` | How many topics get a section with a lead story and small rows, from `0` to `6`. |
| `topic_lists` | `3` | How many more topics show as short lists of headlines, from `0` to `6`. |
| `post_image` | `"standard"` | How wide a post's lead picture is: `standard` (as wide as the text) or `wide` (as wide as the page). With the contents beside the post, it keeps to the text. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description as a subtitle under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"row"` | The shape of the cards on list pages: `row` (Headline's rows), `image`, `text`, `compact` or `ranked`. |
| `card_reading_time` | `false` | Each card shows how long the post takes to read. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related` | `"row"` | How related posts are laid out: `row`, `grid`, `list`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `contents` | `"none"` | Where a long post's contents go: `none`, as Ghost's Headline shows none, or `inline-folded`, `inline`, `side-end` or `side-start`. With a side column the byline stays under the subtitle. |
| `contents_numbers` | `"numbered"` | How the contents are numbered: `numbered`, `nested` for 1, 1.1, or `none`. |
| `reading_progress` | `"bar"` | A thin line at the top of a post that fills as the reader scrolls: `bar` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `share` | `"top"` | Where a post shows its share links: `top` as the Share button in the byline, `end` as a row after the post, `both`, or `none`. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. On the front page it is the dark band over the footer. |
| `cta_band` | `"none"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a pill. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its newest posts. They work with scripts off. |
| `video_section` | `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `false` | A row of the newest posts with a podcast or other sound player, when there are any. |

The call to action comes from `[template.cta]`; see [the templates guide](../templates.md#your-call-to-action). Without it, the header pill and the band do not show.

For example, a news site that picks its own front page sections, two of them, with five topic lists under them:

```toml
[template]
name = "headline"
[template.options]
topic_sections = 2
topic_lists = 5

[[reader.home_sections]]
category = "city-hall"

[[reader.home_sections]]
category = "schools"
```

## What Headline leaves out

Headline's Ghost features that a Khalid blog does not have are mapped or left out:

- Tag sections: Ghost picks tags by slug or by how many posts they have. Khalid uses your categories: the sections in `[[reader.home_sections]]`, or else the topics with the newest posts.
- Members, paid tiers, sign-in and the Subscribe button: Khalid's newsletter form takes the place of Ghost's footer sign-up, and your call to action takes the place of the Subscribe button.
- Ghost's comments, search and share window: Khalid's own comments, search page and share links take their place.
- "Show more" and infinite scroll: an **All posts** link to the archive, and numbered page links on list pages.
- The large featured story with its title over the picture: Khalid puts no text over pictures, so a featured story shows like any lead story.
- The full-screen cover on a tag page with a picture, the accent and dark header styles, the white logo for a see-through header, the serif title font and the footer text setting: set colours, fonts and the footer links in your site's settings instead.
- The gallery lightbox: Khalid's own larger-picture view, when `enlarge_images` is on.

## Good to know

- Everything reads with scripts off. Headline adds no script.
- The newsletter form shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table. It signs readers up to your default list.

## Design

This is Headline's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A local newsroom or any publication with many beats, that wants every topic visible from the front page.

**Reference.** Ghost's Headline theme, its templates and its stylesheet.

| Part | Headline |
|---|---|
| Type pairing | Inter for headings, menus, labels and bylines; Lora for summaries and posts. Section names in capitals. With no theme, the system's sans and Georgia. |
| Colour | White ground, near-black ink, light grey hairlines, a dark footer band in the ink colour. The link colour for the topic over a post's title. |
| Spacing rhythm | 80 px between sections, 32 px between rows and 40 px between the two columns. A hairline under each lead story and over each "more" link. |
| Frame at 1440 px and wider | At least **75rem** (1200 px). |
| Cards and lists | No box. A lead story with its picture beside the words; small rows with a picture 128 px wide beside the title; list pages with a picture 160 px wide in a 720 px column. Author and date in small capitals. |
| Phone header and menu | The logo at the start, the search link and a Menu button at the end; the call to action moves into the menu. |
| Sections at 390 px | One column; the lead story's picture goes on top; small rows keep a 100 px picture and hide the author. |
| Link, card and focus states | A card's title fades on hover and its link covers the whole card. Links in a post are underlined in the link colour. Focus is a 3 px accent ring. |
| Signature details | The stacked header with the menu between two hairlines across the screen. Capital section names. The byline in the margin beside the lead picture. The dark sign-up band and footer. |

**Sample brand.** The demo uses `fixtures/showcase-brands/headline/theme/`: a deep news red accent (`#c8102e`, links `#a8102a`) on white with a warm grey field (`#f4f2ef`), Inter for headings and Lora for text, and a red square mark beside the name.

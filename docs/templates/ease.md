---
title: "Ease"
order: 45
---

# Ease

A help centre or documentation site: a large search box first, then each topic as a list of links.

Based on Ghost's Ease theme (MIT).

```toml
[template]
name = "ease"
```

## Who it suits

Ease suits a product or service that answers its customers' questions: a help centre, a knowledge base or a set of how-to guides. A visitor lands on a band in your accent colour with a large search box, so they can look for an answer at once. Under it, your featured articles sit in a strip of picture cards. Then every category shows as a short list of its newest articles, two categories across, so a visitor can scan all topics on one screen.

Pick another template if your posts are long reads that people browse by picture, or if your blog has no categories.

## What each page shows

**Front page**
- A band across the full width in your accent colour with your blog's name and intro in the middle, and a large search box under them. The search box sends the words to the blog's search page. A static site has no search page, so it shows no box.
- "Featured": the posts marked `featured`, up to `picks` of them, as picture cards with a 2:1 picture, the title and the category as a small label. Four show across a wide screen, three on a tablet and one on a phone. The strip scrolls sideways when there are more, and snaps to each card.
- One section per category, two across on a wide screen and one under another on a phone: the category's name over a grey line, its four newest posts as links with a small arrow, then "View all posts in" that category in your link colour. The sections follow the order of each category's newest post, up to six categories.
- A last section, "Latest posts", with the five newest posts and a link to the older ones.
- When there are more categories than sections, the topic links follow.
- A blog without categories shows its newest posts as plain rows with numbered page links, as pages 2 and on do.

**A post**
- A large title, then one grey line with the date it was last updated and its category, and the share links.
- The lead picture, then the text in one column of 38rem, about 70 characters a line, a little larger than the rest of the site. With `contents` set to a side placement, the contents sit in a column beside the text and the post grows by that column, so the text keeps its width.
- The author box, when the site shows it. Then up to `related_count` related articles, two across, each with its category as a small grey label over its title in bold. Then comments when they are open.

**Lists**, such as a category or a tag, show the name and its description in grey, then each post as a plain row: the title in bold and two lines of its summary, with a grey line between rows. On a wide screen the rows sit two across over the full frame, as the front page sets its topics; on a phone or tablet they form one column. The whole row opens the post. Numbered page links follow.

**An author's page** shows a small round photo, the name, the role and the bio, then the author's posts as the same plain rows.

Everything reads with scripts off. Ease brings its own look: dark grey text on white, light grey lines, one violet for the band and the search button, and small rounded corners. Your theme tokens, such as the ones theme sync writes, replace any of those colours and fonts, and the logo comes from them too.

## Options

Set these under `[template.options]`. Each line shows the default.

| Option | Default | What it changes |
|---|---|---|
| `logo` | `true` | Whether the header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `false` | Whether each page shows the trail of links from the home page to it. |
| `dek` | `false` | Whether a post shows its `description` under its title. |
| `updated_note` | `false` | Whether an updated post says why it was updated, under its date. |
| `picks` | `4` | How many featured posts the front page shows in its strip, from 0 for none to 8. |
| `pagination` | `"numbered"` | The links under a long list: `pages` for newer and older, or `numbered` for a link to each page. |
| `related_count` | `4` | How many related articles show under a post, from 3 to 10. |
| `contents` | `"none"` | Where a long post's contents go: `side-end`, `side-start`, `inline`, `inline-folded` or `none`. Ghost's Ease has no contents list, so none shows until you set it. |
| `contents_numbers` | `"numbered"` | How the contents are numbered: `numbered`, `nested` for 1.1, or `none`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `share` | `"top"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "linkedin", "facebook", "email"]` | The networks the share links go to, in the order they show. |
| `newsletter` | `"none"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. It needs `[mail]`, so a static site shows none. |
| `cta_band` | `"none"` | Where the band with your `[template.cta]` shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | Whether the header shows your `[template.cta]` as a button. |

A full example:

```toml
[template]
name = "ease"
[template.options]
picks = 8
share = "end"
```

To put an article in the featured strip, mark it in its front matter:

```markdown
featured: true
```

## Sizes and spacing

Ease sets these in place of classic's. A value in your own `theme/tokens.toml` wins.

```toml
[colors]
background = "#ffffff"
surface = "#f7f7f7"
text = "#333333"
muted = "#666666"
link = "#5a4ad1"
accent = "#6c5ce7"
border = "#e6e6e6"

[fonts]
heading = "-apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif"
body = "-apple-system, BlinkMacSystemFont, \"Segoe UI\", Helvetica, Arial, sans-serif"

[shape]
radius = "0.3125rem"
measure = "38rem"

[type]
body = "1rem 1.0625rem"
h1 = "2rem 2.375rem"
h2 = "1.375rem 1.625rem"
h3 = "1.1875rem 1.375rem"
h4 = "1.0625rem 1.125rem"
small = "0.875rem"
line_height = 1.6
heading_weight = 700

[space]
unit = "0.5rem"
section = "4.5rem"
```

The stylesheet adds a few sizes of its own, as Ghost's Ease sets them: the band is 25 to 31.25rem tall, the search box 25rem wide and 3.125rem tall, section headings 1.375rem, and a list page's title 1.75rem. The frame is 75rem, and category and tag pages fill it with two columns of rows on a wide screen. A post and an author's page keep to the 40rem measure, about 70 characters a line; Ghost's Ease uses 45rem, which holds more than 75. With the contents beside a post, the post grows by 18rem so the text keeps that measure.

## Reader features

Ease keeps every `[reader]` feature as it is. Comments stay off unless your site runs as a server with comments on.

## From Ghost's Ease theme

Ease is a rewrite of Ghost's Ease theme for Khalid's templates and tokens. Some of Ghost's parts work differently here:

- Ghost's topic sections come from tags. Here each section is a category, with its four newest posts. Ghost lists the five oldest posts of each tag and every tag; Khalid shows up to six categories, and the topic links reach the rest.
- Ghost's search box opens Ghost's own search. Here it is a plain form that sends the words to Khalid's search page, so it needs no script. A static site has no search page, so the box does not show.
- Ghost's featured strip has arrow buttons that need a script. Here the strip scrolls sideways and snaps to each card, with no buttons.
- Ghost's links to the previous and next post become the related articles, in the same two columns with a small label over each title.
- Ghost's home page has no list of every post. Ease adds a short "Latest posts" section with a link to the older ones, so every post can be reached.
- Ghost's members sign-up is Khalid's newsletter form. Ghost's comments are Khalid's own.
- Ghost's cover picture with a dark layer over it is a band in your accent colour, as Ghost draws it when the site has no cover picture.
- Ghost's settings for the header layout and the title and body fonts come from your theme tokens.
- Ghost's Ease has no contents list. The `contents` option adds Khalid's, in the text or in a column beside it.
- Ghost's gallery lightbox and its scripts are left out. A post page runs no script of its own.

## Design

This is Ease's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A help centre, knowledge base or documentation site that readers search and scan.

**Reference.** Ghost's Ease theme: a tall white header, a band in the accent colour with a centred search box, a strip of four picture cards, then topics two across, each a list of links with chevrons under a heading with a grey line.

| Part | Ease |
|---|---|
| Type pairing | One plain sans for headings and text. With no theme, the pack uses the system sans for both, headings at weight 700. |
| Colour | White with dark grey text and light grey lines, and one accent for the band, the search button and the "View all" links. |
| Spacing rhythm | A base step of 0.5rem. Links in a topic 0.875rem apart. Topics 5rem apart down and 3.75rem across. |
| Frame at 1440 px and wider | **75rem**, as Ghost's Ease sets it. The band runs the full width of the screen. Category and tag pages fill it with rows two across. Posts and author pages keep to 40rem. |
| Cards and lists | Featured cards: a 2:1 picture with square corners, a title in 1.0625rem and a small round-ended label. Lists: plain rows with a bold title and two grey lines of summary, a hairline between. |
| Phone header and menu | The logo, a search mark and the menu button on one row. On a tablet, a menu too long for the row scrolls sideways inside its own box. Topics stack one under another; the featured strip shows one card and the edge of the next. |
| Link, card and focus states | Titles take the link colour on hover. Topic links underline on hover. Focus is a 3 px accent ring; in the band, a ring in the text colour. |
| The one idea | A help centre's front page: one search box, then every topic at a glance. Someone shown only the thumbnail should name it by the coloured band with a white search box over two columns of link lists. |
| Type scale | The blog's name in the band 1.5 to 2.25rem, post titles 2 to 2.375rem, section headings 1.375rem, topic links 0.9375rem. |
| Signature details | The search box with the square button inside its end. The chevron before each topic link. The arrow after each "View all" link. The small label over each related title. |

**Sample brand.** The demo uses `fixtures/showcase-brands/ease/theme/`: sea green (`#0b6b63` links, `#0f7a70` accent) on white, Georgia for headings, as Ease's "Elegant serif" title setting does, and the system sans for text, and a question mark in a circle as the logo.

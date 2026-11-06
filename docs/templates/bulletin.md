---
title: "Bulletin"
order: 25
---

# Bulletin

Bulletin is a template for a blog that is the home of a newsletter. Its front page asks readers to sign up first and shows the latest issue beside the form. Every other post reads as a past issue.

Based on Ghost's Bulletin theme (MIT).

```toml
[template]
name = "bulletin"
```

## Who it suits

- A writer or a small team that sends a weekly or monthly email and keeps every issue on the blog.
- A blog whose main goal is new subscribers, where the sign-up form should be the first thing a reader sees.
- Posts that read like letters: one column, no sidebar, few pictures.

It suits a blog with many topics less well. For that, Atlas or Wire reads better.

## What the front page shows

1. **The header:** your logo or the blog's name, the menu in the middle, then search and your call to action as a round button.
2. **The sign-up panel:** on the start side, a panel filled with your accent colour. It holds the blog's name and the `intro` of your home page, then the newsletter form: a heading, one line about the emails, and an email field drawn as a thick line with a round arrow button at its end. Your menu sits at the foot of the panel. When your blog takes no sign-ups, the panel shows your call to action in the same style, or a link to the archive.
3. **The latest issue:** on the end side, the newest post set like an email: a label with a blinking dot in the corner, the title large and centred, the writer and the date in small capitals, the lead picture and the summary. The words fade out above a large ink button that opens the issue.
4. **Past issues:** every other post on the page as one ruled column. Each row opens with its date in your link colour and in capitals, then the title in heavy type and the summary. The first row also shows its picture.
5. Numbered page links, then each home section, when you have them.

From 62rem the panel and the latest issue sit side by side and fill the window under the header. On a phone the panel comes first and the latest issue follows. The menu at the foot of the panel shows on wide screens only, since the header's menu button holds it on a phone.

A later page of the front page shows the blog's name and that page's posts in the same ruled column.

## What a post shows

1. The title, large and centred.
2. One line in small capitals: "By" and the writers, the topic in your link colour, the date, when it was updated and the reading time.
3. The subtitle, from the post's `description`, in the body font.
4. The share links, as a row of round buttons.
5. The lead picture, as wide as the text, wider, or from edge to edge (option `image_width`).
6. The post in one centred column of about 65 letters a line. Links are underlined in your link colour.
7. The author box, when it is on.
8. A ruled line of links in heavy capitals: "All posts" on the start side, and the archive on the end side.
9. The newsletter form, centred on a hairline, and the call to action band, when they are on.
10. Comments, then three related posts as a ruled list.

## List pages

A topic, tag, author or archive page shows its name, then its posts in the same ruled column as the front page. An author page puts the round photo beside the name and the bio.

## Your brand

Bulletin takes your colours, fonts and logo from `theme/tokens.toml`, so it looks like your main site. The sign-up panel is filled with your accent colour, with black or white text, whichever reads better. Headings, labels, menus and lists use your heading font; posts use your body font.

Bulletin's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Bulletin's default |
|---|---|
| `[colors]` | white page, near-black text, a pink accent `#d1105a` and link colour `#b80d4f` |
| `[fonts] heading` | the reader's own sans: San Francisco, Segoe UI, Helvetica or Arial |
| `[fonts] body` | the reader's own serif: Iowan Old Style, Charter, Sitka, Cambria or Georgia |
| `[type] body` | `1.0625rem` on a phone to `1.125rem` on a wide screen |
| `[type] h1` | `2rem` to `2.875rem` |
| `[type] heading_weight` | `800` |
| `[type] line_height` | `1.6` |
| `[space] section` | `4rem` |
| `[shape] radius` | `0.25rem`; buttons are round |

Nothing is downloaded for the fonts: each reader sees the sans and serif their own device has.

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `image_width` | `"small"` | How wide a post's lead picture runs: `small` keeps it to the text, `wide` runs it up to 75rem, `full` runs it from edge to edge. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `dek` | `true` | A post shows its `description` as a subtitle under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the cards on list pages: `text`, `image`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `false` | Each card shows how long the post takes to read. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `contents` | `"none"` | Where a long post's contents go: `none`, `inline`, `inline-folded`, `side-end` or `side-start`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` for 1, 1.1, or `none`. |
| `related` | `"list"` | How related posts are laid out: `list`, `grid`, `row`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `share` | `"top"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | Which share links show, in this order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. On the front page it fills the sign-up panel. |
| `cta_band` | `"none"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a round button. |
| `hero_stats` | none | Up to four numbers under the blog's name in the sign-up panel, each written `number \| what it counts`. |
| `topic_tabs` | `false` | Tabs under the past issues, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `false` | A row of the newest posts with a podcast or other sound player, when there are any. |

The call to action comes from `[template.cta]`; see [the templates guide](../templates.md#your-call-to-action).

For example, a site that wants wide lead pictures and no share links:

```toml
[template]
name = "bulletin"
[template.options]
image_width = "wide"
share = "none"
```

## Good to know

- Everything reads with scripts off. Bulletin adds no script. The dot beside "Latest posts" stops blinking for readers who ask their device for less motion.
- The newsletter form shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table.
- Ghost's members, paid tiers, comments and search are not part of Bulletin. Khalid's own newsletter form, comments and search take their places.
- Ghost loads more posts as a reader scrolls. Bulletin uses numbered pages, and the archive at `/archive/` lists every post.

## Design

This is Bulletin's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A writer or small team whose blog is the home of their newsletter. The first goal is a new subscriber; the second is reading the latest issue.

**Reference.** Ghost's Bulletin theme: the front page split into a sign-up panel in a punchy colour and the latest issue, an archive of issues in one ruled column, and a centred post.

| Part | Bulletin |
|---|---|
| Type pairing | A heavy geometric sans (weight 800) for headings, labels, menus and lists, over a calm text serif for posts. With no theme, the pack uses the system sans and the system serif. |
| Colour | White ground, near-black ink, light grey hairlines and one punchy accent that fills the sign-up panel and marks dates and links. Buttons to read are ink. |
| Spacing rhythm | A base step of 0.5rem. The panel has 3.5rem inside on a wide screen. List rows are 2rem apart under hairlines. |
| Frame at 1440 px and wider | The header and footer across **75rem**. Lists in one **45rem** column (720 px, the frame the layout check holds), posts in **37.5rem**. The front page's split runs from edge to edge. |
| Cards and lists | Ruled rows, not cards: the date in capitals in the link colour, a heavy title, the summary in grey. Only the first row on the front page shows its picture. |
| Phone header and menu | The logo, search and the menu button on one row. The sign-up panel comes first under it, then the latest issue. |
| Tabs and indexes at 390 px | One column everywhere. The latest issue's label sits above its title. |
| Link, card and focus states | Titles fade a little on hover. Links in a post are underlined. Focus is a 3 px accent ring, and a ring in the panel's text colour inside the panel. |
| Signature details | The sign-up panel in the accent colour with an email field drawn as a thick line and a round arrow button. The "Latest posts" label with a blinking dot. The latest issue fading out above a large ink button. The ruled "All posts" and archive line after each post. |

**Sample brand.** The demo uses `fixtures/showcase-brands/bulletin/theme/`: tomato red (`#b02e16` links, `#c8361b` accent) on white with a warm panel (`#fbf3ef`), Mulish for headings and Lora for text, and an envelope mark.

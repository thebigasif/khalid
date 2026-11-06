---
title: "Journal"
order: 72
---

# Journal

Journal is a template for one writer's newsletter. It is set in type alone: the latest issue opens the front page in very large letters, with no picture. Every other issue follows as text beside a side column that introduces the writer and asks for the email.

Based on Ghost's Journal theme (MIT).

```toml
[template]
name = "journal"
```

## Who it suits

- One writer who sends a personal letter every week or month and keeps every issue on the blog.
- Essays, notes and letters, where the words matter more than pictures.
- A blog that wants new subscribers, with a quiet form beside the posts and one bold band before the footer.

It suits a team blog or a blog led by data less well. For studies and long how-tos, Ledger reads better. For a newsletter with a large sign-up panel and pictures, Bulletin reads better.

## What the front page shows

1. **The header:** search on the start side, your logo or the blog's name in the middle, and your call to action as a round button on the end side. The menu sits centred under them, in small capitals between two hairlines.
2. **The latest issue:** the newest post, with no picture. A small line says "Latest posts" and the date in your link colour. Then the title in very large type, the summary in large type, and the writer and the reading time in small grey capitals.
3. **Past issues:** under a small label with a hairline, every other post on the page as text. Each shows the title, the summary, then the date in your link colour and the reading time. Numbered page links follow.
4. **The side column** (option `sidebar`): beside the past issues, the blog's site icon, its name and the `intro` of your home page, the newsletter form drawn as one field with the button inside it, up to five featured posts in small type, and every topic as a row with an arrow.
5. Each home section and topic tab, when you have them.
6. **The sign-up band:** the newsletter form, centred, on a band of your accent colour that runs from edge to edge before the footer.

From 48rem the past issues and the side column sit side by side, and the side column stays in view while the list scrolls. On a phone the side column follows the list.

A later page of the front page shows the blog's name and that page's posts, laid out like a topic page.

## What a post shows

1. One line in small capitals: "By" and the writers, the topic, the date, when it was updated and the reading time.
2. The title in very large type and the subtitle, from the post's `description`, in large type. Both start at the edge of the wide frame.
3. The lead picture across the wide frame.
4. The post in one centred column of about 65 letters a line. Links are underlined in your link colour.
5. The author box, when it is on.
6. A ruled row of three: "All posts" with the blog's name on the start side, the share links in the middle, and "Keep reading" with the first related post on the end side.
7. Comments, then more related posts as text, the call to action band when it is on, and the sign-up band.

## List pages

A topic, tag or archive page puts its name, its label and its intro in a narrow column on the start side, with its posts beside it as text. An author page does the same with the round photo, the name, the post count and the bio. On a phone the name comes first and the posts follow.

## Your brand

Journal takes your colours, fonts and logo from `theme/tokens.toml`, so it looks like your main site. The sign-up band is filled with your accent colour, with black or white text, whichever reads better. Titles use your heading font. Labels, dates, menus, the side column and posts use your body font.

Journal's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Journal's default |
|---|---|
| `[colors]` | white page, near-black text, a cobalt accent `#2c50d3` and link colour `#2442b5` |
| `[fonts] heading` | the reader's own sans: San Francisco, Segoe UI, Helvetica or Arial |
| `[fonts] body` | the same sans |
| `[type] body` | `1.0625rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.625rem` to `4.625rem` |
| `[type] heading_weight` | `600` |
| `[type] line_height` | `1.6` |
| `[space] section` | `5rem` |
| `[shape] radius` | `0.375rem`; buttons are round |
| `[shape] measure` | `38rem` |

Nothing is downloaded for the fonts: each reader sees the sans their own device has. A serif heading font from theme sync gives the titles a literary feel, as Ghost's "elegant serif" setting does.

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `sidebar` | `true` | The front page shows the side column with the blog's intro, the newsletter form, featured posts and topics. With `false`, the past issues stand alone. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `dek` | `true` | A post shows its `description` as a subtitle under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the cards on list pages: `text`, `image`, `row`, `compact` or `ranked`. Journal shows no pictures in its lists. |
| `card_reading_time` | `true` | Each card shows how long the post takes to read. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `contents` | `"none"` | Where a long post's contents go: `none`, `inline`, `inline-folded`, `side-end` or `side-start`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` for 1, 1.1, or `none`. |
| `related` | `"list"` | How related posts are laid out: `list`, `grid`, `row`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show after the first, which the "Keep reading" link names, from `3` to `10`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. At the end they sit in the middle of the ruled row. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | Which share links show, in this order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. On the front page it shows in the side column and in the sign-up band; on a post, in the band. |
| `cta_band` | `"none"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a round button. |
| `hero_stats` | none | Up to four numbers under the blog's name in the side column, each written `number \| what it counts`. |
| `topic_tabs` | `false` | Tabs under the past issues, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `false` | A row of the newest posts with a podcast or other sound player, when there are any. |

The call to action comes from `[template.cta]`; see [the templates guide](../templates.md#your-call-to-action).

For example, a site that wants the past issues alone and the share links under the title:

```toml
[template]
name = "journal"
[template.options]
sidebar = false
share = "top"
```

## Good to know

- Everything reads with scripts off. Journal adds no script.
- The newsletter form shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table. Without it, the side column links to the archive when the archive is on.
- Ghost's members, paid tiers, upgrade button, recommendations, comments and search are not part of Journal. Khalid's own newsletter form, comments and search take their places.
- Ghost counts the issues in each topic. Journal lists the topics with an arrow in place of the count.
- Ghost links the previous and next issue after a post. Journal links back to every post and on to the first related post.
- Ghost loads more issues with a button. Journal uses numbered pages, and the archive at `/archive/` lists every post.

## Design

This is Journal's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** One writer whose blog is the home of a personal newsletter. The first goal is reading the latest issue; the second is a new subscriber.

**Reference.** Ghost's Journal theme: a minimal, typography-heavy newsletter look with a stacked header, the latest issue as a very large text card, more issues as text beside a sticky side column, and a sign-up band in the accent colour.

| Part | Journal |
|---|---|
| Type pairing | One sans for everything, as Ghost's default, with titles set large at weight 600 and tight spacing. Labels, dates and menus in small capitals. A serif heading font turns only the titles serif. |
| Colour | White ground, near-black ink, light grey hairlines, dates in the link colour and one accent that fills the sign-up band and the buttons. No pictures in any list. |
| Spacing rhythm | A base step of 0.5rem. Issues 5rem apart on a wide screen, the latest issue 7.5rem above the list, the band 4.5rem tall inside. |
| Frame at 1440 px and wider | The header, the menu's hairlines, lists and the footer across **75rem** (1200 px, the frame the layout check holds). The issues in **45rem**, the side column beside them. Posts in **38rem**, their title and picture across the frame. |
| Cards and lists | Text only: the title, the summary, then the date in the link colour and the reading time in small capitals, split by small dots. No rules between issues; space alone parts them. |
| Phone header and menu | The logo on the start side, search and the menu button on the end side, on one row over a hairline. |
| Tabs and indexes at 390 px | One column everywhere. The side column follows the issues. |
| Link, card and focus states | Titles fade a little on hover. Links in a post are underlined. Focus is a 3 px accent ring, and a ring in the band's text colour inside the band. |
| Signature details | The stacked header with the menu between hairlines. The very large latest issue with no picture. Small capital labels followed by a hairline to the edge. The topic rows with a round arrow pill. The sign-up field with its button inside. The accent band before the footer. |

**Sample brand.** The demo uses `fixtures/showcase-brands/journal/theme/`: cobalt (`#2440b3` links, `#2a4bd2` accent) on warm paper (`#fffdf8`, surface `#f4f0e7`), Lora for the titles and Inter for everything else, and a bookmark mark.

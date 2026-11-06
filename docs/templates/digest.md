---
title: "Digest"
order: 36
---

# Digest

Digest is a template for a short newsletter that keeps every issue on its blog. Its front page always leads with the latest issue. Readers can open it, subscribe, or browse every past issue below it.

Based on Ghost's Digest theme (MIT).

```toml
[template]
name = "digest"
```

## Who it suits

- A writer or small team that sends a short weekly or monthly digest and posts each issue on the blog.
- A blog where the newest issue matters most and older issues form an archive.
- Posts that read like emails: one column, one picture at the top, no sidebar.

It suits a blog with many topics less well. For that, Atlas or Wire reads better. For a front page that asks readers to sign up first, Bulletin fits better.

## What the front page shows

1. **The header:** your logo or the blog's name, the menu in small capitals, then search and your call to action as a round button.
2. **The latest issue:** the newest post, set like the head of a post. A line in small capitals says "Sent on" in your link colour and the date. Then the title, large and heavy, the summary in grey and the lead picture. The picture is as wide as the text, wider, or from edge to edge (option `image_width`).
3. **The issue links:** a ruled line in small capitals under the issue, with a link to read the whole issue and, in your link colour, a link to every issue.
4. **The subscribe box:** a soft grey panel with a large heading, one line about the emails, and the email field drawn as a round field with the Subscribe button inside its end. When your blog takes no sign-ups, the panel shows your call to action, when it is on.
5. **Featured:** up to three posts marked `featured`, each in its own soft grey panel (option `picks`).
6. **Past issues:** every other post on the page as one ruled list. Each row is a heavy title, the summary in grey, then the writer, the date and the topic in small capitals.
7. Numbered page links, then each home section, when you have them.

Each section starts with a label in small capitals in your link colour, over a hairline, with a thick bar in your accent colour under the word.

A later page of the front page shows the blog's name and that page's posts in the same ruled list.

## What a post shows

1. **A grey band** behind the header and the post's head, from edge to edge.
2. One line in small capitals: the topic in your link colour, the date, "By" and the writers, when it was updated and the reading time.
3. The title, large and heavy, then the subtitle from the post's `description`.
4. The share links, as a row of round buttons.
5. The lead picture, overlapping the foot of the grey band. It is as wide as the text, wider, or from edge to edge (option `image_width`).
6. The post in one column of about 68 letters a line. Links are underlined in your link colour.
7. The author box, when it is on.
8. A ruled line of links in small capitals: a recent issue on the start side, and every issue on the end side.
9. The subscribe box and the call to action band, when they are on.
10. Comments, then three related posts as a ruled list.

When the contents go beside the post (option `contents`), the grey band is left out, so the contents column reads on the page colour.

## List pages

A topic, tag or archive page shows its name large and heavy, then its posts in the same ruled list as the front page. An author page puts the writer in a soft grey panel, with their round photo overlapping the panel's top edge, then lists their posts.

## Your brand

Digest takes your colours, fonts and logo from `theme/tokens.toml`, so it looks like your main site. Your surface colour fills the post band and the soft panels. Your accent colour fills the buttons and the bar under each section label. Labels and dates use your link colour.

Digest's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Digest's default |
|---|---|
| `[colors]` | white page, near-black text, a pale grey surface `#f4f4f5`, a cobalt accent `#3352d6` and link colour `#2a46b8` |
| `[fonts] heading` | the reader's own sans: San Francisco, Segoe UI, Helvetica or Arial |
| `[fonts] body` | the same sans, as Digest sets everything in one sans |
| `[type] body` | `1.125rem` on a phone to `1.1875rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3rem` |
| `[type] heading_weight` | `800` |
| `[type] line_height` | `1.6` |
| `[space] section` | `4rem` |
| `[shape] radius` | `0.5rem` for the panels; buttons and fields are round |

Nothing is downloaded for the fonts: each reader sees the sans their own device has.

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `image_width` | `"small"` | How wide the lead picture of the latest issue and of a post runs: `small` keeps it to the text, `wide` runs it up to 75rem, `full` runs it from edge to edge. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `dek` | `true` | A post shows its `description` as a subtitle under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"text"` | The shape of the cards on list pages: `text`, `image`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `false` | Each card shows how long the post takes to read. |
| `picks` | `3` | How many posts marked `featured` the front page shows in soft panels, from `0` for none to `8`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `contents` | `"none"` | Where a long post's contents go: `none`, `inline`, `inline-folded`, `side-end` or `side-start`. |
| `header_search` | `"button"` | What the header's search does on a served blog: `button` opens a search dialog, `link` opens the search page, `none` hides it. |
| `contents_numbers` | `"none"` | How the contents are numbered: `numbered`, `nested` for 1, 1.1, or `none`. |
| `related` | `"list"` | How related posts are laid out: `list`, `grid`, `row`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `share` | `"top"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "facebook", "linkedin", "email"]` | Which share links show, in this order. |
| `newsletter` | `"both"` | Where the subscribe box shows: `none`, `post`, `home` or `both`. On the front page it follows the latest issue. |
| `cta_band` | `"none"` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your call to action as a round button. |
| `topic_tabs` | `false` | Tabs under the past issues, one per topic, each with its four newest posts. |
| `video_section` | `false` | A row of the newest posts with a video, when there are any. |
| `podcast_section` | `false` | A row of the newest posts with a podcast or other sound player, when there are any. |

The call to action comes from `[template.cta]`; see [the templates guide](../templates.md#your-call-to-action).

For example, a site that wants the latest issue's picture from edge to edge and no featured panels:

```toml
[template]
name = "digest"
[template.options]
image_width = "full"
picks = 0
```

## Good to know

- Everything reads with scripts off. Digest adds no script.
- The front page shows the latest issue's summary, not its whole text. The issue links open the whole issue.
- The subscribe box shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table.
- Ghost's members, paid tiers, comments and search are not part of Digest. Khalid's own newsletter form, comments and search take their places.
- Ghost loads more posts as a reader scrolls, and links each issue to the one before and after it. Digest uses numbered pages, links each post to the latest issue and the archive, and the archive at `/archive/` lists every post.

## Design

This is Digest's design brief. It fixes how the template should look, so every change to its stylesheet keeps the same character. Your own colours and fonts from theme sync still replace the ones named here.

**Who it is for.** A writer or small team whose blog is the archive of a short, regular newsletter. The first goal is reading the latest issue; the second is a new subscriber.

**Reference.** Ghost's Digest theme: the front page given to the latest issue, a soft grey subscribe box, an archive of featured issues in grey panels over a ruled list, and a post that opens on a grey band.

**How it differs from Bulletin.** Bulletin splits its front page into a sign-up panel in the accent colour beside the latest issue, and centres its posts. Digest keeps one compact column on every page, leads with the issue rather than the form, and uses quiet grey panels, small capitals and the accent only in thin bars and buttons.

| Part | Digest |
|---|---|
| Type pairing | One sans for everything, headings at weight 800 and tight, labels and dates in small bold capitals. With no theme, the pack uses the system sans. |
| Colour | White ground, near-black ink, a pale grey surface for the post band and the panels, and one accent for buttons and the bar under each label. Labels and dates in the link colour or grey. |
| Spacing rhythm | A base step of 0.5rem. List rows are 3rem apart under hairlines. Panels have 2.25rem inside. Sections are 4rem apart. |
| Frame at 1440 px and wider | The header across the whole window, the footer across **75rem**. Lists and the front page in one **45rem** column (720 px, the frame the layout check holds), posts in **37.5rem**. Panels reach 2rem past the column on each side. |
| Cards and lists | Ruled rows, not cards: a heavy title, the summary in grey, then the writer, the date and the topic in small capitals. Featured rows sit in soft grey panels. |
| Phone header and menu | The logo, search and the menu button on one row. The panels run from edge to edge with square corners. |
| Tabs and indexes at 390 px | One column everywhere. The issue links stack. |
| Link, card and focus states | Titles underline on hover. Links in a post are underlined. Focus is a 3 px accent ring. |
| Signature details | The latest issue as the whole front page's lead. The section label with a thick accent bar under the word over a hairline. The round email field with the button inside it. The post's grey band with the picture overlapping its foot. The author's photo overlapping the top of their panel. |

**Sample brand.** The demo uses `fixtures/showcase-brands/digest/theme/`: cobalt (`#2340bd` links, `#2747d4` accent) on white with cool grey panels (`#f3f4f7`), Raleway for everything, and a mark of three lines like a list of issues.

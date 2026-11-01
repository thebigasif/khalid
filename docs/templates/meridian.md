# Meridian

Meridian is a calm company journal. It suits a product or services company that publishes a few considered posts a month, often written by more than one person, and keeps a handful of long guides that it updates over time.

Pick it when you want the blog to feel quiet and trustworthy: lots of white space, large titles, one column of text, and colour used only where a reader can act. If you publish many short news items a day, or want pictures to lead every list, another template will suit you better.

## How to switch it on

```toml
# site.toml
[template]
name = "meridian"
```

Build or restart the blog. Your colours, fonts, logo and corner rounding still come from `theme/tokens.toml`, so Meridian takes on your main site's look.

## What readers see

**The front page**

- Your logo in the header, with your call to action as a button beside the menu.
- A row of tabs under the blog's name: "All", then each category your posts use. On a phone the row scrolls sideways inside its own strip.
- Editor's picks: up to three posts you marked `featured: true`, as short text cards.
- Every post as a wide row: the author with their role, the date, a large title and the summary, with the post's picture as a square on the far side. Rows are split by thin lines.
- A newsletter form, when your blog takes sign-ups (see below), and numbered page links under long lists.

**A journal post** (every post, unless it asks for the reference layout)

- Breadcrumbs: the blog, the post's category, the post.
- The title, then every author with their round photo, name and role, then the dates.
- One quiet column of text. Figures keep their captions. No table of contents, no reading time and no author box, so nothing competes with the words.
- After the post: the call-to-action band, then the newsletter form.

**A reference guide** (a post with `template: reference` in its front matter)

Use this for long guides you keep up to date.

- Breadcrumbs and the title. No byline at the top: a guide speaks for the company.
- One line with "Last updated" and its date, and your call to action as a button.
- The contents, open, numbered 1, 2, 3 for sections and 1.1, 1.2 for their parts. A guide needs at least three sections (`##` headings) for the contents to show.
- After the guide: three related articles, then a closing band with two buttons: your call to action and "All articles".

Three blocks fit a guide well:

```markdown
:::callout{kind=summary}
- What the guide answers, in one line each.
:::

::::columns
:::column{title="Free plan"}
For one blog.
:::
:::column{title="Team plan"}
For a team with several authors.
:::
::::

:::callout{kind=note}
A disclaimer, or anything a reader should check first.
:::
```

**An author page** shows the author's round photo beside their name and role, their bio and profile links, then their posts as the same wide rows.

## Options

Set any of these under `[template.options]` in `site.toml`. A line you leave out keeps Meridian's default.

| Option | Meridian's default | What it changes |
|---|---|---|
| `logo` | `true` | The header shows your logo from `[brand] logo`. Without a logo, the blog's name shows. |
| `breadcrumbs` | `true` | Each page shows the trail of links from the front page to it. |
| `dek` | `false` | A post shows its `description` as a subtitle under its title. |
| `cards` | `row` | The shape of post cards in lists: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_author_role` | `true` | Each card shows its author's role after their name. |
| `category_tabs` | `true` | The front page shows the row of category tabs. |
| `picks` | `3` | How many posts marked `featured: true` the front page shows as editor's picks, from 0 for none to 8. |
| `pagination` | `numbered` | The links under a long list: `pages` for newer and older, or `numbered` for a link to each page. |
| `related` | `list` | How related posts are laid out when you turn them on for journal posts: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show, from 3 to 10. |
| `contents` | `none` | Where a journal post's contents go: `side-end`, `side-start`, `inline`, `inline-folded` or `none`. A reference guide always shows its contents open. |
| `contents_numbers` | `nested` | How a journal post's contents are numbered when you turn them on: `numbered`, `nested` or `none`. |
| `share` | `none` | Where a post shows share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["linkedin", "x", "email"]` | Which networks the share links go to, in order. |
| `newsletter` | `both` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `post` | Where the band with your call to action shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | Whether the header shows your call to action as a button. |

For example, to turn the tabs off and show five editor's picks:

```toml
[template.options]
category_tabs = false
picks = 5
```

## Reader features Meridian changes

Meridian turns three `[reader]` features off by default: `reading_time`, `author_box` and `related_posts`. Write any of them in `[reader]` to turn it back on. Your line always wins. A reference guide shows its three related articles either way.

## Your call to action and newsletter

The header button, the band after each post and the guide's buttons all show the call to action you write once:

```toml
[template.cta]
label = "Book a demo"
href = "https://example.com/demo/"
note = "See it on your own site."
```

Without `[template.cta]`, none of them shows. The newsletter form shows only on a blog that can take sign-ups: `mode = "server"` with a `[mail]` table.

## Sizes and spacing

Meridian sets these in place of the shared defaults. A value in your own `theme/tokens.toml` wins.

| Token | Meridian's value |
|---|---|
| `[type] body` | 1.0625rem on a phone to 1.1875rem on a wide screen |
| `[type] h1` | 2.5rem to 4rem |
| `[type] h2` | 1.75rem to 2.25rem |
| `[type] h3` | 1.3125rem |
| `[type] line_height` | 1.7 |
| `[space] section` | 4.5rem between sections |
| `[shape] radius` | 0.5rem corners |
| `[shape] measure` | 64ch, the width of a line of text |

The page itself is up to 72rem wide, so rows have room; the text of a post keeps to the 64ch measure.

## Colours

Text is `--color-text` on `--color-background`. The accent appears only on links, buttons, the current tab and the edge of a summary box. The call-to-action band is your accent mixed at 7% into the background. Buttons pick black or white text, whichever reads better on your accent.

## What Meridian does not do yet

- A post's category does not show on its card or above its title yet. The breadcrumbs and the tabs carry it.
- A row's square picture is the post's lead picture (`image:` in a folder post). It shows on category, tag and archive pages and on author pages. The front page shows rows without pictures for now, because `khalid serve` sends the front page before pictures have their sizes.
- The front page does not group posts under category headings yet. It will once the shared home sections are filled from `site.toml`.
- Each post has one call to action. Several different banners per site are a later step.

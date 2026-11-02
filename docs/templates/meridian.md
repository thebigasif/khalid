# Meridian

Meridian is a calm company journal. It suits a product or services company that publishes a few considered posts a month, often written by more than one person, and keeps a handful of long guides that it updates over time.

Pick it when you want the blog to feel quiet and trustworthy: lots of room, large serif titles over a plain sans text, one column to read, hairlines instead of boxes, and one accent colour used only where a reader can act. If you publish many short news items a day, another template will suit you better.

## How to switch it on

```toml
# site.toml
[template]
name = "meridian"
```

Build or restart the blog. Meridian brings its own palette (ink on a warm paper colour, one deep indigo accent) and serif titles. Any value in your own `theme/tokens.toml` wins, so a blog synced with its main site keeps that site's colours, fonts, logo and corners.

## What readers see

**The front page**

- Your logo in the header, the menu at its end and your call to action as an outlined button. On a phone the menu is one row that scrolls, and the button gives way to the band further down.
- The blog's name as a large serif title, with the intro from your front page's `_index.md` beside it when you write one.
- A row of tabs under the name: "All", then each category your posts use. On a phone the tabs become small rounded buttons that wrap.
- The newest post as the lead story: a large picture beside a large title, its summary and its author.
- Editor's picks: up to three posts you marked `featured: true`, as short text cards on a soft band across the page.
- Every other post as a wide row: the category in small capitals, the date, the title, the summary and the author with their role, with the post's picture cropped to 3 by 2 on the far side. On a phone the picture sits above the text.
- A newsletter form, when your blog takes sign-ups (see below), and numbered page links under long lists.

**A journal post** (every post, unless it asks for the reference layout)

- Breadcrumbs as one small label: the blog and the post's category.
- A large serif title, then the authors with their round photo, name and role on one side and the dates on the other, between two hairlines.
- The cover a little wider than the text, then one quiet column of text. Callouts are soft panels or a thin rule, quotes are centred, steps are numbered in circles and questions open and close with a plus.
- After the post: the author box, the call-to-action band, the newsletter form, the posts that link here, then three related posts with their pictures across the page.

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
| `related` | `scroll` | How related posts are laid out under journal posts: `grid`, `list`, `row`, `scroll` or `links`. Meridian shows `scroll` as three cards side by side on a wide screen. |
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

Meridian turns `reading_time` off by default, so the byline stays short. Write it in `[reader]` to turn it back on. Your line always wins. The author box and related posts stay on. A reference guide shows its three related articles either way.

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
| `[fonts] heading` | Iowan Old Style, Charter, Sitka Text, Cambria or Georgia: a serif every computer already has, so nothing is downloaded |
| `[type] body` | 1.0625rem on a phone to 1.1875rem on a wide screen |
| `[type] h1` | 2.5rem to 4rem |
| `[type] h2` | 1.625rem to 2rem |
| `[type] h3` | 1.25rem to 1.375rem |
| `[type] small` | 0.875rem |
| `[type] line_height` | 1.7 |
| `[type] heading_weight` | 600 |
| `[space] section` | 5rem between sections |
| `[shape] radius` | 0.375rem corners |
| `[shape] measure` | 42rem, about 70 letters a line |

The page itself is up to 72rem wide, so rows have room; the text of a post keeps to the 42rem measure and its cover runs a little wider.

## Colours

| Token | Meridian's value |
|---|---|
| `background` | `#fcfbf8`, a warm paper colour |
| `surface` | `#f3f0e9`, for bands and panels |
| `text` | `#17191e`, ink |
| `muted` | `#585c64` |
| `link` and `accent` | `#2c3f9c`, a deep indigo |
| `border` | `#e2ddd2` |

The accent appears only on links, small labels, buttons, the current tab and the short rule before each label. Buttons pick black or white text, whichever reads better on the accent. Every pair passes WCAG 2.2 AA.

## What Meridian does not do yet

- The front page does not group posts under category headings yet. It will once the shared home sections are filled from `site.toml`.
- Each post has one call to action. Several different banners per site are a later step.
- The footer shows the footer menus only; it cannot show the blog's name or logo without a change to the shared page frame.

# Workbench

Workbench is the template for blogs that review and compare tools or products. It is built for the post that weighs five options against each other: a table that shows them at a glance, a section per option with its pros and cons and its price, and a plain list of further reading at the end.

## Who it suits

- A software company that publishes "best tools for" lists and head-to-head comparisons.
- A team that writes buying guides, where readers scan a table first and read the detail second.
- A blog that grows its newsletter, because Workbench offers the sign-up form twice on the front page and once after each post.

It suits writing with clear sections and tables. For long essays with few headings, `classic` reads more calmly.

## Pick it

```toml
[template]
name = "workbench"
```

Your colours, fonts and logo still come from theme sync. Workbench draws no colour of its own: labels, borders and panels are your tokens, and light tints of your accent colour.

## What the front page shows

1. Your blog's name, and a "Categories" button that opens a menu of your categories. The menu works without scripts.
2. The newest post marked `featured: true`, large.
3. "Latest posts": the next three posts as rows, each with its category, title, date and reading time.
4. The newsletter form.
5. "More posts": the rest as picture cards, up to three in a row.
6. The newsletter form again, then the page links and "See all posts", which opens the archive.

## What a post shows

1. The category as a small label, the title, and the post's `description` as a subtitle.
2. The writers, the date and the reading time.
3. The contents in an open box at the top.
4. The body. Wide tables can grow past the text, up to 60rem on large screens, so a comparison table has room. The header row sits on your surface colour.
5. A closing line in italics: who wrote it, when it was published and when it was updated.
6. Share links, the author box, the newsletter form, and six related posts as plain links.

## Writing a comparison post

Workbench styles the blocks a comparison needs. Write them in your post as usual:

- A criteria list: a list where each item starts with **bold words**.
- The table: a normal markdown table. Wrap a long one in `:::table{fold=8}` so it shows eight rows and a line to open the rest.
- Each option's section: a `##` heading, then `:::proscons` for its pros and cons, then `:::review` for its rating, or a line with its price in **bold**.
- The questions readers ask: `::::faq`.
- Your call to action: `::cta` on its own line, once you have `[template.cta]`.

## Options

Set these under `[template.options]`. An option you leave out takes the default.

| Option | Default | What it does |
|---|---|---|
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the front page to it. |
| `dek` | `true` | A post shows its `description` as a subtitle under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `image` | The shape of post cards in lists: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `true` | Each card shows how long the post takes to read. |
| `card_author_role` | `false` | Each card shows its author's role. |
| `featured` | `true` | The newest post with `featured: true` shows large at the top. |
| `pagination` | `pages` | The links under a long list: `pages` for newer and older, or `numbered`. |
| `related` | `links` | How related posts are laid out: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `6` | How many related posts show, from 3 to 10. |
| `contents` | `inline` | Where the contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `contents_numbers` | `numbered` | How the contents are numbered: `numbered`, `nested` or `none`. |
| `share` | `end` | Where share links show: `none`, `top`, `end` or `both`. |
| `share_networks` | `["linkedin", "x", "email"]` | Which networks the share links go to, in order. |
| `newsletter` | `both` | Where the newsletter form shows: `none`, `post`, `home` or `both`. On the front page, `home` and `both` show it twice. |
| `cta_band` | `none` | Where the band with your `[template.cta]` shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | The header shows your `[template.cta]` as a button. |
| `lead_image` | `false` | A post shows its lead picture under its title. Comparison posts usually start with the table, so it is off. |
| `closing_line` | `true` | A post ends with one line naming its writers and its dates. Turned off, the updated date shows beside the published date instead. |
| `newsletter_note` | none | One line under the front page's newsletter forms, such as how often you send. |

For example:

```toml
[template.options]
newsletter_note = ["A new review every Tuesday."]
related_count = 4
share = "both"
```

The newsletter form shows only when your blog can take sign-ups: `mode = "server"` with a `[mail]` table.

## Sizes

Workbench sets these defaults. A value in your `theme/tokens.toml` wins.

| Token | Workbench |
|---|---|
| Body text | 1.0625rem to 1.125rem |
| `h1`, `h2`, `h3` | 2.25rem to 3rem, 1.625rem to 2rem, 1.25rem |
| Line height | 1.65 |
| Line length (`measure`) | 72ch |
| Corners (`radius`) | 0.625rem |

## Good to know

- The "Categories" menu needs your categories on the front page. Khalid does not fill them in yet, so the menu stays hidden until it does.
- The category on the rows under "Latest posts" and above a post's title works the same way: it shows once Khalid fills in a post's category.
- Workbench reads with scripts off, and adds no script.

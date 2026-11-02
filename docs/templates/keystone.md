# Keystone

Keystone is a template for a product or company blog: a team that writes about what it builds and what it learns, and wants the blog to look as calm and finished as its main site. It leads with one big story, shows the rest as a grid of framed pictures, and gives each post a quiet reading column with the author and related posts after it.

## Who it suits

- **It suits** a software company, a studio or an agency with several writers, where each post has a good lead picture.
- **It suits** product news, engineering notes, guides and case studies side by side, sorted by category.
- **It suits less well** a personal site with one writer and no pictures, or a news site that publishes many short posts a day. Ledger or Wire fit those better.

## What it shows

**The front page**

1. Your logo in the header, the menu as quiet links, and your call to action as a button when `[template.cta]` is set. On a phone the menu is one line you can swipe, and the button waits for the band further down.
2. The blog's name, with its topics on one line under it, when your blog lists its topics.
3. The newest post marked `featured: true` as the lead story: its picture large on one side on a wide screen, its title, summary and author on the other.
4. "Recently published": your other posts as a grid of framed picture cards, up to three across. Each card shows the category, the title, two or three lines of summary, then the author and the date. A post with no picture gets a drawn cover in tints of your accent colour, so the grid stays even.
5. The newsletter form on a framed panel, with one line over it such as "Join 4,000 readers", when your blog takes sign-ups.
6. One row of three cards for each home section in `[reader]`, each with a "View all" link.
7. Your call to action as a band, when `cta_band` is `home` or `both`.

Lists such as a category or the archive show the lead story and the same picture cards, with numbered page links under them.

**A post**

1. The category, the title and the summary (the post's `dek`), all on one left edge.
2. One line with the author's photo, name and role, then one date (the day it was last updated, or the day it was published if it never was) and the reading time.
3. The lead picture, wider than the text, in a fine frame.
4. Share links. On a wide screen they sit in a column beside the post and stay in view as you scroll. On a phone they sit in one row above the post.
5. The post in one reading column. Callouts, steps, tables, quotes, questions and the contents block each have their own quiet look.
6. Your own lines about the site, when you set them, then the author box.
7. The newsletter form, your call to action, the posts that link here and three related posts.

**Callouts** (`:::callout{kind=note}`, `tip`, `warning` and `read`) each get a small drawn icon; a warning also gets a dark edge, so the kinds differ by more than colour.

Everything reads with scripts off. Keystone adds no script.

## Options

Set these under `[template.options]` in `site.toml`. An option you leave out takes the default.

| Option | Default | Values | What it changes |
|---|---|---|---|
| `logo` | `true` | `true`, `false` | The header shows the logo from your main site in place of the blog's name. Without a logo, the name shows. |
| `cta_button` | `true` | `true`, `false` | The header shows your `[template.cta]` as a button. |
| `dek` | `true` | `true`, `false` | A post shows its summary under its title. |
| `featured` | `true` | `true`, `false` | The newest post marked `featured: true` shows large at the top of the front page and each list. |
| `cards` | `image` | `image`, `text`, `row`, `compact`, `ranked` | The shape of the post cards in lists. |
| `pagination` | `numbered` | `pages`, `numbered` | The links under a long list: newer and older, or a link to each page. |
| `contents` | `none` | `none`, `inline`, `inline-folded`, `side-end`, `side-start` | Where a long post's contents list goes. A post can also carry its own contents block. |
| `share` | `top` | `none`, `top`, `end`, `both` | Where a post shows its share links. `top` is the column beside the post. |
| `share_networks` | `["x", "linkedin", "facebook", "email"]` | any of `x`, `linkedin`, `facebook`, `bluesky`, `reddit`, `hacker-news`, `whatsapp`, `email` | Which networks the share links go to, in order. |
| `newsletter` | `both` | `none`, `post`, `home`, `both` | Where the newsletter form shows. It shows only when your blog takes sign-ups. |
| `cta_band` | `post` | `none`, `post`, `home`, `both` | Where a band with your `[template.cta]` shows. |
| `numbered_headings` | `false` | `true`, `false` | Whether each main heading of a post gets a chapter number: 01, 02, 03. |
| `subscriber_line` | none | a list of one line | One line over the newsletter form on the front page. |
| `about` | none | a list of up to two lines | Lines about your site after each post. |

An example:

```toml
[template]
name = "keystone"

[template.options]
subscriber_line = ["Join 4,000 people who read every post."]
about = ["Field Notes is written by the team that builds Field, in Bristol."]
share_networks = ["linkedin", "x", "email"]
cta_band = "both"
```

## Turning features off

Keystone keeps every reader feature on: the author box, related posts and the reading time. A key you write in `[reader]` wins over the template:

```toml
[reader]
author_box = false      # the author's photo and bio after each post
related_posts = false   # three related posts after each post
reading_time = false    # "8 min read" beside the date
```

## Sizes and spacing

Keystone sets a tight type scale on wide space. Your own `theme/tokens.toml` wins over each value.

| Token | Keystone |
|---|---|
| Body text | 1.0625rem on a phone to 1.125rem on a wide screen |
| Main title (`h1`) | 2.25rem to 3.25rem |
| Main headings (`h2`) | 1.5rem to 1.875rem |
| Headings (`h3`) | 1.1875rem to 1.3125rem |
| Small text | 0.875rem to 0.9375rem |
| Line height | 1.65 |
| Heading weight | 500, where your heading font has it; card titles use 600 |
| Space between sections | 5rem |
| Corners | 0.625rem |
| Line length | 42rem |

Keystone uses your system's own interface font, so it looks native on every device and loads nothing. It brings calm colours: a blue accent (`#3355e0`) and deep ink text (`#0d1526`) on a white page, with a faint grey (`#f5f7fa`) for panels. Your colours, fonts and logo from your main site, through theme sync, replace them, as in every template.

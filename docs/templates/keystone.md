# Keystone

Keystone is a template for a personal guide site: one person, or a small team, who writes long, practical guides under their own name. It opens with one big post, shows the rest as a grid of pictures, and lets each guide read like a short book with numbered chapters.

## Who it suits

- **It suits** a writer whose name is the brand: a consultant, a coach, a solo founder or an expert who teaches one subject.
- **It suits** guides of 1,500 words or more, split by main headings, each with a good lead picture.
- **It suits less well** a news site, a blog with many writers who each need a bio under their posts, or posts without pictures. Ledger or Compass fit those better when they arrive.

## What it shows

**The front page**

1. Your logo in the header, and your call to action as a button when `[template.cta]` is set.
2. The newest post marked `featured: true`, large: its picture on one side on a wide screen, with its author, title and summary.
3. A row of topics that starts with "All", when your blog lists its topics.
4. "Recently published": your other posts as a grid of picture cards, up to three across, each with its author.
5. The newsletter form, with one line over it such as "Join 4,000 readers", when your blog takes sign-ups.
6. One row of three cards for each home section in `[reader]`, each with a "View all" link.

Lists such as a category or the archive show the same picture cards, with numbered page links under them.

**A post**

1. The title, then the author's photo and name, and one date: the day it was last updated, or the day it was published if it never was.
2. The lead picture, as wide as the page.
3. Share links. On a wide screen they sit in a column beside the post and stay in view as you scroll. On a phone they sit in one row under the picture.
4. The post, each main heading marked with its chapter number: 01, 02, 03.
5. Your own lines about the site, such as who writes it, when you set them.

Keystone shows no author box, no related posts, no reading time and no contents list under a post. Each can come back: see "Turning features back on" below.

**Callouts** (`:::callout{kind=note}`, `tip` and `read`) each get a small drawn icon on a soft tint of your accent colour.

Everything reads with scripts off. Keystone adds no script.

## Options

Set these under `[template.options]` in `site.toml`. An option you leave out takes the default.

| Option | Default | Values | What it changes |
|---|---|---|---|
| `logo` | `true` | `true`, `false` | The header shows the logo from your main site in place of the blog's name. Without a logo, the name shows. |
| `cta_button` | `true` | `true`, `false` | The header shows your `[template.cta]` as a button. |
| `featured` | `true` | `true`, `false` | The newest post marked `featured: true` shows large at the top of the front page and each list. |
| `cards` | `image` | `image`, `text`, `row`, `compact`, `ranked` | The shape of the post cards in lists. |
| `pagination` | `numbered` | `pages`, `numbered` | The links under a long list: newer and older, or a link to each page. |
| `contents` | `none` | `none`, `inline`, `inline-folded`, `side-end`, `side-start` | Where a long post's contents list goes. |
| `share` | `top` | `none`, `top`, `end`, `both` | Where a post shows its share links. `top` is the column beside the post. |
| `share_networks` | `["x", "linkedin", "facebook", "email"]` | any of `x`, `linkedin`, `facebook`, `bluesky`, `reddit`, `hacker-news`, `whatsapp`, `email` | Which networks the share links go to, in order. |
| `newsletter` | `home` | `none`, `post`, `home`, `both` | Where the newsletter form shows. It shows only when your blog takes sign-ups. |
| `cta_band` | `none` | `none`, `post`, `home`, `both` | Where a band with your `[template.cta]` shows. |
| `numbered_headings` | `true` | `true`, `false` | Whether each main heading of a post gets a chapter number. |
| `subscriber_line` | none | a list of one line | One line over the newsletter form on the front page. |
| `about` | none | a list of up to two lines | Lines about your site after each post. |

An example:

```toml
[template]
name = "keystone"

[template.options]
subscriber_line = ["Join 4,000 people who read every guide."]
about = ["Field Notes is written by Asif, who has run SEO for small teams for ten years."]
share_networks = ["linkedin", "x", "email"]
```

## Turning features back on

Keystone switches off four reader features. A key you write in `[reader]` wins over the template:

```toml
[reader]
author_box = true       # the author's photo and bio after each post
related_posts = true    # three related posts after each post
reading_time = true     # "8 min read" beside the date
```

For a contents list, set `contents = "inline"` in `[template.options]`.

## Sizes and spacing

Keystone sets larger text and wide space. Your own `theme/tokens.toml` wins over each value.

| Token | Keystone |
|---|---|
| Body text | 1.125rem on a phone to 1.25rem on a wide screen |
| Main title (`h1`) | 2.5rem to 3.75rem |
| Main headings (`h2`) | 1.75rem to 2.25rem |
| Headings (`h3`) | 1.375rem |
| Line height | 1.7 |
| Heading weight | 800, where your heading font has it |
| Space between sections | 4rem |
| Corners | 0.75rem |
| Line length | 66 characters |

Your colours, fonts and logo come from your main site through theme sync, as in every template.

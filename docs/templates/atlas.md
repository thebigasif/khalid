# Atlas

A publication for a blog with many topics: a masthead, featured stories, picture cards and a topic index.

```toml
[template]
name = "atlas"
```

## Who it suits

Atlas suits a publication with many topics and long expert posts, such as an SEO or content marketing team's blog. A reader lands on a front page that works like the front of a magazine: the blog's name large on a soft band with every topic one tap away, your best story large at the top, the newest posts as picture cards, then an index of every topic. A post opens with a large title, a quiet byline and its picture, then reads in a measured column with its contents in a rail that stays in view.

Pick Ledger instead if you publish fewer, longer pieces and want the words to lead, or classic if you want the simplest look.

## What each page shows

**Front page**
- A masthead on a light tint of your accent colour, with a faint map grid: the blog's name large, your `tagline` when you set one, and a row of topic links: "All posts", then one link for each category your posts use.
- The two newest posts marked `featured: true`: the first large with its picture, the second beside it when there is room.
- Editor's picks: the next posts marked featured, four by default, as picture cards.
- The newest posts as picture cards, three across on a laptop and one on a phone, each with its topic, title, summary, author, date and reading time. Numbered page links follow when there are many.
- The topic index: one column per topic with its three newest posts and a link to all of them.
- The band with your `[template.cta]`, when you set one.

With no post marked featured, the front page opens with the masthead and the newest posts.

**A post**
- The trail of links from the front page to the post, on one line. Search engines read the same trail.
- The title large on the same light band, then the byline: each author's photo, name and role, who reviewed it, the dates, the reading time and the topic.
- The picture, framed, across the top of the post.
- The contents, numbered, in a rail on the start side that stays in view on wide screens and marks the section being read, and folded at the top on phones.
- The body in a column about 70 characters wide. Callouts and the summary show on soft panels, a pull quote shows large under an opening mark, tables sit on thin lines, and how-to steps get numbered circles.
- After the body: share links to X, LinkedIn, Facebook and email. They are plain links: no script and no counter, and nothing reaches a network until a reader follows one.
- Then the author box, the band with your call to action, the posts that link here, and three related posts as picture cards.

**Lists and author pages** open on the same light band with the trail and the title, then the featured post large beside its picture, then picture cards in the same grid as the front page.

The header shows your logo, your menu and your call to action as a button. Everything reads with scripts off. Atlas brings its own indigo colours; your theme tokens, such as the ones theme sync writes, replace any of them, and the logo comes from them too.

## Options

Set these under `[template.options]`. Each line shows the default.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | `""` | One line under the blog's name on the front page, up to 140 characters, such as what the blog is about. |
| `logo` | `true` | Whether the header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `true` | Whether each page shows the trail of links from the front page to it. |
| `featured` | `true` | Whether the two newest posts marked `featured: true` show large at the top of the front page. |
| `picks` | `4` | How many more posts marked featured show as editor's picks, from 0 for none to 8. |
| `updated_note` | `false` | Whether an updated post says why it was updated, under its dates. |
| `cards` | `"image"` | The shape of the post cards on list and author pages: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `true` | Whether each card and story shows how long the post takes to read. |
| `card_author_role` | `false` | Whether each card shows its author's role after their name. |
| `pagination` | `"numbered"` | The links under a long list: `pages` for newer and older, or `numbered` for a link to each page. |
| `contents` | `"side-start"` | Where a post's contents go: `side-start`, `side-end`, `inline`, `inline-folded` or `none`. |
| `contents_numbers` | `"numbered"` | How the contents are numbered: `numbered`, `nested` or `none`. |
| `related` | `"grid"` | How the related posts are laid out: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show, from 3 to 10. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "linkedin", "facebook", "email"]` | The networks the share links go to, in order. Also `bluesky`, `reddit`, `hacker-news` and `whatsapp`. |
| `newsletter` | `"none"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. It needs `[mail]`, so a static site shows none. |
| `cta_band` | `"both"` | Where the band with your `[template.cta]` shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | Whether the header shows your `[template.cta]` as a button. |

A full example:

```toml
[template]
name = "atlas"
[template.options]
tagline = "Search and content marketing, tested on real sites"
picks = 8
share = "both"
share_networks = ["linkedin", "email"]

[template.cta]
label = "Start a free trial"
href = "https://example.com/signup"
note = "No card needed."
```

## Sizes and spacing

Atlas sets these in place of classic's. A value in your own `theme/tokens.toml` wins. The header, the footer, the front page and the lists are wider than a post, up to 75rem, so the grid has room; a post's body stays at the measure, with its contents rail beside it.

```toml
[colors]
background = "#ffffff"
surface = "#f4f5fa"
text = "#131a2c"
muted = "#535a6e"
link = "#4338ca"
accent = "#4f46e5"
border = "#e0e2ec"

[shape]
radius = "0.75rem"
measure = "42rem"

[type]
body = "1.0625rem 1.125rem"
h1 = "2.25rem 3.25rem"
h2 = "1.5rem 2rem"
h3 = "1.1875rem 1.375rem"
small = "0.875rem"
line_height = 1.65

[space]
unit = "0.5rem"
section = "4rem"
```

## Reader features

Atlas keeps every `[reader]` feature as it is. The topic links in the masthead and the topic index follow `topic_links`, so `topic_links = false` turns both off.

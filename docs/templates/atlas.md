# Atlas

A portal for a blog with many topics: topic tabs, featured posts, editor's picks and breadcrumbs.

```toml
[template]
name = "atlas"
```

## Who it suits

Atlas suits a large blog with many topics and a steady stream of posts, such as a company's main content hub. A reader lands on a front page that works like the front of a magazine: a bar of topic tabs, your best posts large at the top, a grid of editor's picks, then everything new. A post opens with the trail back to its topic, share links, and its contents in a column that stays in view while the reader scrolls.

Pick Ledger instead if you publish fewer, longer pieces and want the words to lead, or classic if you want the simplest look.

## What each page shows

**Front page**
- A bar of topic tabs under the header: "All posts", then one tab for each category your posts use. It stays at the top of the window as the reader scrolls, and on a phone it scrolls sideways inside itself.
- The blog's name.
- The two newest posts marked `featured: true`, as large cards with their lead image, side by side when there is room.
- Editor's picks: the next posts marked featured, four by default, as picture cards in a grid.
- The feed: every other post with its topic, title, summary, author, reading time and date, then numbered page links when there are many.
- One block per topic with its two newest posts and a link to all of them.
- The band with your `[template.cta]`, when you set one.

With no post marked featured, the front page opens with the topic tabs and the feed.

**A post**
- The trail of links from the front page to the post: home, its topic, the post. Search engines read the same trail.
- The title and byline: each author's photo, name and role, who reviewed it, the date, the reading time and the topic.
- Share links to X, LinkedIn, Facebook and email under the title and again at the end. They are plain links: no script and no counter, and nothing reaches a network until a reader follows one.
- The contents, numbered, in a column on the start side that stays in view on wide screens, and folded at the top on phones.
- The body. A pull quote shows large under an opening mark, further reading shows in italics, and tables get clear edges.
- After the body: the author box, the band with your call to action, then three related posts.

**Lists and author pages** show posts as text cards with their reading time, split by thin lines, under the trail of links.

The header shows your call to action as a button. Everything reads with scripts off. Atlas brings its own indigo colours; your theme tokens, such as the ones theme sync writes, replace any of them, and the logo comes from them too.

## Options

Set these under `[template.options]`. Each line shows the default.

| Option | Default | What it changes |
|---|---|---|
| `logo` | `true` | Whether the header shows your logo from `[brand] logo` in place of the blog's name. |
| `breadcrumbs` | `true` | Whether each page shows the trail of links from the front page to it. |
| `featured` | `true` | Whether the two newest posts marked `featured: true` show large at the top of the front page. |
| `picks` | `4` | How many more posts marked featured show as editor's picks, from 0 for none to 8. |
| `updated_note` | `false` | Whether an updated post says why it was updated, under its dates. |
| `cards` | `"text"` | The shape of the post cards on list and author pages: `image`, `text`, `row`, `compact` or `ranked`. |
| `card_reading_time` | `true` | Whether each card and story shows how long the post takes to read. |
| `card_author_role` | `false` | Whether each card shows its author's role after their name. |
| `pagination` | `"numbered"` | The links under a long list: `pages` for newer and older, or `numbered` for a link to each page. |
| `contents` | `"side-start"` | Where a post's contents go: `side-start`, `side-end`, `inline`, `inline-folded` or `none`. |
| `contents_numbers` | `"numbered"` | How the contents are numbered: `numbered`, `nested` or `none`. |
| `related` | `"list"` | How the related posts are laid out: `grid`, `list`, `row`, `scroll` or `links`. |
| `related_count` | `3` | How many related posts show, from 3 to 10. |
| `share` | `"both"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["x", "linkedin", "facebook", "email"]` | The networks the share links go to, in order. Also `bluesky`, `reddit`, `hacker-news` and `whatsapp`. |
| `newsletter` | `"none"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. It needs `[mail]`, so a static site shows none. |
| `cta_band` | `"both"` | Where the band with your `[template.cta]` shows: `none`, `post`, `home` or `both`. |
| `cta_button` | `true` | Whether the header shows your `[template.cta]` as a button. |

A full example:

```toml
[template]
name = "atlas"
[template.options]
picks = 8
share = "end"
share_networks = ["linkedin", "email"]

[template.cta]
label = "Start a free trial"
href = "https://example.com/signup"
note = "No card needed."
```

## Sizes and spacing

Atlas sets these in place of classic's. A value in your own `theme/tokens.toml` wins. The header, the footer and the front page are wider than a post, up to 76rem, so the grid has room.

```toml
[colors]
background = "#ffffff"
surface = "#f3f4fb"
text = "#151a2d"
muted = "#575e75"
link = "#3337b5"
accent = "#3c40c6"
border = "#dde0ee"

[shape]
radius = "0.5rem"
measure = "72ch"

[type]
body = "1rem 1.0625rem"
h1 = "2rem 2.75rem"
h2 = "1.5rem 1.875rem"
h3 = "1.25rem"
small = "0.875rem"
line_height = 1.6

[space]
unit = "0.5rem"
section = "3rem"
```

## Reader features

Atlas keeps every `[reader]` feature as it is. The topic tabs and the topic blocks follow `topic_links`, so `topic_links = false` turns both off.

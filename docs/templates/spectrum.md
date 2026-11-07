---
title: "Spectrum"
order: 86
---

# Spectrum

Spectrum is a template for a magazine arranged by topic. Each topic gets its own colour. The front page gives every topic a row of its newest posts on a band of that colour, and the same colour follows the topic to its posts and its page.

```toml
[template]
name = "spectrum"
```

## Who it suits

- A general-interest magazine with four to six regular topics, such as science, food, travel, culture, nature and design.
- A club, school or city magazine whose readers come for one topic and stay for the next.
- Any site where what a post is about matters more than how new it is.

It suits a news site that posts many times a day less well. For that, Wire reads better. For a blog with dozens of topics, Atlas reads better.

## Each topic's colour

Topics are your categories. Spectrum gives each one a colour from its place in your main menu, `[[nav.primary]]` in `site.toml`. The first topic in the menu takes the first colour, the second the next, and so on round six colours. Put your topics in the menu in the order you want the colours.

```toml
[[nav.primary]]
label = "Science"
href = "/blog/category/science/"

[[nav.primary]]
label = "Food"
href = "/blog/category/food/"
```

A topic that is not in the menu still gets a colour, chosen from its name, but two such topics may share one.

The colours are made from your accent colour, `accent` in `theme/tokens.toml`, turned round the colour wheel by mixing it with itself. So a red accent gives a spectrum that starts in red, and a blue accent one that starts in blue. Each colour has a light tint for the bands, with your own text colour on it, and a strong shade for rules and small squares only, never for text. Older browsers that cannot mix colours this way show your surface colour for every topic.

## What the front page shows

1. Your magazine's name large, the `tagline` under it, and the **spectrum bar**: one coloured block per topic, each with its name, linking to that topic's row lower down.
2. **Lead story:** your newest post marked `featured: true`, or your newest post, with a large picture, its topic, the title, its description and the writer.
3. **Latest posts** beside it on wide screens: the newest other posts with their dates. `latest_count` sets how many.
4. **One row per topic**, up to six, each on a band of the topic's tint with a strong rule on top. The topic's newest post shows large with its description, and up to three more as rows with a small square picture. On a phone those rows are words only, so a phone loads one picture per topic. Each row ends with an "All in" link to the topic's page. The lead story is not repeated in its row, unless it is the topic's only post.
5. The newsletter form and your call to action, when you set them.

With `[[reader.home_sections]]` in `site.toml`, those sections become the rows, in your order. Later pages of the front page show the posts as cards.

## What a post shows

1. A band across the page in the colour of the post's topic, with the topic, the title, the post's description, the writer and the date.
2. The lead picture under the band, as wide as the page.
3. The post in one column of about 37rem, which holds 60 to 75 characters a line, with a folded contents list for long posts.
4. The tags, the share links and the author box.
5. Related posts as cards, each with a rule in its own topic's colour.

## Other pages

- **A topic** (a category page): the same coloured band as its posts, with the topic's name and intro, then its posts as cards.
- **Tags and the archive:** a plain header, then the posts as cards.
- **Writers** (`/authors/`): everyone in a grid with their mark or photo, role and how many posts they wrote.
- **A writer's page:** their photo or mark, role and bio, then their posts as cards and the other writers.

## Your brand

Spectrum brings its own colours: ink on white with an ink blue accent (`#1f4fbf`), a warm grey surface and square edges. Your colours, fonts and logo in `theme/tokens.toml` replace them. Pick an accent with some colour in it, since the topic colours are made from it: a grey accent gives grey topics.

Spectrum's own defaults, which any value in your `theme/tokens.toml` replaces:

| Token | Spectrum's default |
|---|---|
| `[fonts] heading` | Helvetica Neue, then the system's own sans |
| `[fonts] body` | Charter, then other text serifs and Georgia |
| `[type] body` | `1.125rem` on a phone to `1.25rem` on a wide screen |
| `[type] h1` | `2.25rem` to `3.5rem` |
| `[type] h2` | `1.5rem` to `2.125rem` |
| `[type] h3` | `1.25rem` to `1.5rem` |
| `[type] small` | `0.875rem` |
| `[type] line_height` | `1.6` |
| `[type] heading_weight` | `700` |
| `[shape] radius` | `0rem`, square |
| `[shape] measure` | `40rem` |
| `[space] section` | `4rem`, the space between the parts of a page |

## Options

Set any of these under `[template.options]`. An option you leave out takes the default shown.

| Option | Default | What it changes |
|---|---|---|
| `tagline` | none | One line under the magazine's name on the front page. |
| `latest_count` | `5` | How many of the latest posts show beside the lead story, from `0` to `10`. |
| `logo` | `true` | The header shows your logo from `[brand] logo` in place of the site's name. |
| `breadcrumbs` | `false` | Each page shows the trail of links from the home page to it. |
| `dek` | `true` | A post shows its description under its title. |
| `updated_note` | `false` | An updated post says why it was updated, from its `updated_note`. |
| `cards` | `"image"` | The shape of the cards on shared pages such as search: `image`, `text`, `row`, `compact` or `ranked`. |
| `pagination` | `"numbered"` | `numbered` links to each page near this one; `pages` shows only newer and older. |
| `related_count` | `3` | How many related posts show, from `3` to `10`. |
| `contents` | `"inline-folded"` | Where a long post's contents go: `inline`, `inline-folded`, `side-end`, `side-start` or `none`. |
| `header_search` | `"button"` | What the header's search does on a served site: `button`, `link` or `none`. |
| `contents_numbers` | `"none"` | How the contents are numbered: `none`, `numbered` or `nested`. |
| `share` | `"end"` | Where a post shows its share links: `none`, `top`, `end` or `both`. |
| `share_networks` | `["facebook", "whatsapp", "email"]` | The networks the share links go to, in order. |
| `newsletter` | `"both"` | Where the newsletter form shows: `none`, `post`, `home` or `both`. |
| `cta_band` | `"home"` | Where your call to action shows as a band: `none`, `post`, `home` or `both`. |
| `cta_button` | `false` | The header shows your call to action as a button. |
| `topic_tabs` | `false` | Tabs on the front page, one per topic, each with its four newest posts. Spectrum's rows show every topic already, so most sites leave this off. |
| `video_section` | `false` | A row of the newest posts with a video. |
| `podcast_section` | `false` | A row of the newest posts with a sound player. |
